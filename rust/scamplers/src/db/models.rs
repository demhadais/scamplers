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
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use valuable::Valuable;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub mod chromium_run;
pub mod dataset;
pub mod index_set;
pub mod institution;
pub mod lab;
pub mod multiplexing_tag;
pub mod nucleic_acid;
pub mod person;
pub mod sequencing_run;
pub mod specimen;
pub mod suspension;
pub mod tenx_assay;
pub mod units;

#[cfg_attr(
    target_arch = "wasm32",
    wasm_bindgen(getter_with_clone, js_name = OrderBy)
)]
#[cfg_attr(feature = "python", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[cfg_attr(
    feature = "python",
    pyclass(name = "OrderBy", get_all, set_all, module = "scamplepy.query")
)]
#[derive(Clone)]
pub struct WasmPythonOrderBy {
    pub field: String,
    pub descending: bool,
}

#[cfg(any(target_arch = "wasm32", feature = "python"))]
impl WasmPythonOrderBy {
    fn new(field: String, descending: bool) -> Self {
        Self { field, descending }
    }
}

#[cfg(feature = "python")]
#[pyo3_stub_gen::derive::gen_stub_pymethods]
#[pymethods]
impl WasmPythonOrderBy {
    #[new]
    fn py_new(field: String, descending: bool) -> Self {
        Self::new(field, descending)
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_class = OrderBy)]
impl WasmPythonOrderBy {
    #[wasm_bindgen(constructor)]
    #[must_use]
    pub fn js_new(field: String, descending: bool) -> Self {
        Self::new(field, descending)
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg_attr(feature = "python", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[cfg_attr(
    feature = "python",
    pyclass(get_all, set_all, module = "scamplepy.query")
)]
#[base_model]
pub struct Pagination {
    #[serde(default = "Pagination::default_limit")]
    pub limit: i64,
    #[serde(default = "Pagination::default_offset")]
    #[garde(range(min = 1))]
    pub offset: i64,
}

impl Pagination {
    #[must_use]
    pub const fn default_limit() -> i64 {
        500
    }

    #[must_use]
    pub const fn default_offset() -> i64 {
        0
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            limit: Self::default_limit(),
            offset: Self::default_offset(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Valuable)]
#[serde(transparent)]
#[valuable(transparent)]
pub struct DefaultVec<O>(Vec<O>)
where
    O: Valuable;

impl<O> Default for DefaultVec<O>
where
    O: Default,
    O: Valuable,
{
    fn default() -> Self {
        Self(vec![O::default()])
    }
}

impl<T, const N: usize> From<[T; N]> for DefaultVec<T>
where
    T: Valuable + Clone,
{
    fn from(value: [T; N]) -> Self {
        Self(value.to_vec())
    }
}

impl<O> IntoIterator for DefaultVec<O>
where
    O: Valuable,
{
    type IntoIter = <Vec<O> as IntoIterator>::IntoIter;
    type Item = <Vec<O> as IntoIterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[allow(clippy::into_iter_without_iter)]
impl<'a, O> IntoIterator for &'a DefaultVec<O>
where
    O: Valuable,
{
    type IntoIter = <&'a Vec<O> as IntoIterator>::IntoIter;
    type Item = <&'a Vec<O> as IntoIterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<O> From<O> for DefaultVec<O>
where
    O: Valuable,
{
    fn from(value: O) -> Self {
        Self(vec![value])
    }
}

impl<O> From<Vec<WasmPythonOrderBy>> for DefaultVec<O>
where
    O: From<WasmPythonOrderBy>,
    O: Valuable,
{
    fn from(value: Vec<WasmPythonOrderBy>) -> Self {
        Self(value.into_iter().map(O::from).collect())
    }
}

impl<O> From<DefaultVec<O>> for Vec<WasmPythonOrderBy>
where
    WasmPythonOrderBy: From<O>,
    O: Valuable,
{
    fn from(value: DefaultVec<O>) -> Self {
        value.into_iter().map(WasmPythonOrderBy::from).collect()
    }
}

#[cfg(target_arch = "wasm32")]
impl<O> wasm_bindgen::describe::WasmDescribe for DefaultVec<O>
where
    O: Valuable,
{
    fn describe() {
        Vec::<WasmPythonOrderBy>::describe();
    }
}

#[cfg(target_arch = "wasm32")]
impl<O> wasm_bindgen::convert::IntoWasmAbi for DefaultVec<O>
where
    O: Valuable,
    WasmPythonOrderBy: From<O>,
{
    type Abi = <Vec<WasmPythonOrderBy> as wasm_bindgen::convert::IntoWasmAbi>::Abi;

    fn into_abi(self) -> Self::Abi {
        let as_wasm: Vec<WasmPythonOrderBy> = self.into();
        as_wasm.into_abi()
    }
}

#[cfg(target_arch = "wasm32")]
impl<O> wasm_bindgen::convert::FromWasmAbi for DefaultVec<O>
where
    O: Valuable + From<WasmPythonOrderBy>,
{
    type Abi = <Vec<WasmPythonOrderBy> as wasm_bindgen::convert::IntoWasmAbi>::Abi;

    unsafe fn from_abi(js: Self::Abi) -> Self {
        unsafe { Vec::<WasmPythonOrderBy>::from_abi(js).into() }
    }
}

#[cfg(feature = "python")]
impl<O> PyStubType for DefaultVec<O>
where
    O: Valuable,
{
    fn type_output() -> pyo3_stub_gen::TypeInfo {
        <Vec<WasmPythonOrderBy> as PyStubType>::type_output()
    }
}

#[cfg(feature = "python")]
impl<O> FromPyObject<'_> for DefaultVec<O>
where
    O: From<WasmPythonOrderBy>,
    O: Valuable,
{
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(<Vec<WasmPythonOrderBy> as FromPyObject>::extract_bound(ob)?.into())
    }
}

#[cfg(feature = "python")]
impl<'py, O> IntoPyObject<'py> for DefaultVec<O>
where
    WasmPythonOrderBy: From<O>,
    O: Valuable,
{
    type Error = <Vec<WasmPythonOrderBy> as IntoPyObject<'py>>::Error;
    type Output = <Vec<WasmPythonOrderBy> as IntoPyObject<'py>>::Output;
    type Target = <Vec<WasmPythonOrderBy> as IntoPyObject<'py>>::Target;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let as_py: Vec<WasmPythonOrderBy> = self.into();
        as_py.into_pyobject(py)
    }
}

#[base_model]
#[serde(transparent)]
#[valuable(transparent)]
#[cfg_attr(feature = "app", derive(FromSqlRow))]
#[cfg_attr(feature = "python", derive(IntoPyObject, FromPyObject))]
#[cfg_attr(feature = "python", pyo3(transparent))]
pub struct Links(HashMap<String, String>);

#[allow(clippy::into_iter_without_iter)]
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

        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(self.to_json_bytes())
    }

    /// # Errors
    fn from_json_bytes(json_bytes: &[u8]) -> anyhow::Result<Self> {
        Ok(serde_json::from_slice(json_bytes)?)
    }

    /// # Errors
    fn from_json_string(json_str: &str) -> anyhow::Result<Self> {
        Ok(serde_json::from_str(json_str)?)
    }

    /// # Errors
    fn from_base64_json(base64_json_bytes: &str) -> anyhow::Result<Self> {
        use base64::engine::Engine;

        let decoded = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(base64_json_bytes)
            .or_else(|_| base64::engine::general_purpose::URL_SAFE.decode(base64_json_bytes))?;

        Self::from_json_bytes(&decoded)
    }
}

impl Jsonify for () {}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::{fixture, rstest};
    use serde::{Deserialize, Serialize};
    use time::OffsetDateTime;
    use uuid::Uuid;

    use crate::db::models::Jsonify;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Data {
        field1: String,
        field2: Option<String>,
        field3: Uuid,
        field4: OffsetDateTime,
        field5: Vec<Option<i32>>,
    }

    impl Jsonify for Data {}

    #[fixture]
    fn data() -> Data {
        Data {
            field1: "foo".to_string(),
            field2: Some("bar".to_string()),
            field3: Uuid::now_v7(),
            field4: OffsetDateTime::now_utc(),
            field5: vec![Some(42), None, None, Some(42)],
        }
    }

    #[rstest]
    fn jsonify_str_round_trip(data: Data) {
        let json = data.to_json_string();
        let deserialized = Data::from_json_string(&json).unwrap();
        assert_eq!(data, deserialized);
    }

    #[rstest]
    fn jsonify_bytes_round_trip(data: Data) {
        let json = data.to_json_bytes();
        let deserialized = Data::from_json_bytes(&json).unwrap();
        assert_eq!(data, deserialized);
    }

    #[rstest]
    fn jsonify_base64(data: Data) {
        let json = data.to_base64_json();
        let deserialized = Data::from_base64_json(&json).unwrap();
        assert_eq!(data, deserialized);
    }
}
