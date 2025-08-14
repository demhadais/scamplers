use std::collections::HashMap;

#[cfg(feature = "app")]
use diesel::{
    backend::Backend,
    deserialize::{FromSql, FromSqlRow},
    pg::Pg,
    sql_types,
};
#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3_stub_gen::PyStubType;
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
pub mod multiplexing_tag;
pub mod nucleic_acid;
pub mod person;
pub mod sequencing_run;
pub mod specimen;
pub mod suspension;
pub mod units;

#[cfg_attr(feature = "python", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[cfg_attr(
    target_arch = "wasm32",
    wasm_bindgen(getter_with_clone, js_name = "OrderBy")
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

#[base_model]
#[serde(transparent)]
#[valuable(transparent)]
#[cfg_attr(feature = "app", derive(FromSqlRow))]
#[cfg_attr(feature = "python", derive(IntoPyObject, FromPyObject))]
#[cfg_attr(feature = "python", pyo3(transparent))]
pub struct Links(HashMap<String, String>);

impl<'a> IntoIterator for &'a Links {
    type IntoIter = <&'a HashMap<String, String> as IntoIterator>::IntoIter;
    type Item = <&'a HashMap<String, String> as IntoIterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[cfg(feature = "app")]
impl FromSql<sql_types::Jsonb, Pg> for Links {
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let json = <serde_json::Value as FromSql<sql_types::Jsonb, Pg>>::from_sql(bytes)?;
        Ok(Self(serde_json::from_value(json).unwrap()))
    }
}

#[cfg(target_arch = "wasm32")]
impl wasm_bindgen::describe::WasmDescribe for Links {
    fn describe() {
        js_sys::Map::describe();
    }
}

#[cfg(target_arch = "wasm32")]
impl wasm_bindgen::convert::IntoWasmAbi for Links {
    type Abi = <js_sys::Map as wasm_bindgen::convert::IntoWasmAbi>::Abi;

    fn into_abi(self) -> Self::Abi {
        let map = js_sys::Map::new();
        for (key, val) in &self {
            map.set(&JsValue::from_str(key), &JsValue::from_str(val));
        }

        map.into_abi()
    }
}

#[cfg(feature = "python")]
impl PyStubType for Links {
    fn type_output() -> pyo3_stub_gen::TypeInfo {
        HashMap::<String, String>::type_output()
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
