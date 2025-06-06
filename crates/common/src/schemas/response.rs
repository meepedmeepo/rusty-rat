use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Response<T: Serialize> {
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}

impl<T: Serialize> Response<T> {
    pub fn ok(data: T) -> Response<T> {
        Response {
            data: Some(data),
            error: None,
        }
    }

    pub fn err(err: Error) -> Response<T> {
        Response {
            data: None,
            error: Some(err.into()),
        }
    }

    pub fn from_anyhow_err(err: anyhow::Error) -> Response<T> {
        Response {
            data: None,
            error: Some(Error::from_error(err)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, String>>,
}

impl Error {
    pub fn from_error(err: anyhow::Error) -> Self {
        Self {
            message: err.to_string(),
            extensions: None,
        }
    }
}
