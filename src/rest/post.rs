use std::{future::Future, str::FromStr};

use eventsource_stream::Eventsource;
use futures_util::{StreamExt, TryStreamExt, stream::BoxStream};
use serde::{Serialize, de::DeserializeOwned};

use crate::errors::OapiError;

pub trait Post {
    fn is_streaming(&self) -> bool;
}

pub trait NoStream: Post + Serialize + Sync + Send {
    type Response: DeserializeOwned + FromStr<Err = OapiError> + Send + Sync;

    /// Sends a POST request to the specified URL with the provided api-key.
    fn get_response_string(
        &self,
        url: &str,
        key: &str,
    ) -> impl Future<Output = Result<String, OapiError>> + Send + Sync {
        async move {
            if self.is_streaming() {
                return Err(OapiError::NonStreamingViolation);
            }

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
                .map_err(|e| OapiError::SendError(format!("Failed to send request: {:#?}", e)))?;

            if response.status() != reqwest::StatusCode::OK {
                return Err(
                    crate::errors::OapiError::ResponseStatus(response.status().as_u16()).into(),
                );
            }

            let text = response.text().await.map_err(|e| {
                OapiError::ResponseError(format!("Failed to get response text: {:#?}", e))
            })?;

            Ok(text)
        }
    }

    fn get_response(
        &self,
        url: &str,
        key: &str,
    ) -> impl Future<Output = Result<Self::Response, OapiError>> + Send + Sync {
        async move {
            let text = self.get_response_string(url, key).await?;
            let result = Self::Response::from_str(&text)?;
            Ok(result)
        }
    }
}

pub trait Stream: Post + Serialize + Sync + Send {
    type Response: DeserializeOwned + FromStr<Err = OapiError> + Send + Sync;

    /// Sends a streaming POST request to the specified URL with the provided api-key.
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
    ///         .get_stream_response_string(DEEPSEEK_CHAT_URL, *DEEPSEEK_API_KEY)
    ///         .await
    ///         .unwrap();
    ///
    ///     while let Some(chunk) = response.next().await {
    ///         println!("{}", chunk.unwrap());
    ///     }
    /// }
    /// ```
    fn get_stream_response_string(
        &self,
        url: &str,
        api_key: &str,
    ) -> impl Future<Output = Result<BoxStream<'static, Result<String, OapiError>>, OapiError>>
    + Send
    + Sync {
        async move {
            if !self.is_streaming() {
                return Err(OapiError::StreamingViolation);
            }

            let client = reqwest::Client::new();

            let response = client
                .post(url)
                .headers({
                    let mut headers = reqwest::header::HeaderMap::new();
                    headers.insert("Content-Type", "application/json".parse().unwrap());
                    headers.insert("Accept", "text/event-stream".parse().unwrap());
                    headers
                })
                .bearer_auth(api_key)
                .json(self)
                .send()
                .await
                .map_err(|e| OapiError::ResponseError(format!("Failed to send request: {}", e)))?;

            if !response.status().is_success() {
                return Err(OapiError::ResponseStatus(response.status().as_u16()).into());
            }

            // The following code is generated by Qwen3-480B-Coder
            // 使用 eventsource-stream 解析 SSE
            let stream = response
                .bytes_stream()
                .eventsource()
                .map(|event| match event {
                    Ok(event) => Ok(event.data),
                    Err(e) => Err(OapiError::SseParseError(format!("SSE parse error: {}", e))),
                })
                .boxed();

            Ok(stream as BoxStream<'static, Result<String, OapiError>>)
        }
    }

    fn get_stream_response(
        &self,
        url: &str,
        api_key: &str,
    ) -> impl Future<
        Output = Result<BoxStream<'static, Result<Self::Response, OapiError>>, OapiError>,
    > + Send
    + Sync {
        async move {
            let stream = self.get_stream_response_string(url, api_key).await?;

            let parsed_stream = stream
                .take_while(|result| {
                    let should_continue = match result {
                        Ok(data) => data != "[DONE]",
                        Err(_) => true, // 继续传播错误
                    };
                    async move { should_continue }
                })
                .and_then(|data| async move { Self::Response::from_str(&data) });

            Ok(Box::pin(parsed_stream) as BoxStream<'static, _>)
        }
    }
}
