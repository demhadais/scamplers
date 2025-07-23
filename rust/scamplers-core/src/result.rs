#[cfg(feature = "python")]
use pyo3::{exceptions::PyException, prelude::*};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[cfg_attr(feature = "python", pyclass(get_all, extends = PyException, str))]
#[cfg_attr(feature = "python", derive(IntoPyObject))]
#[derive(Clone, Deserialize, Serialize, Debug, thiserror::Error, valuable::Valuable)]
#[error("{self:?}")]
pub struct ClientError {
    pub message: String,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[cfg_attr(feature = "python", pyclass(get_all, extends = PyException, str))]
#[cfg_attr(feature = "python", derive(IntoPyObject))]
#[derive(Clone, Deserialize, Serialize, Debug, thiserror::Error, valuable::Valuable)]
#[error("{self:?}")]
pub struct DuplicateResourceError {
    pub entity: String,
    pub fields: Vec<String>,
    pub values: Vec<String>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[cfg_attr(feature = "python", pyclass(get_all, extends = PyException, str))]
#[cfg_attr(feature = "python", derive(IntoPyObject))]
#[derive(Clone, Deserialize, Serialize, Debug, thiserror::Error, valuable::Valuable)]
#[error("{self:?}")]
pub struct InvalidReferenceError {
    pub entity: String,
    pub referenced_entity: String,
    pub value: Option<String>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[cfg_attr(feature = "python", pyclass(get_all, extends = PyException, str))]
#[cfg_attr(feature = "python", derive(IntoPyObject))]
#[derive(Clone, Deserialize, Serialize, Debug, thiserror::Error, valuable::Valuable)]
#[error("{self:?}")]
pub struct ResourceNotFoundError {
    pub requested_resource_id: Uuid,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[cfg_attr(feature = "python", pyclass(get_all, extends = PyException, str))]
#[cfg_attr(feature = "python", derive(IntoPyObject))]
#[derive(Clone, Deserialize, Serialize, Debug, thiserror::Error, valuable::Valuable)]
#[error("{self:?}")]
pub struct InvalidDataError {
    pub message: String,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[cfg_attr(feature = "python", pyclass(get_all, extends = PyException, str))]
#[cfg_attr(feature = "python", derive(IntoPyObject))]
#[derive(Clone, Deserialize, Serialize, Debug, thiserror::Error, valuable::Valuable)]
#[error("{self:?}")]
pub struct MalformedRequestError {
    pub message: String,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[cfg_attr(feature = "python", pyclass(get_all, extends = PyException, str))]
#[cfg_attr(feature = "python", derive(IntoPyObject))]
#[derive(Clone, Deserialize, Serialize, Debug, thiserror::Error, valuable::Valuable)]
#[error("{self:?}")]
pub struct PermissionDeniedError {
    pub message: String,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[cfg_attr(feature = "python", pyclass(get_all, extends = PyException, str))]
#[cfg_attr(feature = "python", derive(IntoPyObject))]
#[derive(Clone, Deserialize, Serialize, Debug, thiserror::Error, valuable::Valuable)]
#[error("{self:?}")]
pub struct ServerError {
    pub message: String,
    pub raw_response_body: String,
}

#[cfg_attr(feature = "python", pyclass)]
#[derive(Clone, Deserialize, Serialize, Debug, thiserror::Error, valuable::Valuable)]
#[serde(tag = "type", rename_all = "snake_case")]
#[error(transparent)]
pub enum ScamplersCoreError {
    Client(ClientError),
    DuplicateResource(DuplicateResourceError),
    InvalidReference(InvalidReferenceError),
    ResourceNotFound(ResourceNotFoundError),
    InvalidData(InvalidDataError),
    Server(ServerError),
    MalformedRequest(MalformedRequestError),
    PermissionDenied(PermissionDeniedError),
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[cfg_attr(feature = "python", pyclass(get_all, str))]
#[derive(Deserialize, Serialize, Debug, thiserror::Error, valuable::Valuable)]
#[error("{self:?}")]
pub struct ScamplersErrorResponse {
    pub status: Option<u16>,
    #[source]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(readonly))]
    pub error: ScamplersCoreError,
}

#[cfg(target_arch = "wasm32")]
mod wasm32 {
    use super::ScamplersCoreError;
    use wasm_bindgen::{JsValue, convert::IntoWasmAbi, describe::WasmDescribe};

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
                Self::InvalidData(e) => e.into_abi(),
                Self::InvalidReference(e) => e.into_abi(),
                Self::MalformedRequest(e) => e.into_abi(),
                Self::PermissionDenied(e) => e.into_abi(),
                Self::ResourceNotFound(e) => e.into_abi(),
                Self::Server(e) => e.into_abi(),
            }
        }
    }
}

#[cfg(feature = "python")]
mod python {
    use super::{ScamplersErrorResponse, ScamplersCoreError};
    use pyo3::prelude::*;

