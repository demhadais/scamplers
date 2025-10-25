use macro_attributes::{base_model_default, query, query_newtype};
use uuid::Uuid;

use crate::generic_query::GenericQuery;

#[query]
#[derive(bon::Builder)]
pub struct Filter {
    #[builder(default)]
    pub ids: Vec<Uuid>,
    #[builder(default)]
    pub names: Vec<String>,
}

#[base_model_default]
pub enum OrdinalColumns {
    Id,
    #[default]
    Name,
}

#[query_newtype("InstitutionQuery")]
pub struct Query(pub GenericQuery<Filter, OrdinalColumns>);
