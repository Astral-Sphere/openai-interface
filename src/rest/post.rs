use std::future::Future;

use serde::Serialize;

use crate::errors::RequestError;

pub trait Post {
    fn is_streaming(&self) -> bool;
}

pub trait NoStream: Post + Serialize + Sync + Send {
    /// Sends a POST request to the specified URL with the provided api-key.
    fn get_response(
        &self,
        url: &str,
        key: &str,
    ) -> impl Future<Output = Result<String, RequestError>> + Send + Sync {
        async move {
            if self.is_streaming() {
                return Err(RequestError::StreamingViolation);
            }

            let client = reqwest::Client::new();
            let response = client
                .post(url)
                .headers({
                    let mut headers = reqwest::header::HeaderMap::new();
                    headers.insert("Content-Type", "application/json".parse().unwrap());
                    headers.insert("Accept", "application/json".parse().unwrap());
                    headers
                })
                .bearer_auth(key)
                .json(self)
                .send()
                .await
                .map_err(|e| {
                    RequestError::SendError(format!("Failed to send request: {:#?}", e))
                })?;

            if response.status() != reqwest::StatusCode::OK {
                return Err(crate::errors::RequestError::ResponseStatus(
                    response.status().as_u16(),
                )
                .into());
            }

            let text = response.text().await.map_err(|e| {
                RequestError::ResponseError(format!("Failed to get response text: {:#?}", e))
            })?;

            Ok(text)
        }
    }
}
