pub mod streaming {
    use serde::Deserialize;

    use crate::errors::ResponseError;

    #[derive(Debug, Deserialize)]
    pub struct ChatCompletion {
        /// A unique identifier for the chat completion.
        pub id: String,
        /// A list of chat completion choices. Can be more than one
        /// if `n` is greater than 1.
        pub choices: Vec<CompletionChoice>,
        /// The Unix timestamp (in seconds) of when the chat completion was created.
        pub created: u64,
        /// The model used for the chat completion.
        pub model: String,
        /// The object type, which is always `chat.completion`
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

    impl ChatCompletion {
        pub fn parse_string(content: &str) -> Result<Self, crate::errors::ResponseError> {
            let parse_result: Result<ChatCompletion, _> = serde_json::from_str(content)
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
                let parsed = ChatCompletion::parse_string(stream);
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
                let parsed = ChatCompletion::parse_string(stream);
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
    pub struct ChatCompletion {
        /// A unique identifier for the chat completion.
        pub id: String,
        /// A list of chat completion choices. Can be more than one
        /// if `n` is greater than 1.
        pub choices: Vec<Choice>,
        /// The Unix timestamp (in seconds) of when the chat completion was created.
        pub created: u64,
        /// The model used for the chat completion.
        pub model: String,
        /// Specifies the processing type used for serving the request.
        ///
        /// - If set to 'auto', then the request will be processed with the service tier
        ///   configured in the Project settings. Unless otherwise configured, the Project
        ///   will use 'default'.
        /// - If set to 'default', then the request will be processed with the standard
        ///   pricing and performance for the selected model.
        /// - If set to '[flex](https://platform.openai.com/docs/guides/flex-processing)' or
        ///   '[priority](https://openai.com/api-priority-processing/)', then the request
        ///   will be processed with the corresponding service tier.
        /// - When not set, the default behavior is 'auto'.
        ///
        /// When the `service_tier` parameter is set, the response body will include the
        /// `service_tier` value based on the processing mode actually used to serve the
        /// request. This response value may be different from the value set in the
        /// parameter.
        pub service_tier: Option<ServiceTier>,
        /// The system fingerprint used for the chat completion.
        /// Can be used in conjunction with the `seed` request parameter to understand when
        /// backend changes have been made that might impact determinism.
        pub system_fingerprint: Option<String>,
        /// The object type, which is always `chat.completion`.
        pub object: ChatCompletionObject,
        /// Usage statistics for the completion request.
        pub usage: Option<CompletionUsage>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum ServiceTier {
        Auto,
        Default,
        Flex,
        Scale,
        Priority,
    }

    /// The object type, which is always `chat.completion`.
    #[derive(Debug, Deserialize)]
    pub enum ChatCompletionObject {
        /// The object type is always `chat.completion`.
        #[serde(rename = "chat.completion")]
        ChatCompletion,
    }

    #[derive(Debug, Deserialize)]
    pub struct Choice {
        /// The reason the model stopped generating tokens.
        ///
        /// This will be `stop` if the model hit a natural stop point or a provided stop
        /// sequence, `length` if the maximum number of tokens specified in the request was
        /// reached, `content_filter` if content was omitted due to a flag from our content
        /// filters, `tool_calls` if the model called a tool, or `function_call`
        /// (deprecated) if the model called a function.
        pub finish_reason: FinishReason,
        /// The index of the choice in the list of choices.
        pub index: usize,
        /// Log probability information for the choice.
        pub logprobs: Option<ResponseLogprobs>,
        /// A chat completion message generated by the model.
        pub message: ResponseMessage,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    #[serde(rename_all = "snake_case")]
    pub enum FinishReason {
        Length,
        Stop,
        ToolCalls,
        FunctionCall,
        ContentFilter,
        /// This choice can only be found in the manual of DeepSeek
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

    impl ChatCompletion {
        pub fn parse_string(content: &str) -> Result<Self, crate::errors::ResponseError> {
            let parse_result: Result<ChatCompletion, _> = serde_json::from_str(content)
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

            let parsed = super::ChatCompletion::parse_string(json);
            match parsed {
                Ok(_) => {}
                Err(e) => {
                    panic!("Failed to deserialize: {}", e);
                }
            }
        }
    }
}
