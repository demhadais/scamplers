use axum::{extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse};
#[cfg(feature = "python")]
use pyo3::{exceptions::PyException, prelude::*};
use scamplers_macros::scamplers_error;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use valuable::Valuable;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::db::models::{
    Jsonify,
    library_type_specification::{LibraryType, LibraryTypeSpecification},
};

#[scamplers_error]
#[error("{self:#?}")]
pub struct ClientError {
    pub message: String,
}

#[scamplers_error]
#[error("{self:#?}")]
pub struct DuplicateResourceError {
    pub entity: String,
    pub fields: Vec<String>,
    pub values: Vec<String>,
}

#[scamplers_error]
#[error("{self:#?}")]
pub struct InvalidReferenceError {
    pub entity: String,
    pub referenced_entity: String,
    pub value: Option<String>,
}

#[scamplers_error]
#[error("{self:#?}")]
pub struct ResourceNotFoundError {
    pub requested_resource_id: Uuid,
}

#[scamplers_error]
#[error("{self:#?}")]
pub struct InvalidDataError {
    pub message: String,
}

#[scamplers_error]
#[error("{self:#?}")]
pub struct MalformedRequestError {
    pub message: String,
}

#[scamplers_error]
#[error("{self:#?}")]
pub struct PermissionDeniedError {
    pub message: String,
}

#[scamplers_error]
#[error("{self:#?}")]
pub struct ServerError {
    pub message: String,
    #[builder(default)]
    pub raw_response_body: String,
}

#[scamplers_error]
#[error("{self:#?}")]
pub struct DatasetCmdlineError {
    pub chemistry: Option<String>,
    pub expected_cmdlines: Vec<String>,
    pub found_cmdline: String,
}

#[scamplers_error]
#[error("{self:#?}")]
pub struct DatasetNMetricsFilesError {
    pub expected_n_metrics_files: u64,
    pub found_n_metrics_files: u64,
}

#[scamplers_error]
#[error("{self:#?}")]
pub struct DatasetMetricsFileParseError {
    pub message: String,
}

#[scamplers_error]
#[error("{self:#?}")]
pub struct CdnaLibraryTypeError {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub expected_specifications: Vec<LibraryTypeSpecification>,
}

#[scamplers_error]
#[error("{self:#?}")]
pub struct CdnaGemsError {
    pub message: String,
}

#[scamplers_error]
#[error("{self:#?}")]
pub struct InvalidMeasurementError {
    pub message: String,
}

#[scamplers_error]
#[error("{self:#?}")]
pub struct LibraryIndexSetError {
    pub message: String,
}

