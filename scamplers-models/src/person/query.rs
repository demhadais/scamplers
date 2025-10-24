use macro_attributes::{query, query_newtype};
use macros::define_ordering_enum;
#[cfg(feature = "app")]
use scamplers_schema::person::columns::{email, id, name};
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

define_ordering_enum! { OrderBy { Id(id), Name(name), Email(email) }, default = Name(name) }

#[query_newtype("PersonQuery")]
pub struct Query(pub GenericQuery<Filter, OrderBy>);
