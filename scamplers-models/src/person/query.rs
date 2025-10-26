use macro_attributes::{base_model_default, query, schema_query};
use macros::uuid_newtype;
use uuid::Uuid;

use crate::generic_query::{self};

#[base_model_default]
#[derive(Copy)]
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
    #[builder(default)]
    pub names: Vec<String>,
    #[builder(default)]
    pub emails: Vec<String>,
    #[builder(default)]
    pub orcids: Vec<String>,
    #[builder(default)]
    pub ms_user_ids: Vec<Uuid>,
}

pub type Query = generic_query::Query<Filter, OrdinalColumns>;

#[schema_query]
pub struct PersonQuery(Query);

uuid_newtype!(PersonId, "/people/{id}");
