use default_vec::DefaultVec;
use macro_attributes::{base_model_default, query};
use uuid::Uuid;

use crate::generic_order_by::OrderBy;

#[base_model_default]
#[derive(Copy)]
pub enum OrdinalColumns {
    Id,
    Email,
    #[default]
    Name,
}

#[query("PersonQuery")]
pub struct Query {
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
    #[builder(default)]
    pub limit: i64,
    #[builder(default)]
    pub offset: i64,
    #[builder(default)]
    pub order_by: DefaultVec<OrderBy<OrdinalColumns>>,
}
