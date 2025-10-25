use default_vec::DefaultVec;
use macro_attributes::base_model;

#[base_model]
#[derive(Default)]
#[serde(default)]
#[cfg_attr(feature = "schema", schemars(inline))]
pub struct OrderBy<O>
where
    O: Default,
{
    field: O,
    descending: bool,
}

#[base_model]
#[derive(Default)]
#[serde(default)]
#[cfg_attr(feature = "schema", schemars(inline))]
pub struct GenericQuery<F, O>
where
    F: Default,
    O: Default,
{
    pub limit: u32,
    pub offset: u32,
    #[serde(flatten)]
    pub filter: F,
    pub order_by: DefaultVec<OrderBy<O>>,
}
