use scamplers_macros::base_model;
use serde_json::Value;
use valid_string::ValidString;

#[cfg(feature = "app")]
mod create;

#[base_model]
#[cfg_attr(feature = "app", derive(::diesel::Insertable))]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::chemistry))]
pub struct Chemistry {
    #[garde(dive)]
    pub name: ValidString,
    #[garde(dive)]
    pub description: ValidString,
    #[serde(flatten)]
    #[valuable(skip)]
    pub definition: Value,
    #[garde(dive)]
    pub cmdlines: Vec<Option<ValidString>>,
}
