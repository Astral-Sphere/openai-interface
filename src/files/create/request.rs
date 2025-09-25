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
    file: FileTypes,
    purpose: super::FilePurpose,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_after: Option<ExpiresAfter>,
}

#[derive(Debug, Serialize, Clone)]
pub enum FileTypes {
    FileContent(Vec<u8>),
    FileNameAndContent(String, Vec<u8>),
    FileNameAndContentAndType(String, Vec<u8>, String),
    FileNameAndContentAndTypeAndHeaders(String, Vec<u8>, String, HashMap<String, String>),
}

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "anchor", rename = "snake_case")]
/// Anchor timestamp after which the expiration policy applies.
/// Supported anchors: `created_at`.
pub enum ExpiresAfter {
    /// The number of seconds after the anchor time that the file will expire.
    /// Must be between 3600 (1 hour) and 2592000 (30 days).
    CreatedAt { seconds: usize },
}
