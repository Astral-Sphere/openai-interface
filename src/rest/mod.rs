//! REST API client module for OpenAI interface
//!
//! This module provides the core HTTP functionality for making requests to OpenAI-compatible APIs.
//! It includes traits and implementations for both streaming and non-streaming API calls.
//!
//! # Overview
//!
//! The `rest` module contains:
//! - [`post`]: HTTP POST request functionality with streaming and non-streaming support
//! - [`get`]: HTTP GET request functionality with various parameter handling options
//! - Traits for defining API request behavior
//! - Error handling for HTTP communication
//!
//! # Usage
//!
//! The module is designed to be used through the higher-level API modules (`chat`, `completions`,
//! etc.). However, you can use the traits directly if needed:
//!
//! ## POST Requests
//!
//! ```rust
//! use openai_interface::rest::post::{Post, PostNoStream, PostStream};
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
//! impl Post for MyRequest {
//!     fn is_streaming(&self) -> bool {
//!         self.stream
//!     }
//! }
//!
//! impl PostNoStream for MyRequest {
//!     type Response = MyResponse;
//! }
//! // or impl PostStream for MyRequest {} for streaming requests
//! ```
//!
//! ## GET Requests
//!
//! ```rust
//! use openai_interface::rest::get::{Get, GetNoStream};
//! use openai_interface::errors::OapiError;
//! use serde::Deserialize;
//!
//! use std::str::FromStr;
//!
//! #[derive(Deserialize)]
//! struct MyResponse {
//!     id: String,
//!     name: String,
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
//! // GET request with URL building
//! struct ComplexRequest {
//!     resource_id: String,
//!     limit: Option<u32>,
//! }
//!
//! impl Get for ComplexRequest {
//!     fn build_url(&self, base_url: &str) -> Result<String, OapiError> {
//!         let mut url = format!("{}/{}", base_url, self.resource_id);
//!         if let Some(limit) = self.limit {
//!             url.push_str(&format!("?limit={}", limit));
//!         }
//!         Ok(url)
//!     }
//! }
//!
//! impl GetNoStream for ComplexRequest {
//!     type Response = MyResponse;
//! }
//! ```

pub mod get;
pub mod post;
