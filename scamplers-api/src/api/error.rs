use axum::{
    Json,
    extract::rejection::{JsonRejection, PathRejection},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{db, validate};

#[derive(Debug, thiserror::Error, serde::Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
#[error(transparent)]
pub enum Error {
    ValidateData(#[from] validate::Error),
    Database(#[from] db::Error),
    #[error("{message}")]
    MalformedRequest {
        message: String,
    },
    #[error("{message}")]
    UnprocessableEntity {
        message: String,
    },
    #[error("something went wrong")]
    Other,
}

impl From<deadpool_diesel::InteractError> for Error {
    fn from(err: deadpool_diesel::InteractError) -> Self {
        Self::Database(err.into())
    }
}

#[derive(Debug, thiserror::Error, serde::Serialize)]
#[error("{error}")]
pub struct ErrorResponse {
    pub status: u16,
    pub error: Error,
}

impl From<JsonRejection> for ErrorResponse {
    fn from(err: JsonRejection) -> Self {
        Self {
            status: err.status().as_u16(),
            error: Error::UnprocessableEntity {
                message: err.body_text(),
            },
        }
    }
}

impl From<PathRejection> for ErrorResponse {
    fn from(err: PathRejection) -> Self {
        Self {
            status: err.status().as_u16(),
            error: Error::UnprocessableEntity {
                message: err.body_text(),
            },
        }
    }
}

impl From<deadpool_diesel::InteractError> for ErrorResponse {
    fn from(err: deadpool_diesel::InteractError) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            error: Error::from(err),
        }
    }
}

impl From<db::Error> for ErrorResponse {
    fn from(err: db::Error) -> Self {
        use db::Error::*;
        let status = match err {
            DuplicateResource { .. } => StatusCode::CONFLICT,
            Data { .. } | InvalidReference { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            ResourceNotFound { .. } => StatusCode::NOT_FOUND,
            Other { .. } => {
                return {
                    Self {
                        status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        error: Error::Other,
                    }
                };
            }
        };

        Self {
            status: status.as_u16(),
            error: Error::Database(err),
        }
    }
}

impl From<validate::Error> for ErrorResponse {
    fn from(err: validate::Error) -> Self {
        Self {
            status: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
            error: err.into(),
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::from_u16(self.status).unwrap(), Json(self)).into_response()
    }
}
