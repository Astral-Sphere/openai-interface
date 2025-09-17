pub mod streaming {
    use serde::Deserialize;

    use crate::errors::ResponseError;

    #[derive(Debug, Deserialize)]
    pub struct Completion {
        pub id: String,
        pub choices: Vec<CompletionChoice>,
        pub created: u64,
        pub model: String,
        pub object: String,
        pub usage: Option<CompletionUsage>,
    }

    #[derive(Debug, Deserialize)]
    pub struct CompletionChoice {
        pub delta: CompletionDelta,
        pub index: u32,
        pub logprobs: Option<ChoiceLogprobs>,
        pub finish_reason: Option<FinishReason>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum FinishReason {
        Length,
        Stop,
        ContentFilter,
        ToolCalls,
        InsufficientSystemResource,
    }

    #[derive(Debug, Deserialize)]
    pub struct CompletionDelta {
        #[serde(flatten)]
        pub content: CompletionContent,
        pub role: Option<CompletionRole>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum CompletionRole {
        User,
        Assistant,
        System,
        Tool,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum CompletionContent {
        Content(String),
        ReasoningContent(String),
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum ChoiceLogprobs {
        Content(Vec<LogprobeContent>),
        ReasoningContent(Vec<LogprobeContent>),
    }

    #[derive(Debug, Deserialize)]
    pub struct LogprobeContent {
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
        pub total_tokens: usize,
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
        use super::*;

        #[test]
        fn streaming_example_deepseek() {
            let streams = vec![
                r#"{"id": "1f633d8bfc032625086f14113c411638", "choices": [{"index": 0, "delta": {"content": "", "role": "assistant"}, "finish_reason": null, "logprobs": null}], "created": 1718345013, "model": "deepseek-chat", "system_fingerprint": "fp_a49d71b8a1", "object": "chat.completion.chunk", "usage": null}"#,
                r#"{"choices": [{"delta": {"content": "Hello", "role": "assistant"}, "finish_reason": null, "index": 0, "logprobs": null}], "created": 1718345013, "id": "1f633d8bfc032625086f14113c411638", "model": "deepseek-chat", "object": "chat.completion.chunk", "system_fingerprint": "fp_a49d71b8a1"}"#,
                r#"{"choices": [{"delta": {"content": "!", "role": "assistant"}, "finish_reason": null, "index": 0, "logprobs": null}], "created": 1718345013, "id": "1f633d8bfc032625086f14113c411638", "model": "deepseek-chat", "object": "chat.completion.chunk", "system_fingerprint": "fp_a49d71b8a1"}"#,
                r#"{"choices": [{"delta": {"content": " How", "role": "assistant"}, "finish_reason": null, "index": 0, "logprobs": null}], "created": 1718345013, "id": "1f633d8bfc032625086f14113c411638", "model": "deepseek-chat", "object": "chat.completion.chunk", "system_fingerprint": "fp_a49d71b8a1"}"#,
                r#"{"choices": [{"delta": {"content": " can", "role": "assistant"}, "finish_reason": null, "index": 0, "logprobs": null}], "created": 1718345013, "id": "1f633d8bfc032625086f14113c411638", "model": "deepseek-chat", "object": "chat.completion.chunk", "system_fingerprint": "fp_a49d71b8a1"}"#,
                r#"{"choices": [{"delta": {"content": " I", "role": "assistant"}, "finish_reason": null, "index": 0, "logprobs": null}], "created": 1718345013, "id": "1f633d8bfc032625086f14113c411638", "model": "deepseek-chat", "object": "chat.completion.chunk", "system_fingerprint": "fp_a49d71b8a1"}"#,
                r#"{"choices": [{"delta": {"content": " assist", "role": "assistant"}, "finish_reason": null, "index": 0, "logprobs": null}], "created": 1718345013, "id": "1f633d8bfc032625086f14113c411638", "model": "deepseek-chat", "object": "chat.completion.chunk", "system_fingerprint": "fp_a49d71b8a1"}"#,
                r#"{"choices": [{"delta": {"content": " you", "role": "assistant"}, "finish_reason": null, "index": 0, "logprobs": null}], "created": 1718345013, "id": "1f633d8bfc032625086f14113c411638", "model": "deepseek-chat", "object": "chat.completion.chunk", "system_fingerprint": "fp_a49d71b8a1"}"#,
                r#"{"choices": [{"delta": {"content": " today", "role": "assistant"}, "finish_reason": null, "index": 0, "logprobs": null}], "created": 1718345013, "id": "1f633d8bfc032625086f14113c411638", "model": "deepseek-chat", "object": "chat.completion.chunk", "system_fingerprint": "fp_a49d71b8a1"}"#,
                r#"{"choices": [{"delta": {"content": "?", "role": "assistant"}, "finish_reason": null, "index": 0, "logprobs": null}], "created": 1718345013, "id": "1f633d8bfc032625086f14113c411638", "model": "deepseek-chat", "object": "chat.completion.chunk", "system_fingerprint": "fp_a49d71b8a1"}"#,
                r#"{"choices": [{"delta": {"content": "", "role": null}, "finish_reason": "stop", "index": 0, "logprobs": null}], "created": 1718345013, "id": "1f633d8bfc032625086f14113c411638", "model": "deepseek-chat", "object": "chat.completion.chunk", "system_fingerprint": "fp_a49d71b8a1", "usage": {"completion_tokens": 9, "prompt_tokens": 17, "total_tokens": 26}}"#,
            ];

            for stream in streams {
                let parsed = Completion::parse_string(stream);
                match parsed {
                    Ok(completion) => {
                        println!("Deserialized: {:#?}", completion);
                    }
                    Err(e) => {
                        panic!("Failed to deserialize {}: {}", stream, e);
                    }
                }
            }
        }

        #[test]
        fn streaming_example_qwen() {
            let streams = vec![
                r#"{"id":"chatcmpl-e30f5ae7-3063-93c4-90fe-beb5f900bd57","choices":[{"delta":{"content":"","function_call":null,"refusal":null,"role":"assistant","tool_calls":null},"finish_reason":null,"index":0,"logprobs":null}],"created":1735113344,"model":"qwen-plus","object":"chat.completion.chunk","service_tier":null,"system_fingerprint":null,"usage":null}"#,
                r#"{"id":"chatcmpl-e30f5ae7-3063-93c4-90fe-beb5f900bd57","choices":[{"delta":{"content":"我是","function_call":null,"refusal":null,"role":null,"tool_calls":null},"finish_reason":null,"index":0,"logprobs":null}],"created":1735113344,"model":"qwen-plus","object":"chat.completion.chunk","service_tier":null,"system_fingerprint":null,"usage":null}"#,
                r#"{"id":"chatcmpl-e30f5ae7-3063-93c4-90fe-beb5f900bd57","choices":[{"delta":{"content":"来自","function_call":null,"refusal":null,"role":null,"tool_calls":null},"finish_reason":null,"index":0,"logprobs":null}],"created":1735113344,"model":"qwen-plus","object":"chat.completion.chunk","service_tier":null,"system_fingerprint":null,"usage":null}"#,
                r#"{"id":"chatcmpl-e30f5ae7-3063-93c4-90fe-beb5f900bd57","choices":[{"delta":{"content":"阿里","function_call":null,"refusal":null,"role":null,"tool_calls":null},"finish_reason":null,"index":0,"logprobs":null}],"created":1735113344,"model":"qwen-plus","object":"chat.completion.chunk","service_tier":null,"system_fingerprint":null,"usage":null}"#,
                r#"{"id":"chatcmpl-e30f5ae7-3063-93c4-90fe-beb5f900bd57","choices":[{"delta":{"content":"云的超大规模","function_call":null,"refusal":null,"role":null,"tool_calls":null},"finish_reason":null,"index":0,"logprobs":null}],"created":1735113344,"model":"qwen-plus","object":"chat.completion.chunk","service_tier":null,"system_fingerprint":null,"usage":null}"#,
                r#"{"id":"chatcmpl-e30f5ae7-3063-93c4-90fe-beb5f900bd57","choices":[{"delta":{"content":"语言模型，我","function_call":null,"refusal":null,"role":null,"tool_calls":null},"finish_reason":null,"index":0,"logprobs":null}],"created":1735113344,"model":"qwen-plus","object":"chat.completion.chunk","service_tier":null,"system_fingerprint":null,"usage":null}"#,
                r#"{"id":"chatcmpl-e30f5ae7-3063-93c4-90fe-beb5f900bd57","choices":[{"delta":{"content":"叫通义千","function_call":null,"refusal":null,"role":null,"tool_calls":null},"finish_reason":null,"index":0,"logprobs":null}],"created":1735113344,"model":"qwen-plus","object":"chat.completion.chunk","service_tier":null,"system_fingerprint":null,"usage":null}"#,
                r#"{"id":"chatcmpl-e30f5ae7-3063-93c4-90fe-beb5f900bd57","choices":[{"delta":{"content":"问。","function_call":null,"refusal":null,"role":null,"tool_calls":null},"finish_reason":null,"index":0,"logprobs":null}],"created":1735113344,"model":"qwen-plus","object":"chat.completion.chunk","service_tier":null,"system_fingerprint":null,"usage":null}"#,
                r#"{"id":"chatcmpl-e30f5ae7-3063-93c4-90fe-beb5f900bd57","choices":[{"delta":{"content":"","function_call":null,"refusal":null,"role":null,"tool_calls":null},"finish_reason":"stop","index":0,"logprobs":null}],"created":1735113344,"model":"qwen-plus","object":"chat.completion.chunk","service_tier":null,"system_fingerprint":null,"usage":null}"#,
                r#"{"id":"chatcmpl-e30f5ae7-3063-93c4-90fe-beb5f900bd57","choices":[],"created":1735113344,"model":"qwen-plus","object":"chat.completion.chunk","service_tier":null,"system_fingerprint":null,"usage":{"completion_tokens":17,"prompt_tokens":22,"total_tokens":39,"completion_tokens_details":null,"prompt_tokens_details":{"audio_tokens":null,"cached_tokens":0}}}"#,
            ];

            for stream in streams {
                let parsed = Completion::parse_string(stream);
                match parsed {
                    Ok(completion) => {
                        println!("Deserialized: {:#?}", completion);
                    }
                    Err(e) => {
                        panic!("Failed to deserialize {}: {}", stream, e);
                    }
                }
            }
        }
    }
}

pub mod no_streaming {
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
}
