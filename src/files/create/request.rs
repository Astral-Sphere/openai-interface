use serde::Serialize;
use std::future::Future;
use std::path::PathBuf;

use crate::errors::OapiError;
use crate::rest::post::{NoStream, Post};

/// Upload a file that can be used across various endpoints.
///
/// Individual files can be up to 512 MB, and the size of all files uploaded by one
/// organization can be up to 1 TB.
///
/// The Assistants API supports files up to 2 million tokens and of specific file
/// types. See the
/// [OpenAI Assistants Tools guide](https://platform.openai.com/docs/assistants/tools) for
/// details.
///
/// The Fine-tuning API only supports `.jsonl` files. The input also has certain
/// required formats for fine-tuning
/// [chat](https://platform.openai.com/docs/api-reference/fine-tuning/chat-input) or
/// [completions](https://platform.openai.com/docs/api-reference/fine-tuning/completions-input)
/// models.
///
/// The Batch API only supports `.jsonl` files up to 200 MB in size. The input also
/// has a specific required
/// [format](https://platform.openai.com/docs/api-reference/batch/request-input).
///
/// Please [contact OpenAI](https://help.openai.com/) if you need to increase these
/// storage limits.
#[derive(Debug, Serialize, Clone)]
pub struct CreateFileRequest {
    /// The File object (not file name) to be uploaded.
    #[serde(skip_serializing)]
    pub file: PathBuf,
    /// The intended purpose of the uploaded file. One of: - `assistants`: Used in the
    /// Assistants API - `batch`: Used in the Batch API - `fine-tune`: Used for
    /// fine-tuning - `vision`: Images used for vision fine-tuning - `user_data`:
    /// Flexible file type for any purpose - `evals`: Used for eval data sets
    pub purpose: FilePurpose,
    /// The expiration policy for a file. By default, files with `purpose=batch` expire
    /// after 30 days and all other files are persisted until they are manually deleted.
    ///
    /// This parameter is not supported by Qwen is not tested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after: Option<ExpiresAfter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_body: Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Clone, Default)]
pub enum FilePurpose {
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "batch")]
    #[default]
    Batch,
    #[serde(rename = "fine-tune")]
    FineTune,
    #[serde(rename = "vision")]
    Vision,
    #[serde(rename = "user_data")]
    UserData,
    #[serde(rename = "evals")]
    Evals,
}

// #[derive(Debug, Serialize, Clone)]
// pub enum FileTypes {
//     /// file (or bytes)
//     FileContent(Vec<u8>),
//     /// (filename, file (or bytes))
//     FileNameAndContent(String, Vec<u8>),
//     /// (filename, file (or bytes), content_type)
//     FileNameAndContentAndType(String, Vec<u8>, String),
//     /// (filename, file (or bytes), content_type, headers)
//     FileNameAndContentAndTypeAndHeaders(String, Vec<u8>, String, HashMap<String, String>),
// }

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "anchor", rename = "snake_case")]
/// The expiration policy for a file.
///
/// By default, files with `purpose=batch` expire after 30 days and all other files
/// are persisted until they are manually deleted.
pub enum ExpiresAfter {
    /// Anchor timestamp after which the expiration policy applies.
    /// Supported anchors: `created_at`.
    CreatedAt {
        /// The number of seconds after the anchor time that the file will expire.
        /// Must be between 3600 (1 hour) and 2592000 (30 days).
        seconds: usize,
    },
}

impl Post for CreateFileRequest {
    #[inline]
    fn is_streaming(&self) -> bool {
        false
    }
}

impl NoStream for CreateFileRequest {
    type Response = super::response::FileObject;

    // fn file_pathbuf(&self) -> PathBuf {
    //     self.file.clone()
    // }

    /// Sends a file upload POST request using multipart/form-data format.
    /// This implementation handles the actual file upload with proper file handling.
    fn get_response_string(
        &self,
        url: &str,
        key: &str,
    ) -> impl Future<Output = Result<String, OapiError>> + Send + Sync {
        async move {
            if self.is_streaming() {
                return Err(OapiError::NonStreamingViolation);
            }

            let client = reqwest::Client::new();

            // Check if file exists
            if !self.file.exists() {
                return Err(OapiError::FileNotFoundError(self.file.clone()));
            }

            // Read file content
            let file_content = tokio::fs::read(&self.file).await.map_err(|e| {
                OapiError::ResponseError(format!(
                    "Failed to read file {}: {}",
                    self.file.display(),
                    e
                ))
            })?;

            // Get file name from path
            let file_name = self
                .file
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| OapiError::ResponseError("Invalid file name".to_string()))?
                .to_string();

            // Create multipart form with file and purpose
            let file_part =
                reqwest::multipart::Part::bytes(file_content).file_name(file_name.clone());

            let mut form = reqwest::multipart::Form::new().part("file", file_part);

            // Add purpose field
            let purpose_str = serde_json::to_string(&self.purpose).map_err(|e| {
                OapiError::ResponseError(format!("Failed to serialize purpose: {}", e))
            })?;
            let trimmed_purpose = purpose_str.trim_matches('"').to_string();
            form = form.text("purpose", trimmed_purpose);

            // Add expires_after if present
            if let Some(expires_after) = &self.expires_after {
                let expires_str = serde_json::to_string(expires_after).map_err(|e| {
                    OapiError::ResponseError(format!("Failed to serialize expires_after: {}", e))
                })?;
                form = form.text("expires_after", expires_str);
            }

            let response = client
                .post(url)
                .headers({
                    let mut headers = reqwest::header::HeaderMap::new();
                    headers.insert("Accept", "application/json".parse().unwrap());
                    headers
                })
                .bearer_auth(key)
                .multipart(form)
                .send()
                .await
                .map_err(|e| OapiError::SendError(format!("Failed to send request: {:#?}", e)))?;

            if response.status() != reqwest::StatusCode::OK {
                return Err(OapiError::ResponseStatus(response.status().as_u16()).into());
            }

            let text = response.text().await.map_err(|e| {
                OapiError::ResponseError(format!("Failed to get response text: {:#?}", e))
            })?;

            // let result = <Self::Response as FromStr>::from_str(&text)?;
            Ok(text)
        }
    }
}
