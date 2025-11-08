//! Module for updating stored chat completions.
//!
//! > ![warn] This module is untested!
//! > If you encounter any issues, please report them on the GitHub repository.

pub mod request {
    use std::collections::HashMap;

    use serde::Serialize;

    use crate::rest::post::{Post, PostNoStream};

    /// Modify a stored chat completion.
    ///
    /// Only Chat Completions that have been created
    /// with the `store` parameter set to `true` can be modified. Currently, the only
    /// supported modification is to update the `metadata` field.
    #[derive(Debug, Serialize, Default, Clone)]
    pub struct ChatUpdate<'a> {
        /// The ID of the completion to update.
        #[serde(skip_serializing)]
        pub completion_id: &'a str,
        /// Add additional JSON properties to the request
        #[serde(skip_serializing_if = "Option::is_none")]
        pub extra_body: Option<serde_json::Map<String, serde_json::Value>>,
        /// Add additional query parameters to the request
        #[serde(skip_serializing)]
        pub extra_query: HashMap<&'a str, &'a str>,
        /// Set of 16 key-value pairs that can be attached to an object. This can be useful
        /// for storing additional information about the object in a structured format, and
        /// querying for objects via API or the dashboard.
        ///
        /// Keys are strings with a maximum length of 64 characters. Values are strings with
        /// a maximum length of 512 characters.
        pub metadata: Option<HashMap<&'a str, &'a str>>,
    }

    impl ChatUpdate<'_> {
        pub fn build_url(base_url: impl AsRef<str>, completion_id: impl AsRef<str>) -> String {
            let mut url = url::Url::parse(base_url.as_ref()).expect("Invalid base URL");
            url.path_segments_mut()
                .expect("Cannot modify URL path")
                .push("chat")
                .push("completions")
                .push(completion_id.as_ref());
            url.to_string()
        }
    }

    impl Post for ChatUpdate<'_> {
        fn is_streaming(&self) -> bool {
            false
        }
    }

    impl PostNoStream for ChatUpdate<'_> {
        type Response = crate::chat::ChatCompletion;
    }
}