#[cfg_attr(feature = "python", pyclass(get_all, str))]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, thiserror::Error, Valuable)]
#[serde(tag = "type", rename_all = "snake_case")]
#[error(transparent)]
pub enum ScamplersError {
    Client(#[from] ClientError),
    DuplicateResource(#[from] DuplicateResourceError),
    InvalidReference(#[from] InvalidReferenceError),
    ResourceNotFound(#[from] ResourceNotFoundError),
    InvalidData(#[from] InvalidDataError),
    Server(#[from] ServerError),
    MalformedRequest(#[from] MalformedRequestError),
    PermissionDenied(#[from] PermissionDeniedError),
    DatasetCmdline(#[from] DatasetCmdlineError),
    DatasetNMetricsFiles(#[from] DatasetNMetricsFilesError),
    DatasetMetricsFileParse(#[from] DatasetMetricsFileParseError),
    CdnaLibraryType(#[from] CdnaLibraryTypeError),
    CdnaGems(#[from] CdnaGemsError),
    InvalidMeasurement(#[from] InvalidMeasurementError),
    LibraryIndexSet(#[from] LibraryIndexSetError),
}

#[cfg_attr(feature = "python", pyclass(get_all, str, extends = PyException))]
#[::scamplers_macros::base_model]
#[cfg_attr(
    target_arch = "wasm32",
    ::wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)
)]
#[derive(
    ::scamplers_macros::Jsonify, ::scamplers_macros::WasmJsonify, ::thiserror::Error, ::bon::Builder,
)]
#[builder(on(_, into))]
#[error("status: {}\nerror: {error}", .status.unwrap_or_default())]
pub struct ScamplersErrorResponse {
    pub status: Option<u16>,
    #[source]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(readonly))]
    pub error: ScamplersError,
}

impl ScamplersErrorResponse {
    fn new(status: StatusCode, err: impl Into<ScamplersError>) -> Self {
        ScamplersErrorResponse::builder()
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

    pub fn new_unprocessable_entity_error(err: impl Into<ScamplersError>) -> Self {
        Self::new(StatusCode::UNPROCESSABLE_ENTITY, err)
    }
}

#[cfg(feature = "app")]
impl From<diesel::result::Error> for ScamplersError {
    fn from(err: diesel::result::Error) -> Self {
        use diesel::result::Error::{DatabaseError, NotFound};

        match err {
            DatabaseError(kind, info) => Self::from((kind, info)),
            NotFound => ResourceNotFoundError::builder()
                .requested_resource_id(Uuid::default())
                .build()
                .into(),
            err => ServerError {
                message: err.to_string(),
                ..Default::default()
            }
            .into(),
        }
    }
}

#[cfg(feature = "app")]
impl
    From<(
        diesel::result::DatabaseErrorKind,
        Box<dyn diesel::result::DatabaseErrorInformation + Send + Sync>,
    )> for ScamplersError
{
    fn from(
        (kind, info): (
            diesel::result::DatabaseErrorKind,
            Box<dyn diesel::result::DatabaseErrorInformation + Send + Sync>,
        ),
    ) -> Self {
        use diesel::result::DatabaseErrorKind::{ForeignKeyViolation, UniqueViolation};
        use regex::Regex;

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
            UniqueViolation => DuplicateResourceError {
                entity: entity.to_string(),
                fields,
                values,
            }
            .into(),

            ForeignKeyViolation => {
                let referenced_entity = details
                    .split_whitespace()
                    .last()
                    .unwrap_or_default()
                    .replace('"', "");
                let referenced_entity = referenced_entity.strip_suffix(".").unwrap_or_default();

                InvalidReferenceError {
                    entity: entity.to_string(),
                    referenced_entity: referenced_entity.to_string(),
                    value: values.first().cloned(),
                }
                .into()
            }
            _ => ServerError {
                message: diesel::result::Error::DatabaseError(kind, info).to_string(),
                ..Default::default()
            }
            .into(),
        }
    }
}

#[cfg(feature = "app")]
impl From<deadpool_diesel::PoolError> for ScamplersError {
    fn from(value: deadpool_diesel::PoolError) -> Self {
        ServerError::builder()
            .message(value.to_string())
            .build()
            .into()
    }
}

#[cfg(feature = "app")]
impl From<deadpool_diesel::InteractError> for ScamplersError {
    fn from(value: deadpool_diesel::InteractError) -> Self {
        ServerError::builder()
            .message(value.to_string())
            .build()
            .into()
    }
}

#[cfg(feature = "app")]
impl From<deadpool_diesel::InteractError> for ScamplersErrorResponse {
    fn from(value: deadpool_diesel::InteractError) -> Self {
        ScamplersError::from(value).into()
    }
}

impl From<JsonRejection> for ScamplersErrorResponse {
    fn from(err: JsonRejection) -> Self {
        let error = MalformedRequestError::builder()
            .message(err.body_text())
            .build();

        ScamplersErrorResponse::builder()
            .status(err.status())
            .error(error)
            .build()
    }
}

impl From<garde::Report> for ScamplersErrorResponse {
    fn from(err: garde::Report) -> Self {
        let error = InvalidDataError::builder()
            .message(format!("{err}"))
            .build();

        ScamplersErrorResponse::builder()
            .status(StatusCode::UNPROCESSABLE_ENTITY)
            .error(error)
            .build()
    }
}

impl IntoResponse for ScamplersErrorResponse {
    fn into_response(mut self) -> axum::response::Response {
        #[cfg(feature = "app")]
        tracing::error!(error = self.as_value());

        let status = self
            .status
            .and_then(|s| StatusCode::from_u16(s).ok())
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        if let Self {
            error: ScamplersError::Server(ServerError { message, .. }),
            ..
        } = &mut self
        {
            *message = "something went wrong".to_string();
        }

        (status, axum::Json(self)).into_response()
    }
}

impl From<ScamplersError> for ScamplersErrorResponse {
    fn from(mut err: ScamplersError) -> Self {
        use ScamplersError::*;
        let status = match &mut err {
            Client(_) => None,
            DuplicateResource(_) => Some(StatusCode::CONFLICT),
            ResourceNotFound(_) => Some(StatusCode::NOT_FOUND),
            MalformedRequest(_) => Some(StatusCode::BAD_REQUEST),
            PermissionDenied(_) => Some(StatusCode::UNAUTHORIZED),
            CdnaGems(_)
            | InvalidReference(_)
            | InvalidData(_)
            | DatasetCmdline(_)
            | DatasetNMetricsFiles(_)
            | DatasetMetricsFileParse(_)
            | CdnaLibraryType(_)
            | InvalidMeasurement(_)
            | LibraryIndexSet(_) => Some(StatusCode::UNPROCESSABLE_ENTITY),
            Server(e) => {
                e.message = "".to_string();
                Some(StatusCode::INTERNAL_SERVER_ERROR)
            }
        };

        Self::builder().maybe_status(status).error(err).build()
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm32 {
    use wasm_bindgen::{JsValue, convert::IntoWasmAbi, describe::WasmDescribe};

    use super::ScamplersError;

    impl WasmDescribe for ScamplersError {
        fn describe() {
            JsValue::describe();
        }
    }

    impl IntoWasmAbi for ScamplersError {
        type Abi = <JsValue as IntoWasmAbi>::Abi;

        fn into_abi(self) -> Self::Abi {
            match self {
                Self::Client(e) => e.into_abi(),
                Self::DuplicateResource(e) => e.into_abi(),
                Self::InvalidReference(e) => e.into_abi(),
                Self::ResourceNotFound(e) => e.into_abi(),
                Self::InvalidData(e) => e.into_abi(),
                Self::Server(e) => e.into_abi(),
                Self::MalformedRequest(e) => e.into_abi(),
                Self::PermissionDenied(e) => e.into_abi(),
                Self::DatasetCmdline(e) => e.into_abi(),
                Self::DatasetNMetricsFiles(e) => e.into_abi(),
                Self::DatasetMetricsFileParse(e) => e.into_abi(),
                Self::CdnaLibraryType(e) => e.into_abi(),
                Self::CdnaGems(e) => e.into_abi(),
                Self::InvalidMeasurement(e) => e.into_abi(),
                Self::LibraryIndexSet(e) => e.into_abi(),
            }
        }
    }
}

#[cfg(feature = "python")]
mod python {
    use pyo3::prelude::*;

    use super::{ScamplersError, ScamplersErrorResponse};

    #[pymethods]
    impl ScamplersErrorResponse {
        #[new]
        fn py_new(error: ScamplersError, status: Option<u16>) -> Self {
            Self { status, error }
        }
    }

    impl From<ScamplersErrorResponse> for PyErr {
        fn from(ScamplersErrorResponse { status, error }: ScamplersErrorResponse) -> Self {
            Self::new::<ScamplersErrorResponse, _>((error, status))
        }
    }
}

pub type ScamplersResult<T> = Result<T, ScamplersError>;
