use std::collections::HashMap;

use garde::Validate;
use serde::Deserialize;

use crate::db::models::index_set::{dual::NewDualIndexSet, single::NewSingleIndexSet};

mod common;
mod dual;
mod single;

#[derive(Deserialize, Validate)]
#[serde(untagged)]
pub enum NewIndexSets {
    Single(#[garde(dive)] Vec<NewSingleIndexSet>),
    Dual(#[garde(dive)] HashMap<String, NewDualIndexSet>),
}
