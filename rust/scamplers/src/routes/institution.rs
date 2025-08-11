use diesel::prelude::*;
use macro_rules_attribute::apply;
use macro_rules_attribute::derive;
use scamplers_schema::institution;
use uuid::Uuid;
use valid_string::ValidString;

#[derive(crate::macros::ApiModel!, Insertable, garde::Validate)]
#[diesel(table_name = institution)]
#[garde(allow_unvalidated)]
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
