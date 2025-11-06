//! Module for retrieving chat completions from OpenAI API.
//!
//! > ![warn] This module is untested!
//! > If you encounter any issues, please report them on the GitHub repository.

pub mod request {
    use std::collections::HashMap;

    use url::Url;

    use crate::{
        errors::OapiError,
        rest::get::{Get, GetNoStream},
    };

    /// Get a stored chat completion.
    ///
    /// Only Chat Completions that have been created with
    /// the `store` parameter set to `true` will be returned.
    pub struct ChatRetrieveRequest {
        pub completion_id: String,
        pub extra_query: HashMap<String, String>,
    }

    impl Get for ChatRetrieveRequest {
        /// base_url should look like <https://api.openai.com/v1/> (must ends with '/')
        fn build_url(&self, base_url: &str) -> Result<String, crate::errors::OapiError> {
            let url = Url::parse(base_url)
                .map_err(|e| OapiError::UrlError(e))?
                .join("chat/")
                .unwrap()
                .join(&self.completion_id)
                .map_err(|e| OapiError::UrlError(e))?;

            Ok(url.to_string())
        }
    }

    impl GetNoStream for ChatRetrieveRequest {
        type Response = crate::chat::ChatCompletion;
    }
}
