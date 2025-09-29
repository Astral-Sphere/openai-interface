use std::str::FromStr;

use serde::Deserialize;

use crate::errors::OapiError;

#[derive(Debug, Deserialize, Clone)]
pub struct ListFilesResponse {
    pub data: Vec<crate::files::FileObject>,
    pub has_more: bool,
    pub object: Option<String>,
}

impl FromStr for ListFilesResponse {
    type Err = OapiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_result: Result<Self, _> =
            serde_json::from_str(s).map_err(|e| OapiError::DeserializationError(e.to_string()));
        parse_result
    }
}
