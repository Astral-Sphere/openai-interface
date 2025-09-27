use serde::Serialize;

use std::collections::HashMap;

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

#[derive(Debug, Serialize, Clone)]
pub enum FilePurpose {
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "batch")]
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
/// Anchor timestamp after which the expiration policy applies.
/// Supported anchors: `created_at`.
pub enum ExpiresAfter {
    /// The number of seconds after the anchor time that the file will expire.
    /// Must be between 3600 (1 hour) and 2592000 (30 days).
    CreatedAt { seconds: usize },
}
