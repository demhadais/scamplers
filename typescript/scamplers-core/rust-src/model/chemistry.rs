use crate::string::ValidString;
use serde_json::Value;
#[cfg(feature = "backend")]
use {scamplers_macros::backend_insertion, scamplers_schema::chemistry};

#[cfg_attr(feature = "backend", backend_insertion(chemistry))]
#[cfg_attr(feature = "backend", derive(Clone))]
pub struct Chemistry {
    #[cfg_attr(feature = "backend", garde(dive))]
    name: ValidString,
    #[cfg_attr(feature = "backend", garde(dive))]
    description: ValidString,
    #[cfg_attr(feature = "backend", serde(flatten), valuable(skip))]
    definition: Value,
    #[cfg_attr(feature = "backend", garde(dive))]
    cmdline: ValidString,
}
