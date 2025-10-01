//! File upload functionality for OpenAI-compatible APIs
//!
//! This module provides functionality for uploading files to OpenAI-compatible APIs.
//! Files can be uploaded for various purposes such as fine-tuning, batch processing,
//! assistants, and more.
//!
//! # Overview
//!
//! The `create` module contains:
//! - [`request`]: File upload request types and functionality
//! - Support for multipart/form-data file uploads
//! - Comprehensive error handling for file operations
//!
//! The response object is in [`super`]
//!
//! # Examples
//!
//! ## Basic File Upload
//!
//! This example demonstrates how to upload a file for batch processing:
//!
//! ```rust
//! use std::path::PathBuf;
//! use std::sync::LazyLock;
//! use openai_interface::files::create::request::{CreateFileRequest, FilePurpose};
//! use openai_interface::files::FileObject;
//! use openai_interface::rest::post::PostNoStream;
//!
//! const MODELSCOPE_URL: &str = "https://dashscope.aliyuncs.com/compatible-mode/v1/files";
//! const MODELSCOPE_KEY: LazyLock<&str> =
//!     LazyLock::new(|| include_str!("../../../keys/modelstudio_domestic_key").trim());
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let file_path = PathBuf::from("src/files/create/file-test.txt");
//!     let create_file_request = CreateFileRequest {
//!         file: file_path,
//!         purpose: FilePurpose::Batch,
//!         ..Default::default()
//!     };
//!
//!     let response: FileObject = create_file_request
//!         .get_response(MODELSCOPE_URL, MODELSCOPE_KEY.as_ref())
//!         .await?;
//!
//!     println!("Uploaded file ID: {}", response.id);
//!     println!("File size: {} bytes", response.bytes);
//!     println!("Filename: {}", response.filename);
//!     Ok(())
//! }
//! ```
//!
//! ## File Upload with Custom Purpose
//!
//! This example shows how to upload a file with a custom purpose:
//!
//! ```rust
//! use std::path::PathBuf;
//! use std::sync::LazyLock;
//! use openai_interface::files::create::request::{CreateFileRequest, FilePurpose};
//! use openai_interface::files::FileObject;
//! use openai_interface::rest::post::PostNoStream;
//!
//! const MODELSCOPE_URL: &str = "https://dashscope.aliyuncs.com/compatible-mode/v1/files";
//! const MODELSCOPE_KEY: LazyLock<&str> =
//!     LazyLock::new(|| include_str!("../../../keys/modelstudio_domestic_key").trim());
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let file_path = PathBuf::from("src/files/create/file-test.txt");
//!     let create_file_request = CreateFileRequest {
//!         file: file_path,
//!         // This parameter is valid only for Qwen
//!         purpose: FilePurpose::Other("file-extract".to_string()),
//!         ..Default::default()
//!     };
//!
//!     let response: FileObject = create_file_request
//!         .get_response(MODELSCOPE_URL, MODELSCOPE_KEY.as_ref())
//!         .await?;
//!
//!     println!("Uploaded file with custom purpose: {}", response.id);
//!     Ok(())
//! }
//! ```

pub mod request;

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, sync::LazyLock};

    use crate::rest::post::PostNoStream;

    use super::*;

    const TEST_FILE_PATH: &str = "src/files/create/file-test.txt";
    const MODELSCOPE_URL: &str = "https://dashscope.aliyuncs.com/compatible-mode/v1/files";
    const MODELSCOPE_KEY: LazyLock<&str> =
        LazyLock::new(|| include_str!("../../../keys/modelstudio_domestic_key").trim());

    #[tokio::test]
    async fn test_file_create() -> Result<(), anyhow::Error> {
        let file_path = PathBuf::from(TEST_FILE_PATH);
        let create_file_request = request::CreateFileRequest {
            file: file_path,
            purpose: request::FilePurpose::Other("file-extract".to_string()),
            ..Default::default()
        };

        let response = create_file_request
            .get_response(MODELSCOPE_URL, MODELSCOPE_KEY.as_ref())
            .await?;

        println!("Uploaded file ID: {}", response.id);
        println!("File size: {} bytes", response.bytes);
        println!("Filename: {}", response.filename);

        Ok(())
    }
}
