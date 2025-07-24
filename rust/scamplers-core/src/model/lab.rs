#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{
    base_api_model, base_api_model_with_default, db_insertion, db_query, db_selection, db_update,
    to_json,
};
#[cfg(feature = "backend")]
use scamplers_schema::lab;
use uuid::Uuid;
use valid_string::ValidString;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::model::{Pagination, SortByGroup, person::PersonSummary};

#[to_json(python)]
#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = lab))]
pub struct NewLab {
    #[garde(dive)]
    pub name: ValidString,
    pub pi_id: Uuid,
    #[garde(dive)]
    pub delivery_dir: ValidString,
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    #[builder(default)]
    pub member_ids: Vec<Uuid>,
}

#[cfg(feature = "python")]
#[pymethods]
impl NewLab {
    #[new]
    fn new(
        name: ValidString,
        pi_id: Uuid,
        delivery_dir: ValidString,
        member_ids: Vec<Uuid>,
    ) -> Self {
        Self {
            name,
            pi_id,
            delivery_dir,
            member_ids,
        }
    }
}

#[to_json(python)]
#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = lab))]
pub struct LabHandle {
    pub id: Uuid,
    pub link: String,
}

#[to_json(python)]
#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = lab))]
pub struct LabSummary {
    #[serde(flatten)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub handle: LabHandle,
    pub name: String,
    pub delivery_dir: String,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = lab))]
pub struct LabCore {
    #[serde(flatten)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub summary: LabSummary,
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub pi: PersonSummary,
}

#[to_json(python)]
#[cfg_attr(
    target_arch = "wasm32",
    wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)
)]
#[cfg_attr(feature = "python", pyclass(get_all, str))]
#[base_api_model]
pub struct Lab {
    #[serde(flatten)]
    pub core: LabCore,
    pub members: Vec<PersonSummary>,
}

#[db_update]
#[cfg_attr(feature = "backend", diesel(table_name = lab))]
pub struct LabUpdateCore {
    pub id: Uuid,
    #[garde(dive)]
    pub name: Option<ValidString>,
    pub pi_id: Option<Uuid>,
    #[garde(dive)]
    pub delivery_dir: Option<ValidString>,
}

#[to_json(python)]
#[cfg_attr(feature = "python", pyclass(get_all, set_all, str))]
#[base_api_model_with_default]
pub struct LabUpdate {
    #[serde(flatten)]
    #[garde(dive)]
    pub core: LabUpdateCore,
    pub add_members: Vec<Uuid>,
    pub remove_members: Vec<Uuid>,
}

#[base_api_model_with_default]
pub enum LabOrdinalColumn {
    #[default]
    Name,
}

#[to_json(python)]
#[db_query]
pub struct LabQuery {
    pub ids: Vec<Uuid>,
    pub name: Option<String>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub order_by: SortByGroup<LabOrdinalColumn>,
    pub pagination: Pagination,
}
