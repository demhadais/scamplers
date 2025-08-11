#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "python")]
use pyo3::prelude::*;

mod chemistry;
mod chromium_run;
mod dataset;
mod institution;
mod lab;
mod library_type_specification;
mod nucleic_acid;
mod person;
mod sequencing_run;
mod specimen;
mod suspension;
mod suspension_pool;

#[cfg(feature = "app")]
mod app_traits {}

#[cfg_attr(
    target_arch = "wasm32",
    wasm_bindgen(getter_with_clone, setter, js_name = "OrderBy")
)]
#[cfg_attr(feature = "python", pyclass(name = "OrderBy", get_all, set_all))]
pub struct WasmPythonOrderBy {
    pub field: String,
    pub descending: bool,
}
