use garde::Validate;
use serde::Deserialize;

use crate::db::models::index_set::common::{DNA_REGEX, INDEX_SET_NAME_REGEX};
#[cfg(feature = "app")]
mod create;

#[derive(Clone, Validate, Deserialize)]
pub struct NewSingleIndexSet(
    #[garde(pattern(INDEX_SET_NAME_REGEX))] String,
    #[garde(inner(pattern(DNA_REGEX)))] [String; 4],
);
