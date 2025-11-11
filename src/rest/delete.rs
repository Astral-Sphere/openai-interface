//! Delete request

use std::str::FromStr;

use serde::de::DeserializeOwned;

use crate::errors::OapiError;

pub trait Delete {
    /// Returns the URL with query parameters
    ///
    /// Parameters:
    /// - _base_url_: should look like <https://api.openai.com/v1>
    fn build_url(&self, base_url: &str) -> Result<String, OapiError>;
}

pub trait DeleteNoStream: Delete + Send + Sync {
    type Response: DeserializeOwned + FromStr<Err = OapiError> + Send + Sync;

    fn get_response_string(
        &self,
        base_url: &str,
        key: &str,
    ) -> impl Future<Output = Result<String, OapiError>> + Send + Sync {
        async move {
            let url = self.build_url(base_url)?;

            let client = reqwest::Client::new();
            let response = client
                .delete(url)
                .header("Accept", "application/json")
                .bearer_auth(key)
                .send()
                .await
                .map_err(|e| {
                    OapiError::SendError(format!("Failed to send DELETE request: {}", e))
                })?;

            if response.status() != reqwest::StatusCode::OK {
                return Err(
                    crate::errors::OapiError::ResponseStatus(response.status().as_u16()).into(),
                );
            }

            let text = response.text().await.map_err(|e| {
                OapiError::ResponseError(format!("Failed to get response text: {:#?}", e))
            })?;

            Ok(text)
        }
    }

    fn get_response(
        &self,
        base_url: &str,
        key: &str,
    ) -> impl Future<Output = Result<Self::Response, OapiError>> + Send + Sync {
        async move {
            let text = self.get_response_string(base_url, key).await?;
            let result = Self::Response::from_str(&text)?;
            Ok(result)
        }
    }
}
