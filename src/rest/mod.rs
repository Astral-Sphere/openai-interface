//! REST API client module for OpenAI interface
//!
//! This module provides the core HTTP functionality for making requests to OpenAI-compatible APIs.
//! It includes traits and implementations for both streaming and non-streaming API calls.
//!
//! # Overview
//!
//! The `rest` module contains:
//! - [`post`]: HTTP POST request functionality with streaming and non-streaming support
//! - Traits for defining API request behavior
//! - Error handling for HTTP communication
//!
//! # Usage
//!
//! The module is designed to be used through the higher-level API modules (`chat`, `completions`,
//! etc.). However, you can use the traits directly if needed:
//!
//! ```rust
//! use openai_interface::rest::post::{NoStream, Stream};
//! use serde::Serialize;
//!
//! #[derive(Serialize)]
//! struct MyRequest {
//!     prompt: String,
//!     stream: bool,
//! }
//!
//! impl openai_interface::rest::post::Post for MyRequest {
//!     fn is_streaming(&self) -> bool {
//!         self.stream
//!     }
//! }
//!
//! impl NoStream for MyRequest {}
//! // or impl Stream for MyRequest {} for streaming requests
//! ```

pub mod post;
