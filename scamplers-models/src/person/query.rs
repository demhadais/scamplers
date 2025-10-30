use macro_attributes::{ordinal_columns, query, schema_query};
use macros::uuid_newtype;
use uuid::Uuid;

use crate::generic_query::{self};

#[ordinal_columns]
pub enum OrdinalColumns {
    Id,
    Email,
    #[default]
    Name,
}

#[query]
pub struct Filter {
    #[builder(default)]
    pub ids: Vec<Uuid>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub orcid: Option<String>,
    #[builder(default)]
    pub microsoft_entra_oids: Vec<Uuid>,
}

pub type Query = generic_query::Query<Filter, OrdinalColumns>;

#[schema_query]
pub struct PersonQuery(Query);

uuid_newtype!(PersonId, "/people/{id}");
