#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::base_model;
use serde::{Serialize, de::DeserializeOwned};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub mod chemistry;
pub mod chromium_run;
pub mod dataset;
pub mod index_set;
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
