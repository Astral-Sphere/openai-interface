//! GET request functionality for OpenAI interface
//!
//! This module provides HTTP GET request functionality for making requests to OpenAI-compatible APIs.
//! Unlike POST requests, GET requests typically don't require serialization of request bodies,
//! but may include query parameters.

use std::{future::Future, str::FromStr};

use serde::de::DeserializeOwned;

use crate::errors::OapiError;

/// Base trait for GET requests
pub trait Get {
    /// Returns the URL with query parameters if needed
    ///
    /// # Example
    ///
    /// ```rust
    /// use openai_interface::rest::get::Get;
    /// use openai_interface::errors::OapiError;
    ///
    /// struct MyRequest {
    ///     id: String,
    ///     limit: Option<u32>,
    /// }
    ///
    /// impl Get for MyRequest {
    ///     fn build_url(&self, base_url: &str) -> Result<String, OapiError> {
    ///         let mut url = format!("{}/{}", base_url, self.id);
    ///         if let Some(limit) = self.limit {
    ///             url.push_str(&format!("?limit={}", limit));
    ///         }
    ///         Ok(url)
    ///     }
    /// }
    /// ```
    fn build_url(&self, base_url: &str) -> Result<String, OapiError>;
}

/// Trait for non-streaming GET requests
pub trait GetNoStream: Get + Sync + Send {
    type Response: DeserializeOwned + FromStr<Err = OapiError> + Send + Sync;

    /// Sends a GET request to the specified URL with the provided api-key.
    fn get_response_string(
        &self,
        base_url: &str,
        key: &str,
    ) -> impl Future<Output = Result<String, OapiError>> + Send + Sync {
        async move {
            let url = self.build_url(base_url)?;
            let client = reqwest::Client::new();
            let response = client
                .get(&url)
                .headers({
                    let mut headers = reqwest::header::HeaderMap::new();
                    headers.insert("Accept", "application/json".parse().unwrap());
                    headers
                })
                .bearer_auth(key)
                .send()
                .await
                .map_err(|e| {
                    OapiError::SendError(format!("Failed to send GET request: {:#?}", e))
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

    /// Sends a GET request and deserializes the response
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
