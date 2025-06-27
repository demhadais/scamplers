use crate::{
    model::{Pagination, SortByGroup},
    string::NonEmptyString,
};
use scamplers_macros::{
    base_api_model_with_default, db_insertion_with_wasm, db_query, db_selection,
};
#[cfg(feature = "backend")]
use scamplers_schema::institution;
use uuid::Uuid;

#[db_insertion_with_wasm]
#[cfg_attr(feature = "backend", diesel(table_name = institution))]
pub struct NewInstitution {
    id: Uuid,
    #[garde(dive)]
    name: NonEmptyString,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = institution))]
pub struct InstitutionHandle {
    id: Uuid,
    link: String,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = institution))]
pub struct Institution {
    #[serde(flatten)]
    #[getset(skip)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    handle: InstitutionHandle,
    name: String,
}
impl Institution {
    pub fn id(&self) -> &Uuid {
        self.handle.id()
    }

    pub fn link(&self) -> &str {
        self.handle.link()
    }
}

#[base_api_model_with_default]
pub enum InstitutionOrdinalColumn {
    #[default]
    Name,
}

#[db_query]
pub struct InstitutionQuery {
    ids: Vec<Uuid>,
    name: Option<String>,
    #[builder(setter(custom))]
    order_by: SortByGroup<InstitutionOrdinalColumn>,
    pagination: Pagination,
}
