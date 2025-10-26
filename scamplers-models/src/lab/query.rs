use default_vec::DefaultVec;
use macro_attributes::{base_model_default, query};
use uuid::Uuid;

use crate::generic_order_by::OrderBy;

#[base_model_default]
pub enum OrdinalColumns {
    Id,
    #[default]
    Name,
}

#[query("LabQuery")]
pub struct Query {
    #[builder(default)]
    pub ids: Vec<Uuid>,
    #[builder(default)]
    pub names: Vec<String>,
    #[builder(default)]
    pub limit: i64,
    #[builder(default)]
    pub offset: i64,
    #[builder(default)]
    pub order_by: DefaultVec<OrderBy<OrdinalColumns>>,
}
