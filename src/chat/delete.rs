//! Module for deleting a stored chat completion.
//!
//! This module is untested! If you encounter any issues,
//! please report them on the repository.

pub mod request {
    use url::Url;

    use crate::{
        chat::delete::response::ChatCompletionDeleted,
        errors::OapiError,
        rest::delete::{Delete, DeleteNoStream},
    };

    /// Delete a stored chat completion.
    ///
    /// Only Chat Completions that have been created
    /// with the `store` parameter set to `true` can be deleted.
    #[derive(Debug, Clone)]
    pub struct ChatDeleteRequest<'a> {
        pub completion_id: &'a str,
    }

    impl Delete for ChatDeleteRequest<'_> {
        fn build_url(&self, base_url: &str) -> Result<String, crate::errors::OapiError> {
            let mut url = Url::parse(base_url).map_err(|e| OapiError::UrlError(e))?;
            url.path_segments_mut()
                .map_err(|_| OapiError::UrlCannotBeBase(base_url.to_string()))?
                .push("chat")
                .push("delete")
                .push(self.completion_id);
            Ok(url.to_string())
        }
    }

    impl DeleteNoStream for ChatDeleteRequest<'_> {
        type Response = ChatCompletionDeleted;
    }
}

pub mod response {
    use std::str::FromStr;

    use serde::Deserialize;

    use crate::errors::OapiError;

    #[derive(Debug, Clone, Deserialize)]
    #[serde(tag = "type")]
    pub enum ChatCompletionDeleted {
        /// The type of object being deleted.
        #[serde(rename = "chat.completion.deleted")]
        ChatCompletionDeleted {
            /// The ID of the chat completion that was deleted.
            id: String,
            /// Whether the chat completion was deleted.
            deleted: bool,
        },
    }

    impl FromStr for ChatCompletionDeleted {
        type Err = OapiError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let parse_result: Result<Self, _> =
                serde_json::from_str(s).map_err(|e| OapiError::DeserializationError(e.to_string()));
            parse_result
        }
    }
}