    #[pymethods]
    impl ScamplersErrorResponse {
        #[new]
        fn new(error: ScamplersCoreError, status: Option<u16>) -> Self {
            Self { status, error }
        }
    }

    // trait IntoTuple {
    //     type Tuple: PyErrArguments + Send + Sync + 'static;
    //     fn into_tuple(self) -> Self::Tuple;
    // }

    // trait PyErrExt<Error>
    // where
    //     Error: IntoTuple + PyTypeInfo,
    //     Error::Tuple: PyErrArguments + Send + Sync + 'static,
    // {
    //     fn from_error(error: Error) -> Self;
    // }

    // impl<Error> PyErrExt<Error> for PyErr
    // where
    //     Error: IntoTuple + PyTypeInfo,
    //     Error::Tuple: PyErrArguments + Send + Sync + 'static,
    // {
    //     fn from_error(error: Error) -> Self {
    //         Self::new::<Error, _>(error.into_tuple())
    //     }
    // }

    // impl IntoTuple for ClientError {
    //     type Tuple = (String,);
    //     fn into_tuple(self) -> Self::Tuple {
    //         (self.message,)
    //     }
    // }

    // #[pymethods]
    // impl ClientError {
    //     #[new]
    //     fn new(message: String) -> Self {
    //         Self { message }
    //     }
    // }

    // impl IntoTuple for DuplicateResourceError {
    //     type Tuple = (String, Vec<String>, Vec<String>);
    //     fn into_tuple(self) -> Self::Tuple {
    //         (self.entity, self.fields, self.values)
    //     }
    // }

    // #[pymethods]
    // impl DuplicateResourceError {
    //     #[new]
    //     fn new(entity: String, fields: Vec<String>, values: Vec<String>) -> Self {
    //         Self {
    //             entity,
    //             fields,
    //             values,
    //         }
    //     }
    // }

    // impl IntoTuple for InvalidReferenceError {
    //     type Tuple = (String, String, Option<String>);
    //     fn into_tuple(self) -> Self::Tuple {
    //         (self.entity, self.referenced_entity, self.value)
    //     }
    // }

    // #[pymethods]
    // impl InvalidReferenceError {
    //     #[new]
    //     fn new(entity: String, referenced_entity: String, value: Option<String>) -> Self {
    //         Self {
    //             entity,
    //             referenced_entity,
    //             value,
    //         }
    //     }
    // }

    // impl IntoTuple for ResourceNotFoundError {
    //     type Tuple = (Uuid,);
    //     fn into_tuple(self) -> Self::Tuple {
    //         (self.requested_resource_id,)
    //     }
    // }

    // #[pymethods]
    // impl ResourceNotFoundError {
    //     #[new]
    //     fn new(requested_resource_id: Uuid) -> Self {
    //         Self {
    //             requested_resource_id,
    //         }
    //     }
    // }

    // impl IntoTuple for InvalidDataError {
    //     type Tuple = (String,);
    //     fn into_tuple(self) -> Self::Tuple {
    //         (self.message,)
    //     }
    // }

    // #[pymethods]
    // impl InvalidDataError {
    //     #[new]
    //     fn new(message: String) -> Self {
    //         Self { message }
    //     }
    // }

    // impl IntoTuple for PermissionDeniedError {
    //     type Tuple = (String,);
    //     fn into_tuple(self) -> Self::Tuple {
    //         (self.message,)
    //     }
    // }
    // #[pymethods]
    // impl PermissionDeniedError {
    //     #[new]
    //     fn new(message: String) -> Self {
    //         Self { message }
    //     }
    // }

    // impl IntoTuple for ServerError {
    //     type Tuple = (String, String);
    //     fn into_tuple(self) -> Self::Tuple {
    //         (self.message, self.raw_response_body)
    //     }
    // }

    // #[pymethods]
    // impl ServerError {
    //     #[new]
    //     fn new(message: String, raw_response_body: String) -> Self {
    //         Self {
    //             message,
    //             raw_response_body,
    //         }
    //     }
    // }

    impl From<ScamplersErrorResponse> for PyErr {
        fn from(ScamplersErrorResponse { status, error }: ScamplersErrorResponse) -> Self {
            Self::new::<ScamplersErrorResponse, _>((error, status))
        }
    }
}
