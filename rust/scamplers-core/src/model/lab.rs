use crate::{
    model::{IsUpdate, Pagination, SortByGroup, person::PersonSummary},
    string::NonEmptyString,
};
use scamplers_macros::{
    base_api_model, base_api_model_with_default, db_insertion, db_query, db_selection, db_update,
};
#[cfg(feature = "backend")]
use scamplers_schema::lab;
use uuid::Uuid;

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = lab))]
pub struct NewLab {
    #[garde(dive)]
    name: NonEmptyString,
    pi_id: Uuid,
    #[garde(dive)]
    delivery_dir: NonEmptyString,
    #[builder(default)]
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    member_ids: Vec<Uuid>,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = lab))]
pub struct LabHandle {
    id: Uuid,
    link: String,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = lab))]
pub struct LabSummary {
    #[serde(flatten)]
    #[getset(skip)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    handle: LabHandle,
    name: String,
    delivery_dir: String,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = lab))]
pub struct LabCore {
    #[serde(flatten)]
    #[getset(skip)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    summary: LabSummary,
    #[cfg_attr(feature = "backend", diesel(embed))]
    pi: PersonSummary,
}

#[base_api_model]
#[derive(::derive_builder::Builder)]
#[builder(pattern = "owned", build_fn(error = crate::model::BuilderError))]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Lab {
    #[serde(flatten)]
    #[getset(skip)]
    core: LabCore,
    members: Vec<PersonSummary>,
}

#[db_update]
#[cfg_attr(feature = "backend", diesel(table_name = lab))]
pub struct LabUpdateCore {
    id: Uuid,
    #[garde(dive)]
    name: Option<NonEmptyString>,
    pi_id: Option<Uuid>,
    #[garde(dive)]
    delivery_dir: Option<NonEmptyString>,
}
impl IsUpdate for LabUpdateCore {
    fn is_update(&self) -> bool {
        matches!(
            self,
            Self {
                name: Some(_),
                pi_id: Some(_),
                delivery_dir: Some(_),
                ..
            },
        )
    }
}

#[base_api_model_with_default]
#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
pub struct LabUpdate {
    #[serde(flatten)]
    #[garde(dive)]
    core: LabUpdateCore,
    add_members: Vec<Uuid>,
    remove_members: Vec<Uuid>,
}

#[base_api_model_with_default]
pub enum LabOrdinalColumn {
    #[default]
    Name,
}

#[db_query]
pub struct LabQuery {
    ids: Vec<Uuid>,
    name: Option<String>,
    #[builder(setter(custom))]
    order_by: SortByGroup<LabOrdinalColumn>,
    pagination: Pagination,
}
