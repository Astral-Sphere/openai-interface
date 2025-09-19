# openai-interface

A low-level Rust interface for interacting with OpenAI's API. Both streaming
and non-streaming APIs are supported.

Currently only chat completion is supported. FIM completion, image generation,
etc. are still in development.

## Features

- **Chat Completions**: Full support for OpenAI's chat completion API
- **Streaming and Non-streaming**: Support for both streaming and non-streaming responses
- **Strong Typing**: Complete type definitions for all API requests and responses,
  utilizing Rust's powerful typing system
- **Error Handling**: All errors are converted into `crate::error::RequestError` or
  `crate::error::ResponseError`
- **Async/Await**: Built with async/await support
- **Musl Support**: Designed to work with musl libc out-of-the-box
- **Multiple Provider Support**: Works with OpenAI, DeepSeek, Qwen, and other compatible APIs

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
openai-interface = "0.2"
```

## Usage

### Chat Completion

#### Non-streaming Chat Completion

```rust
use std::sync::LazyLock;
use openai_interface::chat::request::{Message, RequestBody};
use openai_interface::chat::response::no_streaming::Completion;

// You need to provide your own DeepSeek API key at /keys/deepseek_domestic_key
const DEEPSEEK_API_KEY: LazyLock<&str> =
    LazyLock::new(|| include_str!("../keys/deepseek_domestic_key").trim());
const DEEPSEEK_CHAT_URL: &'static str = "https://api.deepseek.com/chat/completions";
const DEEPSEEK_MODEL: &'static str = "deepseek-chat";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    let response_text = request.get_response(DEEPSEEK_CHAT_URL, &*DEEPSEEK_API_KEY).await?;

    // Parse the response
    let completion = Completion::parse_string(&response_text)?;

    if let Some(choice) = completion.choices.first() {
        if let Some(content) = &choice.message.content {
            println!("Assistant: {}", content);
        }
    }

    Ok(())
}
```

#### Streaming Chat Completion

```rust
use openai_interface::chat::request::{Message, RequestBody};
use openai_interface::chat::response::streaming::Completion;
use futures_util::StreamExt;

use std::sync::LazyLock;

// You need to provide your own DeepSeek API key at /keys/deepseek_domestic_key
const DEEPSEEK_API_KEY: LazyLock<&str> =
    LazyLock::new(|| include_str!("../keys/deepseek_domestic_key").trim());
const DEEPSEEK_CHAT_URL: &'static str = "https://api.deepseek.com/chat/completions";
const DEEPSEEK_MODEL: &'static str = "deepseek-chat";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request = RequestBody {
        messages: vec![
            Message::System {
                content: "You are a helpful assistant.".to_string(),
                name: None,
            },
            Message::User {
                content: "Count from 1 to 10.".to_string(),
                name: None,
            },
        ],
        model: DEEPSEEK_MODEL.to_string(),
        stream: true,
        ..Default::default()
    };

    // Send the request and get a stream
    let mut response_stream = request.stream_response(DEEPSEEK_CHAT_URL, *DEEPSEEK_API_KEY).await?;

    while let Some(chunk) = response_stream.next().await {
        let chunk_text = chunk?;

        // Parse each streaming chunk
        if let Ok(completion) = Completion::parse_string(&chunk_text) {
            for choice in completion.choices {
                if let Some(content) = choice.delta.content {
                    match content {
                        openai_interface::chat::response::streaming::CompletionContent::Content(text) => {
                            print!("{}", text);
                        }
                        openai_interface::chat::response::streaming::CompletionContent::ReasoningContent(text) => {
                            print!("[Reasoning] {}", text);
                        }
                    }
                }
            }
        }
    }
    println!(); // New line after streaming completes
    Ok(())
}
```

#### Custom Request Parameters

You can customize whatever request parameters you want. If you need extra platform
-specific fields, find them in `extra_body`, or add them to `extra_body_map`.

### Getting Response

Parse the response content by `crate::chat::response::streaming::Completion::parse_string`
or `crate::chat::response::no_streaming::Completion::parse_string`

```rust
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

    let parsed = super::Completion::parse_string(json);
    match parsed {
        Ok(_) => {}
        Err(e) => {
            panic!("Failed to deserialize: {}", e);
        }
    }
}
```

### Error Handling

All errors are converted into `crate::error::RequestError` or
`crate::error::ResponseError`.

## Musl Build

This crate is designed to adapt with the musl libc, making it suitable for
lightweight deployments in containerized environments. Longer compile times
may be required, for openssl is needed to be built from source.

To build for musl:

```bash
rustup target add x86_64-unknown-linux-musl
cargo build --target x86_64-unknown-linux-musl
```

## Supported Providers

This crate aims to support standard OpenAI-compatible API endpoints. Sadly, the People's
Republic of China is blocked by the OpenAI website, and I can only refer to Manuals from
DeepSeek and Qwen. Please open an issue if there are any mistakes or inaccuracies in my
implementation.

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for bugs and feature requests.

## License

This project is licensed under the AGPL-3.0 License - see the LICENSE file for details.
