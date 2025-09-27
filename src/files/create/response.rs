use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct FileObject {
    /// The file identifier, which can be referenced in the API endpoints.
    pub id: String,
    /// The size of the file, in bytes.
    pub bytes: usize,
    /// The Unix timestamp (in seconds) for when the file was created.
    pub created_at: usize,
    /// The name of the file.
    pub filename: String,
    /// The object type, which is always `file`.
    pub object: String,
    /// The intended purpose of the file.
    /// Supported values are `assistants`, `assistants_output`, `batch`, `batch_output`,
    /// `fine-tune`, `fine-tune-results`, `vision`, and `user_data`.
    pub purpose: super::FilePurpose,
    /// Deprecated. The current status of the file, which can be either `uploaded`,
    /// `processed`, or `error`.
    pub status: Option<FileStatus>,
    /// The Unix timestamp (in seconds) for when the file will expire.
    pub expires_at: Option<usize>,
    /// Deprecated. For details on why a fine-tuning training file failed validation, see the
    /// `error` field on `fine_tuning.job`.
    pub status_details: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum FileStatus {
    Uploaded,
    Processed,
    Error,
}
