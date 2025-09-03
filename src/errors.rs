use thiserror::Error;

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("Invalid response code: {0}")]
    ResponseCode(u16),
    #[error("Invalid response status: {0}")]
    ResponseStatus(u16),
    #[error("Failed to parse to String: {0}")]
    SseParseError(String),
    #[error("{0}")]
    StreamError(String),
}
