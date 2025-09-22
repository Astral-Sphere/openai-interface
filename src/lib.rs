//! A low-level Rust interface for interacting with OpenAI's API.
//!
//! This crate provides a simple, efficient but low-level way to interact with OpenAI's API,
//! supporting both streaming and non-streaming responses. It utilizes Rust's powerful type
//! system.
//!
//! # Features
//!
//! - **Chat Completions**: Full support for OpenAI's chat completion API
//! - **Streaming and Non-streaming**: Support for both streaming and non-streaming responses
//! - **Strong Typing**: Complete type definitions for all API requests and responses
//! - **Error Handling**: Comprehensive error handling with detailed error types
//! - **Async/Await**: Built with async/await support for efficient asynchronous operations
//! - **Musl Support**: Designed to work with musl libc for lightweight deployments
//!
//! # Modules
//!
//! - [`chat`]: Contains all chat completion related structs, enums and methods.
//! - [`errors`]: Defines error types used throughout the crate
//!
//! # Examples
//!
//! ## Non-streaming Chat Completion
//!
//! ```rust
//! use std::sync::LazyLock;
//! use openai_interface::chat::request::{Message, RequestBody};
//! use openai_interface::chat::response::no_streaming::ChatCompletion;
//! use std::str::FromStr;
//!
//! // You need to provide your own DeepSeek API key at /keys/deepseek_domestic_key
//! const DEEPSEEK_API_KEY: LazyLock<&str> =
//!    LazyLock::new(|| include_str!("../keys/deepseek_domestic_key").trim());
//! const DEEPSEEK_CHAT_URL: &'static str = "https://api.deepseek.com/chat/completions";
//! const DEEPSEEK_MODEL: &'static str = "deepseek-chat";
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let request = RequestBody {
//!         messages: vec![
//!             Message::System {
//!                 content: "You are a helpful assistant.".to_string(),
//!                 name: None,
//!             },
//!             Message::User {
//!                 content: "Hello, how are you?".to_string(),
//!                 name: None,
//!             },
//!         ],
//!         model: DEEPSEEK_MODEL.to_string(),
//!         stream: false,
//!         ..Default::default()
//!     };
//!
//!     // Send the request
//!     let response: String = request
//!         .get_response(DEEPSEEK_CHAT_URL, &*DEEPSEEK_API_KEY)
//!         .await?;
//!     let chat_completion = ChatCompletion::from_str(&response).unwrap();
//!     let text = chat_completion.choices[0]
//!         .message
//!         .content
//!         .as_deref()
//!         .unwrap();
//!     println!("{:?}", text);
//!     Ok(())
//! }
//! ```
//!
//! ## Streaming Chat Completion
//!
//! ```rust
//! use openai_interface::chat::response::streaming::{CompletionContent, ChatCompletionChunk};
//! use openai_interface::chat::request::{Message, RequestBody};
//! use futures_util::StreamExt;
//!
//! use std::str::FromStr;
//! use std::sync::LazyLock;
//!
//! // You need to provide your own DeepSeek API key at /keys/deepseek_domestic_key
//! const DEEPSEEK_API_KEY: LazyLock<&str> =
//!    LazyLock::new(|| include_str!("../keys/deepseek_domestic_key").trim());
//! const DEEPSEEK_CHAT_URL: &'static str = "https://api.deepseek.com/chat/completions";
//! const DEEPSEEK_MODEL: &'static str = "deepseek-chat";
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let request = RequestBody {
//!         messages: vec![
//!             Message::System {
//!                 content: "You are a helpful assistant.".to_string(),
//!                 name: None,
//!             },
//!             Message::User {
//!                 content: "Who are you?".to_string(),
//!                 name: None,
//!             },
//!         ],
//!         model: DEEPSEEK_MODEL.to_string(),
//!         stream: true,
//!         ..Default::default()
//!     };
//!
//!     // Send the request
//!     let mut response_stream = request
//!         .get_stream_response(DEEPSEEK_CHAT_URL, *DEEPSEEK_API_KEY)
//!         .await?;
//!
//!     let mut message = String::new();
//!
//!     while let Some(chunk_result) = response_stream.next().await {
//!         let chunk_string = chunk_result?;
//!         // let json_string = chunk_string.strip_prefix("data: ").unwrap();
//!         // if json_string == "[DONE]" {
//!         //     break;
//!         // }
//!         if &chunk_string == "[DONE]" {
//!             // SSE stream ends.
//!             break;
//!         }
//!         let chunk = ChatCompletionChunk::from_str(&chunk_string).unwrap();
//!         let content: &String = match chunk.choices[0].delta.content.as_ref().unwrap() {
//!             CompletionContent::Content(s) => s,
//!             CompletionContent::ReasoningContent(s) => s,
//!         };
//!         println!("lib::test_streaming message: {}", content);
//!         message.push_str(content);
//!     }
//!
//!     println!("lib::test_streaming message: {}", message);
//!     Ok(())
//! }
//! ```
//!
//! # Musl Build
//!
//! This crate is designed to adapt with the musl libc, making it suitable for
//! lightweight deployments in containerized environments. Longer compile times
//! may be required, for openssl is needed to be built from source.
//!
//! To build for musl:
//! ```bash
//! rustup target add x86_64-unknown-linux-musl
//! cargo build --target x86_64-unknown-linux-musl
//! ```

pub mod chat;
pub mod completions;
pub mod errors;
pub mod rest;

#[cfg(test)]
mod tests {
    use crate::chat::request::{Message, RequestBody};
    use crate::chat::response::no_streaming::ChatCompletion;
    use crate::chat::response::streaming::{ChatCompletionChunk, CompletionContent};
    use futures_util::StreamExt;
    use std::str::FromStr;
    use std::sync::LazyLock;

    // You need to provide your own DeepSeek API key at /keys/deepseek_domestic_key
    const DEEPSEEK_API_KEY: LazyLock<&str> =
        LazyLock::new(|| include_str!("../keys/deepseek_domestic_key").trim());
    const DEEPSEEK_CHAT_URL: &'static str = "https://api.deepseek.com/chat/completions";
    const DEEPSEEK_MODEL: &'static str = "deepseek-chat";

    #[tokio::test]
    async fn test_no_streaming() -> Result<(), Box<dyn std::error::Error>> {
        let request = RequestBody {
            messages: vec![
                Message::System {
                    content: "You are a helpful assistant.".to_string(),
                    name: None,
                },
                Message::User {
                    content: "Hello, how are you?".to_string(),
                    name: None,
                },
            ],
            model: DEEPSEEK_MODEL.to_string(),
            stream: false,
            ..Default::default()
        };

        // Send the request
        let response: String = request
            .get_response(DEEPSEEK_CHAT_URL, &*DEEPSEEK_API_KEY)
            .await?;
        let chat_completion = ChatCompletion::from_str(&response).unwrap();
        let text = chat_completion.choices[0]
            .message
            .content
            .as_deref()
            .unwrap();
        println!("lib::test_no_streaming message: {}", text);
        Ok(())
    }

    #[tokio::test]
    async fn test_streaming() -> Result<(), Box<dyn std::error::Error>> {
        let request = RequestBody {
            messages: vec![
                Message::System {
                    content: "You are a helpful assistant.".to_string(),
                    name: None,
                },
                Message::User {
                    content: "Who are you?".to_string(),
                    name: None,
                },
            ],
            model: DEEPSEEK_MODEL.to_string(),
            stream: true,
            ..Default::default()
        };

        // Send the request
        let mut response_stream = request
            .get_stream_response(DEEPSEEK_CHAT_URL, *DEEPSEEK_API_KEY)
            .await?;

        let mut message = String::new();

        while let Some(chunk_result) = response_stream.next().await {
            let chunk_string = chunk_result?;
            // let json_string = chunk_string.strip_prefix("data: ").unwrap();
            // if json_string == "[DONE]" {
            //     break;
            // }
            if &chunk_string == "[DONE]" {
                // SSE stream ends.
                break;
            }
            let chunk = ChatCompletionChunk::from_str(&chunk_string).unwrap();
            let content = match chunk.choices[0].delta.content.as_ref().unwrap() {
                CompletionContent::Content(s) => s,
                CompletionContent::ReasoningContent(s) => s,
            };
            println!("lib::test_streaming message: {}", content);
            message.push_str(content);
        }

        println!("lib::test_streaming message: {}", message);
        Ok(())
    }
}
