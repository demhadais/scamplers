use axum::{Json, response::IntoResponse};

use crate::db;

#[derive(Debug, thiserror::Error, serde::Serialize)]
#[error(transparent)]
pub enum Error {
    #[error("{message}")]
    Unauthorized {
        message: String,
    },
    Database(#[from] db::Error),
}

impl Error {
    pub fn invalid_api_key() -> Self {
        Self::Unauthorized {
            message: "invalid API key".to_string(),
        }
    }

    pub fn invalid_frontend_token() -> Self {
        Self::Unauthorized {
            message: "invalid frontend token".to_string(),
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

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match &self {
            Self::Unauthorized { .. } => {
                (axum::http::StatusCode::UNAUTHORIZED, Json(self)).into_response()
            }
            Self::Database(..) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    serde_json::json!({"status": 500, "error": {"message": "something went wrong"}}),
                ),
                ).into_response(),
        }
    }
}
