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
//! use openai_interface::errors::OapiError;
//! use serde::{Serialize, Deserialize};
//!
//! use std::str::FromStr;
//!
//! #[derive(Serialize)]
//! struct MyRequest {
//!     prompt: String,
//!     stream: bool,
//! }
//!
//! #[derive(Deserialize)]
//! struct MyResponse {
//!     // Define the fields of your response here
//!     id: String,
//! }
//!
//! impl FromStr for MyResponse {
//!     type Err = OapiError;
//!
//!     fn from_str(content: &str) -> Result<Self, Self::Err> {
//!         let parse_result: Result<Self, _> = serde_json::from_str(content)
//!             .map_err(|e| OapiError::DeserializationError(e.to_string()));
//!         parse_result
//!     }
//! }
//!
//! impl openai_interface::rest::post::Post for MyRequest {
//!     fn is_streaming(&self) -> bool {
//!         self.stream
//!     }
//! }
//!
//! impl NoStream for MyRequest {
//!     type Response = MyResponse;
//! }
//! // or impl Stream for MyRequest {} for streaming requests
//! ```

pub mod post;
