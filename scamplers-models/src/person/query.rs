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
    #[builder(default)]
    pub emails: Vec<String>,
    #[builder(default)]
    pub orcids: Vec<String>,
    #[builder(default)]
    pub ms_user_ids: Vec<Uuid>,
}

#[base_model_default]
pub enum OrdinalColumns {
    Id,
    Email,
    #[default]
    Name,
}

#[query_newtype("PersonQuery")]
pub struct Query(pub GenericQuery<Filter, OrdinalColumns>);
