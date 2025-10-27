use axum::{Json, response::IntoResponse};

use crate::db;

#[derive(Debug, thiserror::Error, serde::Serialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "cause")]
#[error(transparent)]
pub enum Error {
    #[error("{message}")]
    Unauthorized {
        message: String,
    },
    Database(#[from] db::Error),
}

impl Error {
    pub fn no_api_key() -> Self {
        Self::Unauthorized {
            message: "no API key".to_string(),
        }
    }

    pub fn invalid_api_key() -> Self {
        Self::Unauthorized {
            message: "invalid API key".to_string(),
        }
    }

    pub fn no_ui_auth_token() -> Self {
        Self::Unauthorized {
            message: "no UI authentication token".to_string(),
        }
    }

    pub fn invalid_ui_auth_token() -> Self {
        Self::Unauthorized {
            message: "invalid UI authentication token".to_string(),
        }
    }
}

impl From<deadpool_diesel::InteractError> for Error {
    fn from(err: deadpool_diesel::InteractError) -> Self {
        Self::Database(err.into())
    }
}

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        Self::Database(err.into())
    }
}
