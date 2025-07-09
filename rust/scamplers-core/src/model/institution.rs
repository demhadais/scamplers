use crate::{
    model::{Pagination, SortByGroup},
    string::NonEmptyString,
};
use pyo3::{pyclass, pymethods};
use scamplers_macros::{base_api_model_with_default, db_insertion, db_query, db_selection};
#[cfg(feature = "backend")]
use scamplers_schema::institution;
use uuid::Uuid;
use wasm_bindgen::prelude::wasm_bindgen;

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = institution))]
pub struct NewInstitution {
    pub id: Uuid,
    #[garde(dive)]
    pub name: NonEmptyString,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = institution))]
pub struct InstitutionHandle {
    pub id: Uuid,
    pub link: String,
}

#[db_selection]
#[pyclass]
#[cfg_attr(feature = "backend", diesel(table_name = institution))]
pub struct Institution {
    #[serde(flatten)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub handle: InstitutionHandle,
    #[pyo3(get)]
    pub name: String,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[pymethods]
impl Institution {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    #[getter]
    pub fn id(&self) -> Uuid {
        self.handle.id
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    #[getter]
    pub fn link(&self) -> String {
        self.handle.link.to_string()
    }
}

#[base_api_model_with_default]
pub enum InstitutionOrdinalColumn {
    #[default]
    Name,
}

#[db_query]
pub struct InstitutionQuery {
    #[pyo3(get, set)]
    pub ids: Vec<Uuid>,
    #[pyo3(get, set)]
    pub name: Option<String>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub order_by: SortByGroup<InstitutionOrdinalColumn>,
    #[pyo3(get, set)]
    pub pagination: Pagination,
}
