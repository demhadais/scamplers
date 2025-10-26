use macro_attributes::base_model;

#[base_model]
#[derive(Default)]
#[serde(default)]
#[cfg_attr(feature = "schema", schemars(inline))]
pub struct OrderBy<O>
where
    O: Default,
{
    pub field: O,
    pub descending: bool,
}
