use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum OapiError {
    #[error("Failed to send request: {0}")]
    SendError(String),
    #[error("Response error: {0}")]
    ResponseError(String),
    #[error("Invalid response code: {0}")]
    ResponseCode(u16),
    #[error("Invalid response status: {0}")]
    ResponseStatus(u16),
    #[error("Failed to parse to String: {0}")]
    SseParseError(String),
    #[error("{0}")]
    StreamError(String),
    /// If the request is a streaming request, but the context is not streaming.
    #[error("You cannot post a streaming request in a non-streaming context")]
    NonStreamingViolation,
    /// If the request is a non-streaming request, but the context is streaming.
    #[error("You cannot post a non-streaming request in a streaming context")]
    StreamingViolation,
    #[error("Deserialization error:\n{0}\n\nPlease report this error in the project issue.")]
    DeserializationError(String),
    #[error("File not found at: {0}")]
    FileNotFoundError(PathBuf),
    #[error("Failed to read file: {0}")]
    FileReadError(std::io::Error),

    #[error("Not implemented")]
    NotImplemented,
}
