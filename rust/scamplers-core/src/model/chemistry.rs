use crate::string::NonEmptyString;
use scamplers_macros::db_insertion;
#[cfg(feature = "backend")]
use scamplers_schema::chemistry;
use serde_json::Value;

#[db_insertion]
#[derive(Clone)]
#[cfg_attr(feature = "backend", diesel(table_name = chemistry))]
pub struct Chemistry {
    #[garde(dive)]
    name: NonEmptyString,
    #[garde(dive)]
    description: NonEmptyString,
    #[serde(flatten)]
    #[valuable(skip)]
    definition: Value,
    #[garde(dive)]
    cmdline: NonEmptyString,
}
