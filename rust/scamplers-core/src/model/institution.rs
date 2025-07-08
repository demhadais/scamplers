use crate::{
    model::{Pagination, SortBy, SortByGroup},
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
    #[getset(get = "pub")]
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
impl InstitutionQueryBuilder {
    pub fn order_by(mut self, by: InstitutionOrdinalColumn, descending: bool) -> Self {
        let sort_by = SortBy { by, descending };

        if let Some(ref mut ordering) = self.order_by {
            ordering.push(sort_by);
        } else {
            self.order_by = Some(SortByGroup(vec![sort_by]));
        }

        self
    }
}
