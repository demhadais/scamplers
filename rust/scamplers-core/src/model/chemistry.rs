use crate::string::NonEmptyString;
use serde_json::Value;
#[cfg(feature = "backend")]
use {scamplers_macros::backend_insertion, scamplers_schema::chemistry};

#[cfg_attr(feature = "backend", backend_insertion(chemistry))]
#[cfg_attr(feature = "backend", derive(Clone))]
pub struct Chemistry {
    name: NonEmptyString,
    description: NonEmptyString,
    #[cfg_attr(feature = "backend", serde(flatten), valuable(skip))]
    definition: Value,
    cmdline: NonEmptyString,
}
