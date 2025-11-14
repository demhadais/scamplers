use macro_attributes::{ordinal_columns, query, schema_query};
use macros::uuid_newtype;
use uuid::Uuid;

use crate::generic_query::{self};

#[ordinal_columns]
pub enum OrdinalColumns {
    Id,
    #[default]
    Name,
}

#[query]
pub struct Filter {
    ids: Option<Vec<Uuid>>,
    name: Option<String>,
}

impl Filter {
    #[must_use]
    pub fn ids(&self) -> Option<&[Uuid]> {
        self.ids.as_deref()
    }

    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

pub type Query = generic_query::Query<Filter, OrdinalColumns>;

#[schema_query]
pub struct InstitutionQuery(Query);

uuid_newtype!(InstitutionId, "/institutions/{id}");
