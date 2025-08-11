#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::base_model;
use serde::{Serialize, de::DeserializeOwned};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub mod chemistry;
pub mod chromium_run;
pub mod dataset;
pub mod institution;
pub mod lab;
pub mod library_type_specification;
pub mod nucleic_acid;
pub mod person;
pub mod sequencing_run;
pub mod specimen;
pub mod suspension;
pub mod suspension_pool;

#[cfg_attr(feature = "python", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[cfg_attr(
    target_arch = "wasm32",
    wasm_bindgen(getter_with_clone, setter, js_name = "OrderBy")
)]
#[cfg_attr(feature = "python", pyclass(name = "OrderBy", get_all, set_all))]
#[derive(Clone)]
pub struct WasmPythonOrderBy {
    pub field: String,
    pub descending: bool,
}

#[cfg_attr(feature = "python", pyclass(get_all, set_all))]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[base_model]
pub struct Pagination {
    pub limit: i64,
    #[garde(range(min = 1))]
    pub offset: i64,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            limit: 500,
            offset: 0,
        }
    }
}

pub trait Jsonify: DeserializeOwned + Serialize {
    fn to_json_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }

    fn to_json_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn to_base64_json(&self) -> String {
        use ::base64::engine::Engine;

        base64::engine::general_purpose::URL_SAFE.encode(self.to_json_bytes())
    }

    fn from_json_bytes(json_bytes: &[u8]) -> anyhow::Result<Self> {
        Ok(serde_json::from_slice(json_bytes)?)
    }

    fn from_json_string(json_str: &str) -> anyhow::Result<Self> {
        Ok(serde_json::from_str(json_str)?)
    }

    fn from_base64_json(base64_json_bytes: &str) -> anyhow::Result<Self> {
        use base64::engine::Engine;

        let decoded = ::base64::engine::general_purpose::URL_SAFE.decode(base64_json_bytes)?;

        Ok(Self::from_json_bytes(&decoded)?)
    }
}

#[macro_export]
macro_rules! define_ordering_enum {
    ($name:ident; $($variant:ident),*; default = $default_variant:ident; $($enum_attribute:meta),*) => {
        #[derive(Clone, Debug, PartialEq, ::serde::Serialize, ::serde::Deserialize, ::strum::EnumString, ::strum::Display, ::valuable::Valuable)]
        #[serde(tag = "field", rename_all = "snake_case")]
        #[strum(serialize_all = "snake_case")]
        $(#[$enum_attribute])*
        pub enum $name {
            $(
                $variant {
                    descending: bool
                },
            )*
        }

        impl Default for $name {
            fn default() -> Self {
                Self::$default_variant {
                    descending: false
                }
            }
        }

        #[allow(dead_code)]
        impl $name {
            fn new(field: &str, descending: bool) -> Result<Self, ::strum::ParseError> {
                use std::str::FromStr;

                let mut orderby = Self::from_str(field)?;
                orderby.set_descending(descending);

                Ok(orderby)
            }

            fn set_descending(&mut self, descending: bool) {
                match self {
                    $( Self::$variant{ descending: current, ..} )|* => { *current = descending; }
                }
            }

            fn descending(&self) -> bool {
                match self {
                    $( Self::$variant{ descending, ..} )|* => *descending
                }
            }
        }

        #[cfg(target_arch = "wasm32")]
        impl From<crate::routes::WasmPythonOrderBy> for $name {
            fn from(wasm: crate::routes::WasmPythonOrderBy) -> Self {
                use wasm_bindgen::prelude::*;

                Self::new(&wasm.field, wasm.descending).unwrap_throw()
            }
        }

        #[cfg(feature = "python")]
        impl From<crate::routes::WasmPythonOrderBy> for $name {
            fn from(py: crate::routes::WasmPythonOrderBy) -> Self {
                // TODO: bad unwrap
                Self::new(&py.field, py.descending).unwrap()
            }
        }

        #[cfg(any(target_arch = "wasm32", feature = "python"))]
        impl From<$name> for crate::routes::WasmPythonOrderBy {
            fn from(order_by: $name) -> crate::routes::WasmPythonOrderBy {
                crate::routes::WasmPythonOrderBy {
                    field: order_by.to_string(),
                    descending: order_by.descending()
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_order_by {
    ($query_struct:ident) => {
        #[cfg(any(target_arch = "wasm32", feature = "python"))]
        impl $query_struct {
            fn order_by_inner(&self) -> Vec<crate::routes::WasmPythonOrderBy> {
                self.order_by.clone().into_iter().map(Into::into).collect()
            }

            fn set_order_inner(&mut self, orderings: Vec<crate::routes::WasmPythonOrderBy>) {
                self.order_by = orderings.into_iter().map(From::from).collect();
            }
        }
    };
}

#[macro_export]
macro_rules! impl_wasm_order_by {
    ($query_struct:ident) => {
        #[cfg(target_arch = "wasm32")]
        #[::wasm_bindgen::prelude::wasm_bindgen]
        impl $query_struct {
            #[wasm_bindgen(getter)]
            pub fn get_order_by(&self) -> Vec<crate::routes::WasmPythonOrderBy> {
                self.order_by_inner()
            }

            #[wasm_bindgen(setter)]
            pub fn set_order_by(&mut self, orderings: Vec<crate::routes::WasmPythonOrderBy>) {
                self.set_order_by_inner(orderings)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_python_order_by {
    ($query_struct:ident) => {
        #[cfg(feature = "python")]
        #[::pyo3_stub_gen::derive::gen_stub_pymethods]
        #[::pyo3::pymethods]
        impl $query_struct {
            #[getter]
            pub fn order_by(&self) -> Vec<crate::routes::WasmPythonOrderBy> {
                self.order_by()
            }

            #[setter]
            pub fn set_order_by(&mut self, orderings: Vec<crate::routes::WasmPythonOrderBy>) {
                self.set_order_by(orderings)
            }
        }
    };
}
