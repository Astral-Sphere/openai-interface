//! This module contains the request body and POST method for the chat completion API.

use std::collections::HashMap;

use serde::Serialize;

use crate::{
    chat::ServiceTier,
    rest::post::{Post, PostNoStream, PostStream},
};

/// Creates a model response for the given chat conversation.
///
/// # Example
///
/// ```rust
/// use std::sync::LazyLock;
/// use futures_util::StreamExt;
/// use openai_interface::chat::request::{Message, RequestBody};
/// use openai_interface::rest::post::PostStream;
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
///         .get_stream_response_string(DEEPSEEK_CHAT_URL, *DEEPSEEK_API_KEY)
///         .await
///         .unwrap();
///
///     while let Some(chunk) = response.next().await {
///         println!("{}", chunk.unwrap());
///     }
/// }
/// ```
#[derive(Serialize, Debug, Default, Clone)]
pub struct RequestBody {
    /// Other request bodies that are not in standard OpenAI API.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub extra_body: Option<ExtraBody>,

    /// Other request bodies that are not in standard OpenAI API and
    /// not included in the ExtraBody struct.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub extra_body_map: Option<serde_json::Map<String, serde_json::Value>>,

    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on their
    /// existing frequency in the text so far, decreasing the model's likelihood to
    /// repeat the same line verbatim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,

    /// Whether to return log probabilities of the output tokens or not. If true,
    /// returns the log probabilities of each output token returned in the `content` of
    /// `message`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,

    /// An upper bound for the number of tokens that can be generated for a completion,
    /// including visible output tokens and reasoning tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<u32>,

    /// The maximum number of tokens that can be generated in the chat completion.
    /// Deprecated according to OpenAI's Python SDK in favour of
    /// `max_completion_tokens`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,

    /// A list of messages comprising the conversation so far.
    pub messages: Vec<Message>,

    /// Set of 16 key-value pairs that can be attached to an object. This can be useful
    /// for storing additional information about the object in a structured format, and
    /// querying for objects via API or the dashboard.
    ///
    /// Keys are strings with a maximum length of 64 characters. Values are strings with
    /// a maximum length of 512 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,

    /// Output types that you would like the model to generate. Most models are capable
    /// of generating text, which is the default:
    ///
    /// `["text"]`
    ///
    /// The `gpt-4o-audio-preview` model can also be used to
    /// [generate audio](https://platform.openai.com/docs/guides/audio). To request that
    /// this model generate both text and audio responses, you can use:
    ///
    /// `["text", "audio"]`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modalities: Option<Vec<Modality>>,

    /// Name of the model to use to generate the response.
    pub model: String, // The type of this attribute needs improvements.

    /// How many chat completion choices to generate for each input message. Note that
    /// you will be charged based on the number of generated tokens across all of the
    /// choices. Keep `n` as `1` to minimize costs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,

    /// Whether to enable
    /// [parallel function calling](https://platform.openai.com/docs/guides/function-calling#configuring-parallel-function-calling)
    /// during tool use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,

    /// Static predicted output content, such as the content of a text file that is
    /// being regenerated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prediction: Option<ChatCompletionPredictionContentParam>,

    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on
    /// whether they appear in the text so far, increasing the model's likelihood to
    /// talk about new topics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,

    /// Used by OpenAI to cache responses for similar requests to optimize your cache
    /// hit rates. Replaces the `user` field.
    /// [Learn more](https://platform.openai.com/docs/guides/prompt-caching).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,

    /// Constrains effort on reasoning for
    /// [reasoning models](https://platform.openai.com/docs/guides/reasoning). Currently
    /// supported values are `minimal`, `low`, `medium`, and `high`. Reducing reasoning
    /// effort can result in faster responses and fewer tokens used on reasoning in a
    /// response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<String>,

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
    pub response_format: Option<ResponseFormat>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<ServiceTier>,

    /// Up to 4 sequences where the API will stop generating further tokens. The
    /// returned text will not contain the stop sequence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<StopKeywords>,

