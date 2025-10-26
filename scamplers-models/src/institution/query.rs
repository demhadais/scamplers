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
    #[builder(default)]
    pub ids: Vec<Uuid>,
    #[builder(default)]
    pub names: Vec<String>,
}

pub type Query = generic_query::Query<Filter, OrdinalColumns>;

#[schema_query]
pub struct InstitutionQuery(Query);

uuid_newtype!(InstitutionId, "/institutions/{id}");
