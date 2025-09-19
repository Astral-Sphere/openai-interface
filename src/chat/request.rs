use serde::{Deserialize, Serialize};

use futures_util::{TryStreamExt, stream::BoxStream};

use crate::errors::RequestError;

#[derive(Serialize, Deserialize, Debug, Default)]
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
    pub tools: Option<Vec<Tools>>,

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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolCallFunction {
    /// The arguments to call the function with, as generated by the model in JSON
    /// format. Note that the model does not always generate valid JSON, and may
    /// hallucinate parameters not defined by your function schema. Validate the
    /// arguments in your code before calling your function.
    arguments: String,
    /// The name of the function to call.
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolCallCustom {
    /// The input for the custom tool call generated by the model.
    input: String,
    /// The name of the custom tool to call.
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum StopKeywords {
    Word(String),
    Words(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Tools {
    #[serde(rename = "type")]
    pub type_: String,
    pub function: Option<Vec<ToolFunction>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToolFunction {
    name: String,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    strict: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToolFunctionParameter {
    name: String,
    description: String,
    required: bool,
    parameters: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ToolChoice {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "required")]
    Required,
    #[serde(untagged)]
    Specific {
        /// This parameter should always be "function" literal.
        #[serde(rename = "type")]
        type_: ToolChoiceSpecificType,
        function: ToolChoiceFunction,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToolChoiceFunction {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ToolChoiceSpecificType {
    Function,
}

#[derive(Serialize, Deserialize, Debug)]
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

impl RequestBody {
    pub async fn get_response(&self, url: &str, key: &str) -> anyhow::Result<String> {
        assert!(!self.stream);

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert("Content-Type", "application/json".parse().unwrap());
                headers.insert("Accept", "application/json".parse().unwrap());
                headers
            })
            .bearer_auth(key)
            .json(self)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send request: {}", e))?;

        if response.status() != reqwest::StatusCode::OK {
            return Err(
                crate::errors::RequestError::ResponseStatus(response.status().as_u16()).into(),
            );
        }

        let text = response.text().await?;

        Ok(text)
    }

    /// Getting stream response. You must ensure self.stream is true, or otherwise it will panic.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::sync::LazyLock;
    /// use futures_util::StreamExt;
    /// use openai_interface::chat::request::{Message, RequestBody};
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
    ///         .stream_response(DEEPSEEK_CHAT_URL, *DEEPSEEK_API_KEY)
    ///         .await
    ///         .unwrap();
    ///
    ///     while let Some(chunk) = response.next().await {
    ///         println!("{}", chunk.unwrap());
    ///     }
    /// }
    /// ```
    pub async fn stream_response(
        &self,
        url: &str,
        api_key: &str,
    ) -> Result<BoxStream<'static, Result<String, anyhow::Error>>, anyhow::Error> {
        // 断言开启了流模式
        assert!(
            self.stream,
            "RequestBody::stream_response requires `stream: true`"
        );

        let client = reqwest::Client::new();

        let response = client
            .post(url)
            .headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert("Content-Type", "application/json".parse().unwrap());
                headers.insert("Accept", "application/json".parse().unwrap());
                headers
            })
            .bearer_auth(api_key)
            .json(self)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send request: {}", e))?;

        if !response.status().is_success() {
            return Err(RequestError::ResponseStatus(response.status().as_u16()).into());
        }

        let stream = response
            .bytes_stream()
            .map_err(|e| RequestError::StreamError(e.to_string()).into())
            .try_filter_map(|bytes| async move {
                let s = std::str::from_utf8(&bytes)
                    .map_err(|e| RequestError::SseParseError(e.to_string()))?;
                if s.starts_with("[DONE]") {
                    Ok(None)
                } else {
                    Ok(Some(s.to_string()))
                }
            });

        Ok(Box::pin(stream) as BoxStream<'static, _>)

        // return Err(anyhow!("Not implemented"));
    }
}

#[cfg(test)]
mod request_test {
    use std::sync::LazyLock;

    use futures_util::StreamExt;

    use crate::chat::request::{Message, RequestBody};

    const DEEPSEEK_API_KEY: LazyLock<&str> =
        LazyLock::new(|| include_str!("../.././keys/deepseek_domestic_key").trim());
    const DEEPSEEK_CHAT_URL: &'static str = "https://api.deepseek.com/chat/completions";
    const DEEPSEEK_MODEL: &'static str = "deepseek-chat";

    #[tokio::test]
    async fn test_00_basics() {
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
    async fn test_01_streaming() {
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
            .stream_response(DEEPSEEK_CHAT_URL, *DEEPSEEK_API_KEY)
            .await
            .unwrap();

        while let Some(chunk) = response.next().await {
            println!("{}", chunk.unwrap());
        }
    }
}
