#[cfg(feature = "python")]
use pyo3::{exceptions::PyException, prelude::*};
use scamplers_macros::{scamplers_error, to_from_json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::model::library_type_specification::LibraryType;

#[scamplers_error]
pub struct ClientError {
    pub message: String,
}

#[scamplers_error]
pub struct DuplicateResourceError {
    pub entity: String,
    pub fields: Vec<String>,
    pub values: Vec<String>,
}

#[scamplers_error]
pub struct InvalidReferenceError {
    pub entity: String,
    pub referenced_entity: String,
    pub value: Option<String>,
}

#[scamplers_error]
pub struct ResourceNotFoundError {
    pub requested_resource_id: Uuid,
}

#[scamplers_error]
pub struct InvalidDataError {
    pub message: String,
}

#[scamplers_error]
pub struct MalformedRequestError {
    pub message: String,
}

#[scamplers_error]
pub struct PermissionDeniedError {
    pub message: String,
}

#[scamplers_error]
pub struct ServerError {
    pub message: String,
    pub raw_response_body: String,
}

#[scamplers_error]
pub struct DatasetCmdlineError {
    pub chemistry: Option<String>,
    pub expected_cmdline: String,
    pub found_cmdline: String,
}

#[scamplers_error]
pub struct DatasetNMetricsFilesError {
    pub expected_n_metrics_files: u64,
    pub found_n_metrics_files: u64,
}

#[scamplers_error]
pub struct DatasetMetricsFileParseError {
    pub message: String,
}

#[scamplers_error]
pub struct CdnaLibraryTypeError {
    pub expected_library_types: Vec<LibraryType>,
    pub found_library_types: Vec<LibraryType>,
}

#[scamplers_error]
pub struct CdnaGemsError {
    pub message: String,
}

#[scamplers_error]
pub struct InvalidMeasurementError {
    pub message: String,
}

#[to_from_json(python)]
#[cfg_attr(feature = "python", pyclass(get_all, name = "ScamplersError", str))]
#[derive(Clone, Deserialize, Serialize, Debug, thiserror::Error, valuable::Valuable)]
#[serde(tag = "type", rename_all = "snake_case")]
#[error(transparent)]
pub enum ScamplersCoreError {
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
}

#[scamplers_error]
#[cfg_attr(feature = "python", pyo3(extends = PyException, name = "ScamplersApiErrorResponse"))]
pub struct ScamplersCoreErrorResponse {
    pub status: Option<u16>,
    #[source]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(readonly))]
    pub error: ScamplersCoreError,
}

#[cfg(target_arch = "wasm32")]
mod wasm32 {
    use wasm_bindgen::{JsValue, convert::IntoWasmAbi, describe::WasmDescribe};

    use super::ScamplersCoreError;

    impl WasmDescribe for ScamplersCoreError {
        fn describe() {
            JsValue::describe();
        }
    }

    impl IntoWasmAbi for ScamplersCoreError {
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
            }
        }
    }
}

#[cfg(feature = "python")]
mod python {
    use pyo3::prelude::*;

    use super::{ScamplersCoreError, ScamplersCoreErrorResponse};

    #[pymethods]
    impl ScamplersCoreErrorResponse {
        #[new]
        fn new(error: ScamplersCoreError, status: Option<u16>) -> Self {
            Self { status, error }
        }
    }

    impl From<ScamplersCoreErrorResponse> for PyErr {
        fn from(ScamplersCoreErrorResponse { status, error }: ScamplersCoreErrorResponse) -> Self {
            Self::new::<ScamplersCoreErrorResponse, _>((error, status))
        }
    }
}
