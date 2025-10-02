//! This module provides functionality for retrieving files from the OpenAI API.
//!
//! It includes the `RetrieveRequest` struct which implements the `Get` and `GetNoStream` traits
//! to build URLs and fetch file data asynchronously.
//!
//! # Example
//!
//! ```rust
//! use std::sync::LazyLock;
//!
//! use openai_interface::files::retrieve::*;
//! use openai_interface::{
//!     files::{list::request::ListFilesRequest, retrieve::request::RetrieveRequest},
//!     rest::get::{Get, GetNoStream},
//! };
//! use anyhow::bail;
//! use futures_util::future::{self};
//!
//! const MODELSCOPE_BASE_URL: &str = "https://dashscope.aliyuncs.com/compatible-mode/v1/";
//! const MODELSCOPE_KEY: LazyLock<&str> =
//!     LazyLock::new(|| include_str!("../../keys/modelstudio_domestic_key").trim());
//!
//! #[tokio::main]
//! async fn main() -> Result<(), anyhow::Error> {
//!     // first get all files
//!     let list_request = ListFilesRequest {
//!         limit: Some(5), // avoid rate limit
//!         ..Default::default()
//!     };
//!
//!     let list_response = list_request
//!         .get_response(MODELSCOPE_BASE_URL, &MODELSCOPE_KEY)
//!         .await?;
//!
//!     let futures: Vec<_> = list_response
//!         .data
//!         .iter()
//!         .map(|file_object| {
//!             let file_id = file_object.id.clone();
//!             let base_url = MODELSCOPE_BASE_URL.to_string();
//!             let key = MODELSCOPE_KEY.to_string();
//!             async move {
//!                 let retrieve_request = RetrieveRequest { file_id: &file_id };
//!                 retrieve_request.get_response(&base_url, &key).await
//!             }
//!         })
//!         .collect();
//!
//!     let results = future::join_all(futures).await;
//!
//!     for (i, result) in results.iter().enumerate() {
//!         match result {
//!             Ok(file_object) => {
//!                 assert_eq!(&list_response.data[i].id, &file_object.id);
//!                 assert_eq!(&list_response.data[i].filename, &file_object.filename);
//!                 assert_eq!(&list_response.data[i].purpose, &file_object.purpose);
//!                 // assert_eq!(&list_response.data[i], file_object);
//!             }
//!             Err(e) => {
//!                 bail!(
//!                     "Failed to get response: {e:#}. The file is: index {i}, {:?}",
//!                     list_response.data[i]
//!                 )
//!             }
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```

pub mod request {
    use url::Url;

    use crate::{
        errors::OapiError,
        rest::get::{Get, GetNoStream},
    };

    pub struct RetrieveRequest<'a> {
        pub file_id: &'a str,
    }

    impl<'a> Get for RetrieveRequest<'a> {
        /// base_url should look like <https://api.openai.com/v1/> (must ends with '/')
        fn build_url(&self, base_url: &str) -> Result<String, OapiError> {
            let url = Url::parse(base_url)
                .map_err(|e| OapiError::UrlError(e))?
                .join("files/")
                .unwrap()
                .join(self.file_id)
                .map_err(|e| OapiError::UrlError(e))?;

            Ok(url.to_string())
        }
    }

    impl<'a> GetNoStream for RetrieveRequest<'a> {
        type Response = crate::files::FileObject;
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use super::*;
    use crate::{
        files::{list::request::ListFilesRequest, retrieve::request::RetrieveRequest},
        rest::get::{Get, GetNoStream},
    };
    use anyhow::bail;
    use futures_util::future::{self};

    const MODELSCOPE_BASE_URL: &str = "https://dashscope.aliyuncs.com/compatible-mode/v1/";
    const MODELSCOPE_KEY: LazyLock<&str> =
        LazyLock::new(|| include_str!("../../keys/modelstudio_domestic_key").trim());

    #[test]
    fn test_build_url() {
        let request = request::RetrieveRequest { file_id: "file_id" };
        let url = request.build_url("https://api.openai.com/v1/").unwrap();
        assert_eq!(url, "https://api.openai.com/v1/files/file_id");
    }

    #[tokio::test]
    async fn test_retrieve_file() -> Result<(), anyhow::Error> {
        // first get all files
        let list_request = ListFilesRequest {
            limit: Some(5), // avoid rate limit
            ..Default::default()
        };

        let list_response = list_request
            .get_response(MODELSCOPE_BASE_URL, &MODELSCOPE_KEY)
            .await?;

        let futures: Vec<_> = list_response
            .data
            .iter()
            .map(|file_object| {
                let file_id = file_object.id.clone();
                let base_url = MODELSCOPE_BASE_URL.to_string();
                let key = MODELSCOPE_KEY.to_string();
                async move {
                    let retrieve_request = RetrieveRequest { file_id: &file_id };
                    retrieve_request.get_response(&base_url, &key).await
                }
            })
            .collect();

        let results = future::join_all(futures).await;

        for (i, result) in results.iter().enumerate() {
            match result {
                Ok(file_object) => {
                    assert_eq!(&list_response.data[i].id, &file_object.id);
                    assert_eq!(&list_response.data[i].filename, &file_object.filename);
                    assert_eq!(&list_response.data[i].purpose, &file_object.purpose);
                    // assert_eq!(&list_response.data[i], file_object);
                }
                Err(e) => {
                    bail!(
                        "Failed to get response: {e:#}. The file is: index {i}, {:?}",
                        list_response.data[i]
                    )
                }
            }
        }

        Ok(())
    }
}
