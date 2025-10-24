use macro_attributes::{query, query_newtype};
use macros::define_ordering_enum;
#[cfg(feature = "app")]
use scamplers_schema::institution::{id, name};
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

define_ordering_enum!(OrderBy { Id(id), Name(name) }, default = Name(name));

#[query_newtype("InstitutionQuery")]
pub struct Query(pub GenericQuery<Filter, OrderBy>);
