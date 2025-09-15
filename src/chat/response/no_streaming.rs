use serde::Deserialize;

use crate::errors::ResponseError;

#[derive(Debug, Deserialize)]
pub struct Completion {
    pub id: String,
    pub choices: Vec<ResponseChoice>,
    pub created: u64,
    pub model: String,
    pub system_fingerprint: String,
    pub object: String,
    pub usage: CompletionUsage,
}

#[derive(Debug, Deserialize)]
pub struct ResponseChoice {
    pub finish_reason: FinishReason,
    pub index: usize,
    pub message: ResponseMessage,
    pub logprobs: Option<ResponseLogprobs>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    Length,
    Stop,
    ContentFilter,
    InsufficientSystemResource,
}

#[derive(Debug, Deserialize)]
pub struct ResponseMessage {
    /// This shall always be ResponseRole::Assistant
    pub role: ResponseRole,
    pub content: Option<String>,
    pub reasoning_content: Option<String>,
    /// Tool calls deserialization is not supported yet.
    pub tool_calls: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseRole {
    User,
    Assistant,
    System,
    Tool,
}

#[derive(Debug, Deserialize)]
pub struct ResponseLogprobs {
    pub content: Vec<LogProb>,
    pub reasoning_content: Vec<LogProb>,
}

#[derive(Debug, Deserialize)]
pub struct LogProb {
    pub token: String,
    pub logprob: f32,
    pub bytes: Option<Vec<u8>>,
    pub top_logprobs: Vec<TopLogprob>,
}

#[derive(Debug, Deserialize)]
pub struct TopLogprob {
    pub token: String,
    pub logprob: f32,
    pub bytes: Option<Vec<u8>>,
}

#[derive(Debug, Deserialize)]
pub struct CompletionUsage {
    pub completion_tokens: usize,
    pub prompt_tokens: usize,

    // These two fields seem to be DeepSeek specific.
    pub prompt_cache_hit_tokens: Option<usize>,
    pub prompt_cache_miss_tokens: Option<usize>,

    pub total_tokens: usize,
    pub completion_tokens_details: Option<CompletionTokensDetails>,
}

#[derive(Debug, Deserialize)]
pub struct CompletionTokensDetails {
    pub reasoning_tokens: usize,
}

impl Completion {
    pub fn parse_string(content: &str) -> Result<Self, crate::errors::ResponseError> {
        let parse_result: Result<Completion, _> = serde_json::from_str(content)
            .map_err(|e| ResponseError::DeserializationError(e.to_string()));
        parse_result
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn no_streaming_example_deepseek() {
        let json = r#"{
          "id": "30f6413a-a827-4cf3-9898-f13a8634b798",
          "object": "chat.completion",
          "created": 1757944111,
          "model": "deepseek-chat",
          "choices": [
            {
              "index": 0,
              "message": {
                "role": "assistant",
                "content": "Hello! How can I help you today? 😊"
              },
              "logprobs": null,
              "finish_reason": "stop"
            }
          ],
          "usage": {
            "prompt_tokens": 10,
            "completion_tokens": 11,
            "total_tokens": 21,
            "prompt_tokens_details": {
              "cached_tokens": 0
            },
            "prompt_cache_hit_tokens": 0,
            "prompt_cache_miss_tokens": 10
          },
          "system_fingerprint": "fp_08f168e49b_prod0820_fp8_kvcache"
        }"#;

        let parsed = super::Completion::parse_string(json);
        match parsed {
            Ok(_) => {}
            Err(e) => {
                panic!("Failed to deserialize: {}", e);
            }
        }
    }
}
