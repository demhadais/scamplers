use axum::{
    extract::rejection::{JsonRejection, PathRejection},
    http::StatusCode,
    response::IntoResponse,
};
use diesel::result::DatabaseErrorInformation;
use diesel_async::pooled_connection::deadpool;
use regex::Regex;
use scamplers_core::result::{
    DuplicateResourceError, InvalidDataError, InvalidReferenceError, MalformedRequestError,
    ResourceNotFoundError, ScamplersCoreError, ScamplersCoreErrorResponse, ServerError,
};
use serde::Serialize;
use uuid::Uuid;
use valuable::Valuable;

#[derive(Serialize, Debug, thiserror::Error, Valuable)]
#[serde(transparent)]
#[error(transparent)]
pub struct ScamplersError(#[from] ScamplersCoreErrorResponse);

impl ScamplersError {
    fn new(status: StatusCode, err: impl Into<ScamplersCoreError>) -> Self {
        ScamplersCoreErrorResponse::builder()
            .status(status)
            .error(err)
            .build()
            .into()
    }

    #[must_use]
    pub fn new_server_error(message: &str) -> Self {
        Self::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::builder()
                .raw_response_body("")
                .message(message)
                .build(),
        )
    }

    pub fn new_unprocessable_entity_error(err: impl Into<ScamplersCoreError>) -> Self {
        Self::new(StatusCode::UNPROCESSABLE_ENTITY, err)
    }

    #[must_use]
    pub fn inner(&self) -> &ScamplersCoreError {
        &self.0.error
    }
}

impl From<diesel::result::Error> for ScamplersError {
    fn from(err: diesel::result::Error) -> Self {
        use diesel::result::Error::{DatabaseError, NotFound};

        match err {
            DatabaseError(kind, info) => Self::from((kind, info)),
            NotFound => {
                let inner = ResourceNotFoundError::builder()
                    .requested_resource_id(Uuid::default())
                    .build();

                ScamplersCoreErrorResponse::builder()
                    .status(StatusCode::NOT_FOUND)
                    .error(inner)
                    .build()
                    .into()
            }
            err => ScamplersError::new_server_error(&err.to_string()),
        }
    }
}
impl
    From<(
        diesel::result::DatabaseErrorKind,
        Box<dyn DatabaseErrorInformation + Send + Sync>,
    )> for ScamplersError
{
    fn from(
        (kind, info): (
            diesel::result::DatabaseErrorKind,
            Box<dyn DatabaseErrorInformation + Send + Sync>,
        ),
    ) -> Self {
        use diesel::result::DatabaseErrorKind::{ForeignKeyViolation, UniqueViolation};
        let entity = info.table_name().unwrap_or_default();

        let detail_regex = Regex::new(r"Key \((.+)\)=\((.+)\).+").unwrap(); // This isn't perfect
        let details = info.details().unwrap_or_default();
        let field_value: Vec<String> = detail_regex
            .captures(details)
            .and_then(|cap| {
                cap.iter()
                    .take(3)
                    .map(|m| m.map(|s| s.as_str().to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let into_split_vecs = |v: &[String], i: usize| {
            v.get(i)
                .cloned()
                .unwrap_or_default()
                .split(", ")
                .map(str::to_string)
                .collect()
        };
        let fields = into_split_vecs(&field_value, 1);
        let values = into_split_vecs(&field_value, 2);

        match kind {
            UniqueViolation => {
                let err = DuplicateResourceError {
                    entity: entity.to_string(),
                    fields,
                    values,
                };

                ScamplersCoreErrorResponse::builder()
                    .status(StatusCode::CONFLICT)
                    .error(err)
                    .build()
                    .into()
            }

            ForeignKeyViolation => {
                let referenced_entity = details
                    .split_whitespace()
                    .last()
                    .unwrap_or_default()
                    .replace('"', "");
                let referenced_entity = referenced_entity.strip_suffix(".").unwrap_or_default();

                let err = InvalidReferenceError {
                    entity: entity.to_string(),
                    referenced_entity: referenced_entity.to_string(),
                    value: values.first().cloned(),
                };

                ScamplersCoreErrorResponse::builder()
                    .status(StatusCode::UNPROCESSABLE_ENTITY)
                    .error(err)
                    .build()
                    .into()
            }
            _ => ScamplersError::new_server_error(
                &diesel::result::Error::DatabaseError(kind, info).to_string(),
            ),
        }
    }
}

impl From<JsonRejection> for ScamplersError {
    fn from(err: JsonRejection) -> Self {
        let error = MalformedRequestError::builder()
            .message(err.body_text())
            .build();

        ScamplersCoreErrorResponse::builder()
            .status(err.status())
            .error(error)
            .build()
            .into()
    }
}

impl From<PathRejection> for ScamplersError {
    fn from(err: PathRejection) -> Self {
        let error = MalformedRequestError::builder()
            .message(err.body_text())
            .build();

        ScamplersCoreErrorResponse::builder()
            .status(err.status())
            .error(error)
            .build()
            .into()
    }
}

impl From<deadpool::PoolError> for ScamplersError {
    fn from(err: deadpool::PoolError) -> Self {
        Self::new_server_error(&format!("Database connection error: {err}"))
    }
}

impl From<garde::Report> for ScamplersError {
    fn from(err: garde::Report) -> Self {
        let error = InvalidDataError::builder()
            .message(format!("{err}"))
            .build();

        ScamplersCoreErrorResponse::builder()
            .status(StatusCode::UNPROCESSABLE_ENTITY)
            .error(error)
            .build()
            .into()
    }
}

impl IntoResponse for ScamplersError {
    fn into_response(mut self) -> axum::response::Response {
        tracing::error!(error = self.as_value());

        let status = self
            .0
            .status
            .and_then(|s| StatusCode::from_u16(s).ok())
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        if let Self(ScamplersCoreErrorResponse {
            error: ScamplersCoreError::Server(ServerError { message, .. }),
            ..
        }) = &mut self
        {
            *message = "something went wrong".to_string();
        }

        (status, axum::Json(self)).into_response()
    }
}

pub type ScamplersResult<T> = std::result::Result<T, ScamplersError>;
