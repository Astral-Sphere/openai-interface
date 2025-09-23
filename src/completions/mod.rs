//! Given a prompt, the model will return one or more predicted completions,
//! and can also return the probabilities of alternative tokens at each position.
//! Compared to the `chat` API, this one does not provide the ability to have
//! multiple rounds of conversation. This API is getting deprecated in favor of the
//! `chat` API.

pub mod request;
pub mod response;

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use futures_util::StreamExt;

    use crate::rest::post::{NoStream, Stream};

    use super::*;

    const QWEN_MODEL: &str = "qwen-coder-turbo-latest";
    const QWEN_URL: &str = "https://dashscope.aliyuncs.com/compatible-mode/v1/completions";
    const QWEN_API_KEY: LazyLock<&'static str> =
        LazyLock::new(|| include_str!("../../keys/modelstudio_domestic_key").trim());

    const REQUEST_BODY: LazyLock<request::CompletionRequest> =
        LazyLock::new(|| request::CompletionRequest {
            model: QWEN_MODEL.to_string(),
            prompt: request::Prompt::PromptString(
                "\"桂棹兮兰桨，击空明兮溯流光\" 出自哪里？".to_string(),
            ),
            max_tokens: Some(500),
            stream: false,
            ..Default::default()
        });

    #[tokio::test]
    async fn test_qwen_completion_no_stream() -> Result<(), anyhow::Error> {
        let request_body = &REQUEST_BODY;
        let completion = request_body.get_response(QWEN_URL, *QWEN_API_KEY).await?;
        let text = &completion.choices[0].text;
        println!("Completion no-stream: {}", text);
        Ok(())
    }

    #[tokio::test]
    async fn test_qwen_completion_stream() -> Result<(), anyhow::Error> {
        let mut request_body = REQUEST_BODY.clone();
        request_body.stream = true;

        let mut stream = request_body
            .get_stream_response(QWEN_URL, *QWEN_API_KEY)
            .await?;

        while let Some(chunk) = stream.next().await {
            match chunk {
                Ok(completion) => {
                    println!(
                        "Completion stream chunk: {}; finish reason: {:?}",
                        completion.choices[0].text, completion.choices[0].finish_reason
                    );
                }
                Err(e) => {
                    eprintln!("Error receiving chunk: {:?}", e);
                    break;
                }
            }
        }

        Ok(())
    }
}
