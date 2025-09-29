//! File list request functionality
//!
//! This module provides functionality for listing files via GET requests,
//! demonstrating the use of the GET request traits.

use url::Url;

use crate::rest::get::{Get, GetNoStream};

/// Request parameters for listing files
#[derive(Debug, Clone, Default)]
pub struct ListFilesRequest {
    /// A cursor for use in pagination.
    ///
    /// `after` is an object ID that defines your place
    /// in the list. For instance, if you make a list request and receive 100 objects,
    /// ending with obj_foo, your subsequent call can include after=obj_foo in order to
    /// fetch the next page of the list.
    pub after: Option<String>,
    /// A limit on the number of objects to be returned. Limit can range between 1 and
    /// 10,000, and the default is 10,000.
    pub limit: Option<u32>,
    /// Only return files with the given purpose.
    pub purpose: Option<String>,
}

impl Get for ListFilesRequest {
    /// base_url should look like https://api.openai.com/v1/ (must end with '/')
    fn build_url(&self, base_url: &str) -> String {
        let mut url = Url::parse(base_url)
            .expect("Failed to parse base URL")
            .join("files")
            .expect("Failed to join URL");

        let mut has_params = false;
        {
            let mut query_pairs = url.query_pairs_mut();

            if let Some(ref purpose) = self.purpose {
                query_pairs.append_pair("purpose", purpose);
                has_params = true;
            }

            if let Some(limit) = self.limit {
                query_pairs.append_pair("limit", &limit.to_string());
                has_params = true;
            }

            if let Some(ref after) = self.after {
                query_pairs.append_pair("after", after);
                has_params = true;
            }
        }

        // Remove trailing '?' if no parameters were added
        let url_string = url.to_string();
        if url_string.ends_with('?') && !has_params {
            url_string.trim_end_matches('?').to_string()
        } else {
            url_string
        }
    }
}

impl GetNoStream for ListFilesRequest {
    type Response = super::response::ListFilesResponse;
}
