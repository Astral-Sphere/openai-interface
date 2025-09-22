//! This module contains the request body and POST method for the chat completion API.

use serde::Serialize;

use crate::rest::post::{NoStream, Post, Stream};

/// Creates a model response for the given chat conversation.
///
/// # Example
///
/// ```rust
/// use std::sync::LazyLock;
/// use futures_util::StreamExt;
/// use openai_interface::chat::request::{Message, RequestBody};
/// use openai_interface::rest::post::Stream;
///
/// const DEEPSEEK_API_KEY: LazyLock<&str> =
///     LazyLock::new(|| include_str!("../.././keys/deepseek_domestic_key").trim());
/// const DEEPSEEK_CHAT_URL: &'static str = "https://api.deepseek.com/chat/completions";
/// const DEEPSEEK_MODEL: &'static str = "deepseek-chat";
///
/// #[tokio::main]
/// async fn main() {
///     let request = RequestBody {
///         messages: vec![
///             Message::System {
///                 content: "This is a request of test purpose. Reply briefly".to_string(),
///                 name: None,
///             },
///             Message::User {
///                 content: "What's your name?".to_string(),
///                 name: None,
///             },
///         ],
///         model: DEEPSEEK_MODEL.to_string(),
///         stream: true,
///         ..Default::default()
///     };
///
///     let mut response = request
///         .get_stream_response(DEEPSEEK_CHAT_URL, *DEEPSEEK_API_KEY)
///         .await
///         .unwrap();
///
///     while let Some(chunk) = response.next().await {
///         println!("{}", chunk.unwrap());
///     }
/// }
/// ```
#[derive(Serialize, Debug, Default)]
pub struct RequestBody {
    /// A list of messages comprising the conversation so far.
    pub messages: Vec<Message>,

    /// Name of the model to use to generate the response.
    pub model: String,

    /// Although it is optional, you should explicitly designate it
    /// for an expected response.
    pub stream: bool,

    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on their
    /// existing frequency in the text so far, decreasing the model's likelihood to
    /// repeat the same line verbatim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,

    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on
    /// whether they appear in the text so far, increasing the model's likelihood to
    /// talk about new topics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,

    /// The maximum number of tokens that can be generated in the chat completion.
    /// Deprecated according to OpenAI's Python SDK in favour of
    /// `max_completion_tokens`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,

    /// An upper bound for the number of tokens that can be generated for a completion,
    /// including visible output tokens and reasoning tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<u32>,

    /// specifying the format that the model must output.
    ///
    /// Setting to `{ "type": "json_schema", "json_schema": {...} }` enables Structured
    /// Outputs which ensures the model will match your supplied JSON schema. Learn more
    /// in the
    /// [Structured Outputs guide](https://platform.openai.com/docs/guides/structured-outputs).
    /// Setting to `{ "type": "json_object" }` enables the older JSON mode, which
    /// ensures the message the model generates is valid JSON. Using `json_schema` is
    /// preferred for models that support it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>, // The type of this attribute needs improvements.

    /// A stable identifier used to help detect users of your application that may be
    /// violating OpenAI's usage policies. The IDs should be a string that uniquely
    /// identifies each user. It is recommended to hash their username or email address, in
    /// order to avoid sending any identifying information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_identifier: Option<String>,

    /// If specified, the system will make a best effort to sample deterministically. Determinism
    /// is not guaranteed, and you should refer to the `system_fingerprint` response parameter to
    /// monitor changes in the backend.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,

    /// How many chat completion choices to generate for each input message. Note that
    /// you will be charged based on the number of generated tokens across all of the
    /// choices. Keep `n` as `1` to minimize costs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,

    /// Up to 4 sequences where the API will stop generating further tokens. The
    /// returned text will not contain the stop sequence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<StopKeywords>,

    /// Options for streaming response. Only set this when you set `stream: true`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamOptions>,

    /// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will
    /// make the output more random, while lower values like 0.2 will make it more
    /// focused and deterministic. It is generally recommended to alter this or `top_p` but
    /// not both.
    pub temperature: Option<f32>,

    /// An alternative to sampling with temperature, called nucleus sampling, where the
    /// model considers the results of the tokens with top_p probability mass. So 0.1
    /// means only the tokens comprising the top 10% probability mass are considered.
    ///
    /// It is generally recommended to alter this or `temperature` but not both.
    pub top_p: Option<f32>,

