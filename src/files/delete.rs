//! Delete a file by file ID.

pub mod request {
    use url::Url;

    use crate::{
        errors::OapiError,
        rest::delete::{Delete, DeleteNoStream},
    };

    #[derive(Debug, Default)]
    pub struct DeleteRequest<'a> {
        pub file_id: &'a str,
        /// This parameter makes no difference yet.
        pub extra_headers: serde_json::Map<String, serde_json::Value>,
    }

    impl Delete for DeleteRequest<'_> {
        /// base_url should look like https://api.openai.com/v1/ (must ends with '/')
        fn build_url(&self, base_url: &str) -> Result<String, crate::errors::OapiError> {
            let url = Url::parse(base_url)
                .map_err(|e| OapiError::UrlError(e))?
                .join("files/")
                .unwrap()
                .join(self.file_id)
                .map_err(|e| OapiError::UrlError(e))?;

            println!("Built URL: {}", url);
            Ok(url.to_string())
        }
    }

    impl<'a> DeleteNoStream for DeleteRequest<'a> {
        type Response = super::response::FileDeleted;
    }
}

pub mod response {
    use std::str::FromStr;

    use serde::Deserialize;

    use crate::errors::OapiError;

    #[derive(Debug, Deserialize, Clone)]
    pub struct FileDeleted {
        pub id: String,
        pub deleted: bool,
        pub object: FileDeletedObject,
    }

    #[derive(Debug, Deserialize, Clone)]
    #[serde(rename_all = "snake_case")]
    pub enum FileDeletedObject {
        File,
    }

    impl FromStr for FileDeleted {
        type Err = crate::errors::OapiError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            serde_json::from_str(s).map_err(|e| OapiError::DeserializationError(e.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use anyhow::{Context, anyhow};

    use crate::{
        files::{delete::request::DeleteRequest, list::request::ListFilesRequest},
        rest::{delete::DeleteNoStream, get::GetNoStream},
    };

    const MODELSTUDIO_BASE_URL: &str = "https://dashscope.aliyuncs.com/compatible-mode/v1/";
    const MODELSTUDIO_KEY: LazyLock<&str> =
        LazyLock::new(|| include_str!("../../keys/modelstudio_domestic_key").trim());

    #[tokio::test]
    async fn test_modelstudio_delete_files() -> anyhow::Result<()> {
        let list_request = ListFilesRequest::default();
        let list_response = list_request
            .get_response(MODELSTUDIO_BASE_URL, &MODELSTUDIO_KEY)
            .await
            .with_context(|| anyhow!("Failed to list files."))?;

        let files = list_response
            .data
            .iter()
            .filter_map(|file_object| {
                let name = file_object.filename.as_str();
                let file_id = file_object.id.as_str();
                if name.starts_with("test") {
                    Some(file_id)
                } else {
                    None
                }
            })
            .collect::<Vec<&str>>();

        let futures = files
            .iter()
            .map(|file| async move {
                let delete_request = DeleteRequest {
                    file_id: file,
                    ..Default::default()
                };
                delete_request
                    .get_response(MODELSTUDIO_BASE_URL, &MODELSTUDIO_KEY)
                    .await
                    .with_context(|| anyhow!("Failed to delete file {}", file))
            })
            .collect::<Vec<_>>();

        let results = futures_util::future::join_all(futures).await;

        for result in results {
            result?;
        }

        Ok(())
    }
}
