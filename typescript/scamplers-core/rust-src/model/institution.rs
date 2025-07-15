use crate::model::{Pagination, SortByGroup};
use scamplers_macros::{
    base_api_model_with_default, db_insertion, db_query, db_selection, getters_impl,
};
#[cfg(feature = "backend")]
use scamplers_schema::institution;
use uuid::Uuid;
use valid_string::ValidString;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = institution))]
pub struct NewInstitution {
    pub id: Uuid,
    #[garde(dive)]
    pub name: ValidString,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = institution))]
pub struct InstitutionHandle {
    #[cfg_attr(feature = "python", pyo3(get))]
    pub id: Uuid,
    #[cfg_attr(feature = "python", pyo3(get))]
    pub link: String,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = institution))]
pub struct Institution {
    #[serde(flatten)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub handle: InstitutionHandle,
    #[cfg_attr(feature = "python", pyo3(get))]
    pub name: String,
}

#[getters_impl]
impl Institution {
    #[must_use]
    pub fn id(&self) -> Uuid {
        self.handle.id
    }

    #[must_use]
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
    #[builder(default)]
    #[cfg_attr(feature = "python", pyo3(get, set))]
    pub ids: Vec<Uuid>,
    #[cfg_attr(feature = "python", pyo3(get, set))]
    pub name: Option<String>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub order_by: SortByGroup<InstitutionOrdinalColumn>,
    #[builder(default)]
    #[cfg_attr(feature = "python", pyo3(get, set))]
    pub pagination: Pagination,
}
