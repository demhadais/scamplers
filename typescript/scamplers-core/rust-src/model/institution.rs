#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{
    FromJson, ToJson, base_api_model_with_default, db_insertion, db_query, db_selection,
};
#[cfg(feature = "backend")]
use scamplers_schema::institution;
use uuid::Uuid;
use valid_string::ValidString;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::model::{Pagination, SortByGroup};

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = institution))]
#[derive(ToJson, FromJson)]
#[cfg_attr(not(target_arch = "wasm32"), json(python))]
pub struct NewInstitution {
    pub id: Uuid,
    #[garde(dive)]
    pub name: ValidString,
}

#[cfg(feature = "python")]
#[pymethods]
impl NewInstitution {
    #[new]
    #[pyo3(signature = (*, id, name))]
    fn new(id: Uuid, name: ValidString) -> Self {
        Self { id, name }
    }
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = institution))]
#[derive(ToJson, FromJson)]
#[cfg_attr(not(target_arch = "wasm32"), json(python))]
pub struct InstitutionHandle {
    pub id: Uuid,
    pub link: String,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = institution))]
#[derive(ToJson, FromJson)]
#[cfg_attr(not(target_arch = "wasm32"), json(python))]
pub struct Institution {
    #[serde(flatten)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub handle: InstitutionHandle,
    pub name: String,
}

#[base_api_model_with_default]
pub enum InstitutionOrdinalColumn {
    #[default]
    Name,
}

#[db_query]
#[derive(ToJson, FromJson)]
#[cfg_attr(not(target_arch = "wasm32"), json(python))]
pub struct InstitutionQuery {
    #[builder(default)]
    pub ids: Vec<Uuid>,
    pub name: Option<String>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub order_by: SortByGroup<InstitutionOrdinalColumn>,
    #[builder(default)]
    pub pagination: Pagination,
}