    /// Whether or not to store the output of this chat completion request for use in
    /// our [model distillation](https://platform.openai.com/docs/guides/distillation)
    /// or [evals](https://platform.openai.com/docs/guides/evals) products.
    ///
    /// Supports text and image inputs. Note: image inputs over 8MB will be dropped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,

    /// Although it is optional, you should explicitly designate it
    /// for an expected response.
    pub stream: bool,

    /// Options for streaming response. Only set this when you set `stream: true`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamOptions>,

    /// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will
    /// make the output more random, while lower values like 0.2 will make it more
    /// focused and deterministic. It is generally recommended to alter this or `top_p` but
    /// not both.
    pub temperature: Option<f32>,

    /// Controls which (if any) tool is called by the model. `none` means the model will
    /// not call any tool and instead generates a message. `auto` means the model can
    /// pick between generating a message or calling one or more tools. `required` means
    /// the model must call one or more tools. Specifying a particular tool via
    /// `{"type": "function", "function": {"name": "my_function"}}` forces the model to
    /// call that tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,

    /// A list of tools the model may call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<RequestTool>>,

    /// An integer between 0 and 20 specifying the number of most likely tokens to
    /// return at each token position, each with an associated log probability.
    /// `logprobs` must be set to `true` if this parameter is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u32>,

    /// An alternative to sampling with temperature, called nucleus sampling, where the
    /// model considers the results of the tokens with top_p probability mass. So 0.1
    /// means only the tokens comprising the top 10% probability mass are considered.
    ///
    /// It is generally recommended to alter this or `temperature` but not both.
    pub top_p: Option<f32>,

    /// This field is being replaced by `safety_identifier` and `prompt_cache_key`. Use
    /// `prompt_cache_key` instead to maintain caching optimizations. A stable
    /// identifier for your end-users. Used to boost cache hit rates by better bucketing
    /// similar requests and to help OpenAI detect and prevent abuse.
    /// [Learn more](https://platform.openai.com/docs/guides/safety-best-practices#safety-identifiers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    /// Constrains the verbosity of the model's response. Lower values will result in
    /// more concise responses, while higher values will result in more verbose
    /// responses. Currently supported values are `low`, `medium`, and `high`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbosity: Option<LowMediumHighEnum>,

    /// This tool searches the web for relevant results to use in a response. Learn more
    /// about the
    /// [web search tool](https://platform.openai.com/docs/guides/tools-web-search?api-mode=chat).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_search: Option<WebSearchOptions>,
}

#[derive(Serialize, Debug, Clone)]
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

#[derive(Debug, Serialize, Clone)]
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

#[derive(Debug, Serialize, Clone)]
pub struct ToolCallFunction {
    /// The arguments to call the function with, as generated by the model in JSON
    /// format. Note that the model does not always generate valid JSON, and may
    /// hallucinate parameters not defined by your function schema. Validate the
    /// arguments in your code before calling your function.
    arguments: String,
    /// The name of the function to call.
    name: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ToolCallCustom {
    /// The input for the custom tool call generated by the model.
    input: String,
    /// The name of the custom tool to call.
    name: String,
}

#[derive(Debug, Serialize, Clone)]
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

#[derive(Debug, Serialize, Clone)]
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

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Modality {
    Text,
    Audio,
}

#[derive(Serialize, Debug, Clone)]
pub struct ChatCompletionPredictionContentParam {
    /// The content that should be matched when generating a model response. If
    /// generated tokens would match this content, the entire model response can be
    /// returned much more quickly.
    pub content: ChatCompletionPredictionContentParamContent,

