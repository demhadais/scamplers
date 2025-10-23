use default_vec::DefaultVec;
use macro_attributes::query;
use macros::define_ordering_enum;
#[cfg(feature = "app")]
use scamplers_schema::institution::name;
use uuid::Uuid;

define_ordering_enum!(InstitutionOrderBy { name }, default = name);

#[query]
#[derive(bon::Builder)]
pub struct Query {
    #[builder(default)]
    ids: Vec<Uuid>,
    #[builder(default)]
    names: Vec<String>,
    #[builder(default)]
    order_by: DefaultVec<InstitutionOrderBy>,
}
