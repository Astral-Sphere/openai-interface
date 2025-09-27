pub mod request;
pub mod response;

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, sync::LazyLock};

    use crate::rest::post::NoStream;

    use super::*;

    const TEST_FILE_PATH: &str = "src/files/create/file-test.txt";
    const MODELSCOPE_URL: &str = "https://dashscope.aliyuncs.com/compatible-mode/v1/files";
    const MODELSCOPE_KEY: LazyLock<&str> =
        LazyLock::new(|| include_str!("../../../keys/modelstudio_domestic_key").trim());

    #[tokio::test]
    async fn test_file_create() -> Result<(), anyhow::Error> {
        let file_path = PathBuf::from(TEST_FILE_PATH);
        let create_file_request = request::CreateFileRequest {
            file: file_path,
            purpose: request::FilePurpose::Other("file-extract".to_string()),
            ..Default::default()
        };

        let _response = create_file_request
            .get_response(MODELSCOPE_URL, MODELSCOPE_KEY.as_ref())
            .await?;

        Ok(())
    }
}
