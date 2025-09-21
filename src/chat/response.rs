pub mod streaming {
    use std::str::FromStr;

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

    impl FromStr for ChatCompletion {
        type Err = crate::errors::ResponseError;

        fn from_str(content: &str) -> Result<Self, Self::Err> {
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
                let parsed = ChatCompletion::from_str(stream);
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
                let parsed = ChatCompletion::from_str(stream);
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
    use std::str::FromStr;

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
        pub logprobs: Option<ChoiceLogprobs>,
        /// A chat completion message generated by the model.
        pub message: ChatCompletionMessage,
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

    /// Fields that are not supported yet:
    /// - _audio_: If the audio output modality is requested, this object contains
    /// data about the audio response from the model.
    /// [Learn more from OpenAI](https://platform.openai.com/docs/guides/audio).
    #[derive(Debug, Deserialize)]
    pub struct ChatCompletionMessage {
        /// The role of the author of this message. This shall always
        /// be ResponseRole::Assistant
        pub role: ResponseRole,
        /// The contents of the message.
        pub content: Option<String>,
        pub reasoning_content: Option<String>,
        /// The tool calls generated by the model, such as function calls.
        /// Tool calls deserialization is not supported yet.
        pub tool_calls: Option<Vec<ChatCompletionMessageToolCall>>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(tag = "type", rename_all = "snake_case")]
    pub enum ChatCompletionMessageToolCall {
        /// The type of the tool. Currently, only `function` is supported.
        /// The field { type = "function" } is added automatically.
        Function {
            /// The ID of the tool call.
            id: String,
            /// The function that the model called.
            function: String, // function type
        },
        /// The type of the tool. Always `custom`.
        /// The field { type = "custom" } is added automatically.
        Custom {
            /// The id of the tool call.
            id: String,
            /// The custom tool that the model called.
            custom: MessageToolCallCustom,
        },
    }

    #[derive(Debug, Deserialize)]
    pub struct MessageToolCallCustom {
        /// The input for the custom tool call generated by the model.
        pub input: String,
        /// The name of the custom tool to call.
        pub name: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct MessageToolCallFunction {
        /// The arguments to call the function with, as generated by the model in JSON
        /// format. Note that the model does not always generate valid JSON, and may
        /// hallucinate parameters not defined by your function schema. Validate the
        /// arguments in your code before calling your function.
        pub arguments: String,
        /// The name of the function to call.
        pub name: String,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum ResponseRole {
        /// The role of the response message is always assistant.
        Assistant,
    }

    #[derive(Debug, Deserialize)]
    pub struct ChoiceLogprobs {
        /// A list of message content tokens with log probability information.
        pub content: Option<Vec<TokenLogProb>>,
        /// Only found in DeepSeek's manual.
        pub reasoning_content: Option<Vec<TokenLogProb>>,
        /// A list of message refusal tokens with log probability information.
        pub refusal: Option<Vec<TokenLogProb>>,
    }

    #[derive(Debug, Deserialize)]
    pub struct TokenLogProb {
        /// The token.
        pub token: String,
        /// The log probability of this token, if it is within the top 20 most likely
        /// tokens. Otherwise, the value `-9999.0` is used to signify that the token is very
        /// unlikely.
        pub logprob: f32,
        /// A list of integers representing the UTF-8 bytes representation of the token.
        ///
        /// Useful in instances where characters are represented by multiple tokens and
        /// their byte representations must be combined to generate the correct text
        /// representation. Can be `null` if there is no bytes representation for the token.
        pub bytes: Option<Vec<u8>>,
        /// List of the most likely tokens and their log probability, at this token
        /// position. In rare cases, there may be fewer than the number of requested
        /// `top_logprobs` returned.
        pub top_logprobs: Vec<TopLogprob>,
    }

    #[derive(Debug, Deserialize)]
    pub struct TopLogprob {
        /// The token.
        pub token: String,
        /// A list of integers representing the UTF-8 bytes representation of the token.
        ///
        /// Useful in instances where characters are represented by multiple tokens and
        /// their byte representations must be combined to generate the correct text
        /// representation. Can be `null` if there is no bytes representation for the token.
        pub logprob: f32,
        /// List of the most likely tokens and their log probability, at this token
        /// position. In rare cases, there may be fewer than the number of requested
        /// `top_logprobs` returned.
        pub bytes: Option<Vec<u8>>,
    }

    #[derive(Debug, Deserialize)]
    pub struct CompletionUsage {
        /// Number of tokens in the generated completion.
        pub completion_tokens: usize,
        /// Number of tokens in the prompt.
        pub prompt_tokens: usize,

        // These two fields seem to be DeepSeek specific.
        /// Number of tokens in the prompt that hits the context cache.
        pub prompt_cache_hit_tokens: Option<usize>,
        /// Number of tokens in the prompt that misses the context cache.
        pub prompt_cache_miss_tokens: Option<usize>,

        /// Total number of tokens used in the request (prompt + completion).
        pub total_tokens: usize,
        /// Breakdown of tokens used in a completion.
        pub completion_tokens_details: Option<CompletionTokensDetails>,
        /// Breakdown of tokens used in the prompt.
        pub prompt_tokens_details: Option<PromptTokensDetails>,
    }

    #[derive(Debug, Deserialize)]
    pub struct CompletionTokensDetails {
        /// When using Predicted Outputs, the number of tokens in the prediction that
        /// appeared in the completion.
        pub accepted_prediction_tokens: Option<usize>,
        /// Audio input tokens generated by the model.
        pub audio_tokens: Option<usize>,
        /// Tokens generated by the model for reasoning.
        pub reasoning_tokens: Option<usize>,
        /// When using Predicted Outputs, the number of tokens in the prediction that did
        /// not appear in the completion. However, like reasoning tokens, these tokens are
        /// still counted in the total completion tokens for purposes of billing, output,
        /// and context window limits.
        pub rejected_prediction_tokens: Option<usize>,
    }

    #[derive(Debug, Deserialize)]
    pub struct PromptTokensDetails {
        /// Audio input tokens present in the prompt.
        pub audio_tokens: Option<usize>,
        /// Cached tokens present in the prompt.
        pub cached_tokens: Option<usize>,
    }

    impl FromStr for ChatCompletion {
        type Err = crate::errors::ResponseError;

        fn from_str(content: &str) -> Result<Self, Self::Err> {
            let parse_result: Result<ChatCompletion, _> = serde_json::from_str(content)
                .map_err(|e| ResponseError::DeserializationError(e.to_string()));
            parse_result
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

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

            let parsed = ChatCompletion::from_str(json);
            match parsed {
                Ok(_) => {}
                Err(e) => {
                    panic!("Failed to deserialize: {}", e);
                }
            }
        }
    }
}
