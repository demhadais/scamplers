use macro_attributes::{base_model_default, query, schema_query};
use uuid::Uuid;

use crate::generic_query::{self};

#[base_model_default]
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
pub struct LabQuery(Query);
