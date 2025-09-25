pub mod request;
pub mod response;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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
