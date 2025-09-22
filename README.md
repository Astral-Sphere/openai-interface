# openai-interface

A low-level Rust interface for interacting with OpenAI's API. Both streaming
and non-streaming APIs are supported.

Currently only chat completion is supported. FIM completion, image generation,
etc. are still in development.

> Repository:
>
> [GitCode Repo](https://github.com/Astral-Sphere/openai-interface)  
> [GitHub Repo](https://github.com/Astral-Sphere/openai-interface)
>
> You are welcomed to contribute to this project through any one of the links.

## Features

- **Chat Completions**: Full support for OpenAI's chat completion API, including both streaming and non-streaming responses
- **Streaming and Non-streaming**: Support for both streaming and non-streaming responses
- **Strong Typing**: Complete type definitions for all API requests and responses,
  utilizing Rust's powerful typing system
- **Error Handling**: Comprehensive error handling with detailed error types defined in the [`errors`] module
- **Async/Await**: Built with async/await support
- **Musl Support**: Designed to work with musl libc out-of-the-box
- **Multiple Provider Support**: Works with OpenAI, DeepSeek, Qwen, and other compatible APIs

## Installation

> [!WARNING] Versions prior to 0.3.0 has serious issues on processing SSE streaming responses.
> Instead of a single chunk, chances are that multiple chunks will be returned in each iteration
> of `chat::request::ChatCompletion::get_streaming_response`.

Add this to your `Cargo.toml`:

```toml
[dependencies]
openai-interface = "0.3"
```

## Usage

### Chat Completion

This crate provides methods for both streaming and non-streaming chat completions. The following examples demonstrate how to use these features.

#### Non-streaming Chat Completion

```rust
use std::sync::LazyLock;
use openai_interface::chat::request::{Message, RequestBody};
use openai_interface::chat::response::no_streaming::ChatCompletion;
use std::str::FromStr;

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
    let response: String = request
        .get_response(DEEPSEEK_CHAT_URL, &*DEEPSEEK_API_KEY)
        .await?;
    let chat_completion = ChatCompletion::from_str(&response).unwrap();
    let text = chat_completion.choices[0]
        .message
        .content
        .as_deref()
        .unwrap();
    println!("{:?}", text);
    Ok(())
}
```

#### Streaming Chat Completion

This example demonstrates how to handle streaming responses from the API.

```rust
use openai_interface::chat::response::streaming::{CompletionContent, ChatCompletionChunk};
use openai_interface::chat::request::{Message, RequestBody};
use futures_util::StreamExt;

use std::str::FromStr;
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
        if &chunk_string == "[DONE]" {
            // SSE stream ends.
            break;
        }
        let chunk = ChatCompletionChunk::from_str(&chunk_string).unwrap();
        let content: &String = match chunk.choices[0].delta.content.as_ref().unwrap() {
            CompletionContent::Content(s) => s,
            CompletionContent::ReasoningContent(s) => s,
        };
        println!("lib::test_streaming message: {}", content);
        message.push_str(content);
    }

    println!("lib::test_streaming message: {}", message);
    Ok(())
}
```

#### Custom Request Parameters

You can customize whatever request parameters you want. If you need extra platform
-specific fields, find them in `extra_body`, or add them to `extra_body_map`.

### Modules

- [`chat`]: Contains all chat completion related structs, enums, and methods.
- [`errors`]: Defines error types used throughout the crate.

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