    /// The type of the predicted content you want to provide.
    /// This type is currently always `content`.
    pub type_: ChatCompletionPredictionContentParamType,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ChatCompletionPredictionContentParamContent {
    Text(String),
    ChatCompletionContentPartTextParam {
        /// The text content.
        text: String,
        /// The type of the content part.
        #[serde(rename = "type")]
        type_: ChatCompletionContentPartTextParamType,
    },
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ChatCompletionContentPartTextParamType {
    Text,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ChatCompletionPredictionContentParamType {
    Content,
}

fn is_false(value: &bool) -> bool {
    !value
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum StopKeywords {
    Word(String),
    Words(Vec<String>),
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum LowMediumHighEnum {
    Low,
    Medium,
    High,
}

#[derive(Serialize, Debug, Clone)]
pub struct WebSearchOptions {
    /// High level guidance for the amount of context window space to use for the
    /// search. One of `low`, `medium`, or `high`. `medium` is the default.
    pub search_context_size: LowMediumHighEnum,

    pub user_location: Option<WebSearchOptionsUserLocation>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WebSearchOptionsUserLocation {
    /// The type of location approximation. Always `approximate`.
    Approximate(WebSearchOptionsUserLocationApproximate),
}

#[derive(Serialize, Debug, Clone)]
pub struct WebSearchOptionsUserLocationApproximate {
    /// Free text input for the city of the user, e.g. `San Francisco`.
    pub city: String,

    /// The two-letter [ISO country code](https://en.wikipedia.org/wiki/ISO_3166-1) of
    /// the user, e.g. `US`.
    pub country: String,

    /// Free text input for the region of the user, e.g. `California`.
    pub region: String,

    /// The [IANA timezone](https://timeapi.io/documentation/iana-timezones) of the
    /// user, e.g. `America/Los_Angeles`.
    pub timezone: String,
}

#[derive(Serialize, Debug, Clone)]
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

#[derive(Serialize, Debug, Clone)]
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

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningEffort {
    Minimal,
    Low,
    Medium,
    High,
}

#[derive(Serialize, Debug, Clone)]
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

#[derive(Serialize, Debug, Clone)]
pub struct ToolCustom {
    /// The name of the custom tool, used to identify it in tool calls.
    pub name: String,
    /// Optional description of the custom tool, used to provide more context.
    pub description: String,
    /// The input format for the custom tool. Default is unconstrained text.
    pub format: String,
}

#[derive(Serialize, Debug, Clone)]
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

#[derive(Debug, Serialize, Clone)]
pub struct ToolCustomFormatGrammarGrammar {
    /// The grammar definition.
    pub definition: String,
    /// The syntax of the grammar definition. One of `lark` or `regex`.
    pub syntax: ToolCustomFormatGrammarGrammarSyntax,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ToolCustomFormatGrammarGrammarSyntax {
    Lark,
    Regex,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ToolChoice {
    None,
    Auto,
    Required,
    #[serde(untagged)]
    Specific(ToolChoiceSpecific),
}

#[derive(Debug, Serialize, Clone)]
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

#[derive(Debug, Serialize, Clone)]
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
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ToolChoiceAllowedToolsMode {
    /// The model can choose whether to use the allowed tools or not.
    Auto,
    /// The model must use at least one of the allowed tools.
    Required,
}

#[derive(Debug, Serialize, Clone)]
pub struct ToolChoiceFunction {
    /// The name of the function to call.
    pub name: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ToolChoiceCustom {
    /// The name of the custom tool to call.
    pub name: String,
}

#[derive(Debug, Serialize, Clone)]
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

impl PostNoStream for RequestBody {
    type Response = super::response::no_streaming::ChatCompletion;
}

impl PostStream for RequestBody {
    type Response = super::response::streaming::ChatCompletionChunk;
}

#[cfg(test)]
mod request_test {
    use std::sync::LazyLock;

    use futures_util::StreamExt;

    use super::*;

    const DEEPSEEK_API_KEY: LazyLock<&str> =
        LazyLock::new(|| include_str!("../../../keys/deepseek_domestic_key").trim());
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
            .get_response_string(DEEPSEEK_CHAT_URL, &*DEEPSEEK_API_KEY)
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
            .get_stream_response_string(DEEPSEEK_CHAT_URL, *DEEPSEEK_API_KEY)
            .await
            .unwrap();

        while let Some(chunk) = response.next().await {
            println!("{}", chunk.unwrap());
        }
    }
}