    /// A list of tools the model may call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<RequestTool>>,

    /// Controls which (if any) tool is called by the model. `none` means the model will
    /// not call any tool and instead generates a message. `auto` means the model can
    /// pick between generating a message or calling one or more tools. `required` means
    /// the model must call one or more tools. Specifying a particular tool via
    /// `{"type": "function", "function": {"name": "my_function"}}` forces the model to
    /// call that tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,

    /// Whether to return log probabilities of the output tokens or not. If true,
    /// returns the log probabilities of each output token returned in the `content` of
    /// `message`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,

    /// An integer between 0 and 20 specifying the number of most likely tokens to
    /// return at each token position, each with an associated log probability.
    /// `logprobs` must be set to `true` if this parameter is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u32>,

    /// Other request bodies that are not in standard OpenAI API.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub extra_body: Option<ExtraBody>,

    /// Other request bodies that are not in standard OpenAI API and
    /// not included in the ExtraBody struct.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub extra_body_map: Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Serialize, Debug)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum Message {
    /// In this case, the role of the message author is `system`.
    /// The field `{ role = "system" }` is added automatically.
    System {
        /// The contents of the system message.
        content: String,
        /// An optional name for the participant.
        ///
        /// Provides the model information to differentiate between
        /// participants of the same role.
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    /// In this case, the role of the message author is `user`.
    /// The field `{ role = "user" }` is added automatically.
    User {
        /// The contents of the user message.
        content: String,
        /// An optional name for the participant.
        ///
        /// Provides the model information to differentiate between
        /// participants of the same role.
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    /// In this case, the role of the message author is `assistant`.
    /// The field `{ role = "assistant" }` is added automatically.
    ///
    /// Unimplemented params:
    /// - _audio_: Data about a previous audio response from the model.
    Assistant {
        /// The contents of the assistant message. Required unless `tool_calls`
        /// or `function_call` is specified. (Note that `function_call` is deprecated
        /// in favour of `tool_calls`.)
        content: Option<String>,
        /// The refusal message by the assistant.
        #[serde(skip_serializing_if = "Option::is_none")]
        refusal: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        /// Set this to true for completion
        #[serde(skip_serializing_if = "is_false")]
        prefix: bool,
        /// Used for the deepseek-reasoner model in the Chat Prefix
        /// Completion feature as the input for the CoT in the last
        /// assistant message. When using this feature, the prefix
        /// parameter must be set to true.
        #[serde(skip_serializing_if = "Option::is_none")]
        reasoning_content: Option<String>,

        /// The tool calls generated by the model, such as function calls.
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_calls: Option<Vec<AssistantToolCall>>,
    },
    /// In this case, the role of the message author is `assistant`.
    /// The field `{ role = "tool" }` is added automatically.
    Tool {
        /// The contents of the tool message.
        content: String,
        /// Tool call that this message is responding to.
        tool_call_id: String,
    },
    /// In this case, the role of the message author is `function`.
    /// The field `{ role = "function" }` is added automatically.
    Function {
        /// The contents of the function message.
        content: String,
        /// The name of the function to call.
        name: String,
    },
    /// In this case, the role of the message author is `developer`.
    /// The field `{ role = "developer" }` is added automatically.
    Developer {
        /// The contents of the developer message.
        content: String,
        /// An optional name for the participant.
        ///
        /// Provides the model information to differentiate between
        /// participants of the same role.
        name: Option<String>,
    },
}

#[derive(Debug, Serialize)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum AssistantToolCall {
    Function {
        /// The ID of the tool call.
        id: String,
        /// The function that the model called.
        function: ToolCallFunction,
    },
    Custom {
        /// The ID of the tool call.
        id: String,
        /// The custom tool that the model called.
        custom: ToolCallCustom,
    },
}

#[derive(Debug, Serialize)]
pub struct ToolCallFunction {
    /// The arguments to call the function with, as generated by the model in JSON
    /// format. Note that the model does not always generate valid JSON, and may
    /// hallucinate parameters not defined by your function schema. Validate the
    /// arguments in your code before calling your function.
    arguments: String,
    /// The name of the function to call.
    name: String,
}

#[derive(Debug, Serialize)]
pub struct ToolCallCustom {
    /// The input for the custom tool call generated by the model.
    input: String,
    /// The name of the custom tool to call.
    name: String,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseFormat {
    /// The type of response format being defined. Always `json_schema`.
    JsonSchema {
        /// Structured Outputs configuration options, including a JSON Schema.
        json_schema: JSONSchema,
    },
    /// The type of response format being defined. Always `json_object`.
    JsonObject,
    /// The type of response format being defined. Always `text`.
    Text,
}

#[derive(Debug, Serialize)]
pub struct JSONSchema {
    /// The name of the response format. Must be a-z, A-Z, 0-9, or contain
    /// underscores and dashes, with a maximum length of 64.
    pub name: String,
    /// A description of what the response format is for, used by the model to determine
    /// how to respond in the format.
    pub description: String,
    /// The schema for the response format, described as a JSON Schema object. Learn how
    /// to build JSON schemas [here](https://json-schema.org/).
    pub schema: serde_json::Map<String, serde_json::Value>,
    /// Whether to enable strict schema adherence when generating the output. If set to
    /// true, the model will always follow the exact schema defined in the `schema`
    /// field. Only a subset of JSON Schema is supported when `strict` is `true`. To
    /// learn more, read the
    /// [Structured Outputs guide](https://platform.openai.com/docs/guides/structured-outputs).
    pub strict: Option<bool>,
}

#[inline]
fn is_false(value: &bool) -> bool {
    !value
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum StopKeywords {
    Word(String),
    Words(Vec<String>),
}

#[derive(Serialize, Debug)]
pub struct StreamOptions {
    /// If set, an additional chunk will be streamed before the `data: [DONE]` message.
    ///
    /// The `usage` field on this chunk shows the token usage statistics for the entire
    /// request, and the `choices` field will always be an empty array.
    ///
    /// All other chunks will also include a `usage` field, but with a null value.
    /// **NOTE:** If the stream is interrupted, you may not receive the final usage
    /// chunk which contains the total token usage for the request.
    pub include_usage: bool,
}

#[derive(Serialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RequestTool {
    /// The type of the tool. Currently, only `function` is supported.
    Function { function: ToolFunction },
    /// The type of the custom tool. Always `custom`.
    Custom {
        /// Properties of the custom tool.
        custom: ToolCustom,
    },
}

#[derive(Serialize, Debug)]
pub struct ToolFunction {
    /// The name of the function to be called. Must be a-z, A-Z, 0-9, or
    /// contain underscores and dashes, with a maximum length
    /// of 64.
    pub name: String,
    /// A description of what the function does, used by the model to choose when and
    /// how to call the function.
    pub description: String,
    /// The parameters the functions accepts, described as a JSON Schema object.
    ///
    /// See the
    /// [openai function calling guide](https://platform.openai.com/docs/guides/function-calling)
    /// for examples, and the
    /// [JSON Schema reference](https://json-schema.org/understanding-json-schema/) for
    /// documentation about the format.
    ///
    /// Omitting `parameters` defines a function with an empty parameter list.
    pub parameters: serde_json::Map<String, serde_json::Value>,
    /// Whether to enable strict schema adherence when generating the function call.
    ///
    /// If set to true, the model will follow the exact schema defined in the
    /// `parameters` field. Only a subset of JSON Schema is supported when `strict` is
    /// `true`. Learn more about Structured Outputs in the
    /// [openai function calling guide](https://platform.openai.com/docs/guides/function-calling).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

#[derive(Serialize, Debug)]
pub struct ToolCustom {
    /// The name of the custom tool, used to identify it in tool calls.
    pub name: String,
    /// Optional description of the custom tool, used to provide more context.
    pub description: String,
    /// The input format for the custom tool. Default is unconstrained text.
    pub format: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum ToolCustomFormat {
    /// Unconstrained text format. Always `text`.
    CustomFormatText,
    /// Grammar format. Always `grammar`.
    CustomFormatGrammar {
        /// Your chosen grammar.
        grammar: ToolCustomFormatGrammarGrammar,
    },
}

#[derive(Debug, Serialize)]
pub struct ToolCustomFormatGrammarGrammar {
    /// The grammar definition.
    pub definition: String,
    /// The syntax of the grammar definition. One of `lark` or `regex`.
    pub syntax: ToolCustomFormatGrammarGrammarSyntax,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolCustomFormatGrammarGrammarSyntax {
    Lark,
    Regex,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolChoice {
    None,
    Auto,
    Required,
    #[serde(untagged)]
    Specific(ToolChoiceSpecific),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum ToolChoiceSpecific {
    /// Allowed tool configuration type. Always `allowed_tools`.
    AllowedTools {
        /// Constrains the tools available to the model to a pre-defined set.
        allowed_tools: ToolChoiceAllowedTools,
    },
    /// For function calling, the type is always `function`.
    Function { function: ToolChoiceFunction },
    /// For custom tool calling, the type is always `custom`.
    Custom { custom: ToolChoiceCustom },
}

#[derive(Debug, Serialize)]
pub struct ToolChoiceAllowedTools {
    /// Constrains the tools available to the model to a pre-defined set.
    ///
    /// - `auto` allows the model to pick from among the allowed tools and generate a
    /// message.
    /// - `required` requires the model to call one or more of the allowed tools.
    pub mode: ToolChoiceAllowedToolsMode,
    /// A list of tool definitions that the model should be allowed to call.
    ///
    /// For the Chat Completions API, the list of tool definitions might look like:
    ///
    /// ```json
    /// [
    ///   { "type": "function", "function": { "name": "get_weather" } },
    ///   { "type": "function", "function": { "name": "get_time" } }
    /// ]
    /// ```
    pub tools: serde_json::Map<String, serde_json::Value>,
}

/// The mode for allowed tools in tool choice.
///
/// Controls how the model should handle the set of allowed tools:
///
/// - `auto` allows the model to pick from among the allowed tools and generate a
///   message.
/// - `required` requires the model to call one or more of the allowed tools.
#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolChoiceAllowedToolsMode {
    /// The model can choose whether to use the allowed tools or not.
    Auto,
    /// The model must use at least one of the allowed tools.
    Required,
}

#[derive(Debug, Serialize)]
pub struct ToolChoiceFunction {
    /// The name of the function to call.
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct ToolChoiceCustom {
    /// The name of the custom tool to call.
    pub name: String,
}

#[derive(Serialize, Debug)]
pub struct ExtraBody {
    /// Make sense only for Qwen API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_thinking: Option<bool>,
    /// Make sense only for Qwen API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking_budget: Option<u32>,
    ///The size of the candidate set for sampling during generation.
    ///
    /// Make sense only for Qwen API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,
}

impl Post for RequestBody {
    fn is_streaming(&self) -> bool {
        self.stream
    }
}

impl NoStream for RequestBody {}

impl Stream for RequestBody {}

#[cfg(test)]
mod request_test {
    use std::sync::LazyLock;

    use futures_util::StreamExt;

    use super::*;

    const DEEPSEEK_API_KEY: LazyLock<&str> =
        LazyLock::new(|| include_str!("../.././keys/deepseek_domestic_key").trim());
    const DEEPSEEK_CHAT_URL: &'static str = "https://api.deepseek.com/chat/completions";
    const DEEPSEEK_MODEL: &'static str = "deepseek-chat";

    #[tokio::test]
    async fn test_deepseek_no_stream() {
        let request = RequestBody {
            messages: vec![
                Message::System {
                    content: "This is a request of test purpose. Reply briefly".to_string(),
                    name: None,
                },
                Message::User {
                    content: "What's your name?".to_string(),
                    name: None,
                },
            ],
            model: DEEPSEEK_MODEL.to_string(),
            stream: false,
            ..Default::default()
        };

        let response = request
            .get_response(DEEPSEEK_CHAT_URL, &*DEEPSEEK_API_KEY)
            .await
            .unwrap();

        println!("{}", response);

        assert!(response.to_ascii_lowercase().contains("deepseek"));
    }

    #[tokio::test]
    async fn test_deepseek_stream() {
        let request = RequestBody {
            messages: vec![
                Message::System {
                    content: "This is a request of test purpose. Reply briefly".to_string(),
                    name: None,
                },
                Message::User {
                    content: "What's your name?".to_string(),
                    name: None,
                },
            ],
            model: DEEPSEEK_MODEL.to_string(),
            stream: true,
            ..Default::default()
        };

        let mut response = request
            .get_stream_response(DEEPSEEK_CHAT_URL, *DEEPSEEK_API_KEY)
            .await
            .unwrap();

        while let Some(chunk) = response.next().await {
            println!("{}", chunk.unwrap());
        }
    }
}
