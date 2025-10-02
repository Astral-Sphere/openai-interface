//! This module provides functionality for listing files from OpenAI-compatible APIs.
//!
//! # Examples
//!
//! ```rust
//!
//! use std::sync::LazyLock;
//!
//! use openai_interface::files::list::request::ListFilesRequest;
//! use openai_interface::rest::get::{Get, GetNoStream};
//!
//! const QWEN_BASE_URL: &'static str = "https://dashscope.aliyuncs.com/compatible-mode/v1/";
//! const QWEN_API_KEY: LazyLock<&str> =
//!     LazyLock::new(|| include_str!("../../../keys/modelstudio_domestic_key").trim());
//!
//! #[tokio::main]
//! async fn main() -> Result<(), anyhow::Error> {
//!    let request = ListFilesRequest {
//!        purpose: Some("batch".to_string()),
//!        limit: Some(1),
//!        // after: Some("file_123".to_string()),
//!        ..Default::default()
//!    };
//!    let response = request.get_response(QWEN_BASE_URL, &QWEN_API_KEY).await?;
//!
//!    println!("Response: {:?}", response);
//!
//!    Ok(())
//! }
//! ```

pub mod request;
pub mod response;

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use super::request::*;
    use crate::rest::get::{Get, GetNoStream};

    const QWEN_BASE_URL: &'static str = "https://dashscope.aliyuncs.com/compatible-mode/v1/";
    const QWEN_API_KEY: LazyLock<&str> =
        LazyLock::new(|| include_str!("../../../keys/modelstudio_domestic_key").trim());

    #[test]
    fn test_build_url_no_params() {
        let request = ListFilesRequest::default();
        let base_url = "https://api.openai.com/v1/";
        let url = request.build_url(base_url).expect("Failed to build url");
        assert_eq!(url, "https://api.openai.com/v1/files");
    }

    #[test]
    fn test_build_url_with_purpose() {
        let request = ListFilesRequest {
            purpose: Some("fine-tune".to_string()),
            ..Default::default()
        };
        let base_url = "https://api.openai.com/v1/";
        let url = request.build_url(base_url).expect("Failed to build url");
        assert_eq!(url, "https://api.openai.com/v1/files?purpose=fine-tune");
    }

    #[test]
    fn test_build_url_with_limit() {
        let request = ListFilesRequest {
            limit: Some(100),
            ..Default::default()
        };
        let base_url = "https://api.openai.com/v1/";
        let url = request.build_url(base_url).expect("Failed to build url");
        assert_eq!(url, "https://api.openai.com/v1/files?limit=100");
    }

    #[test]
    fn test_build_url_with_after() {
        let request = ListFilesRequest {
            after: Some("file_123".to_string()),
            ..Default::default()
        };
        let base_url = "https://api.openai.com/v1/";
        let url = request.build_url(base_url).expect("Failed to build url");
        assert_eq!(url, "https://api.openai.com/v1/files?after=file_123");
    }

    #[test]
    fn test_build_url_with_all_params() {
        let request = ListFilesRequest {
            purpose: Some("assistants".to_string()),
            limit: Some(50),
            after: Some("file_abc".to_string()),
        };
        let base_url = "https://api.openai.com/v1/";
        let url = request.build_url(base_url).expect("Failed to build url");
        // Note: URL parameters are ordered alphabetically by the url crate
        assert!(url.contains("after=file_abc"));
        assert!(url.contains("limit=50"));
        assert!(url.contains("purpose=assistants"));
        assert!(url.starts_with("https://api.openai.com/v1/files?"));
    }

    #[test]
    fn test_build_url_encoding() {
        let request = ListFilesRequest {
            purpose: Some("fine tune".to_string()), // Space should be encoded
            ..Default::default()
        };
        let base_url = "https://api.openai.com/v1/";
        let url = request.build_url(base_url).expect("Failed to build url");
        assert_eq!(url, "https://api.openai.com/v1/files?purpose=fine+tune");
    }

    #[tokio::test]
    async fn test_query_qwen_no_params() -> Result<(), anyhow::Error> {
        let request = ListFilesRequest::default();
        let response = request.get_response(QWEN_BASE_URL, &QWEN_API_KEY).await?;

        println!("Response: {:?}", response);

        Ok(())
    }

    #[tokio::test]
    async fn test_query_qwen_with_params() -> Result<(), anyhow::Error> {
        let request = ListFilesRequest {
            purpose: Some("batch".to_string()),
            limit: Some(1),
            // after: Some("file_123".to_string()),
            ..Default::default()
        };
        let response = request.get_response(QWEN_BASE_URL, &QWEN_API_KEY).await?;

        println!("Response: {:?}", response);

        Ok(())
    }
}
