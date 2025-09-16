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
//! - [`chat`]: Contains all chat completion related functionality
//! - [`errors`]: Defines error types used throughout the crate
//!
//! # Examples
//!
//! ## Non-streaming Chat Completion
//!
//! ```rust
//! use std::sync::LazyLock;
//! use openai_interface::chat::request::{Message, RequestBody};
//!
//! // You need to provide your own DeepSeek API key at /keys/deepseek_domestic_key
//! const DEEPSEEK_API_KEY: LazyLock<&str> =
//!    LazyLock::new(|| include_str!("../keys/deepseek_domestic_key").trim());
//! const DEEPSEEK_CHAT_URL: &'static str = "https://api.deepseek.com/chat/completions";
//! const DEEPSEEK_MODEL: &'static str = "deepseek-chat";
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let request = RequestBody {
//!     messages: vec![
//!         Message::System {
//!             content: "You are a helpful assistant.".to_string(),
//!             name: None,
//!         },
//!         Message::User {
//!             content: "Hello, how are you?".to_string(),
//!             name: None,
//!         },
//!     ],
//!     model: DEEPSEEK_MODEL.to_string(),
//!     stream: false,
//!     ..Default::default()
//! };
//!
//! // Send the request
//! let response = request.get_response(DEEPSEEK_CHAT_URL, &*DEEPSEEK_API_KEY).await?;
//! println!("{:?}", response);
//! Ok(())
//! }
//! ```
//!
//! ## Streaming Chat Completion
//!
//! ```rust
//! use openai_interface::chat::request::{Message, RequestBody};
//! use futures_util::StreamExt;
//!
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
//!                 content: "Count from 1 to 10.".to_string(),
//!                 name: None,
//!             },
//!         ],
//!         model: DEEPSEEK_MODEL.to_string(),
//!         stream: true,
//!         ..Default::default()
//!     };
//!
//!     // Send the request
//!     let mut response_stream = request.stream_response(DEEPSEEK_CHAT_URL, *DEEPSEEK_API_KEY).await?;
//!
//!     while let Some(chunk) = response_stream.next().await {
//!         println!("{}", chunk?);
//!     }
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
pub mod errors;
