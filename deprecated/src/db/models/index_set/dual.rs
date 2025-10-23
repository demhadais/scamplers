use garde::Validate;
use serde::Deserialize;

use crate::db::models::index_set::common::DNA_REGEX;

#[cfg(feature = "app")]
mod create;

#[derive(Deserialize, Validate)]
pub struct NewDualIndexSet {
    #[garde(pattern(DNA_REGEX))]
    #[serde(alias = "index(i7)")]
    index_i7: String,
    #[garde(pattern(DNA_REGEX))]
    #[serde(alias = "index2_workflow_a(i5)")]
    index2_workflow_a_i5: String,
    #[garde(pattern(DNA_REGEX))]
    #[serde(alias = "index2_workflow_b(i5)")]
    index2_workflow_b_i5: String,
}
