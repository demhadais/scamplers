#[cfg(feature = "app")]
use diesel::prelude::*;
use macro_attributes::select;
#[cfg(feature = "app")]
use scamplers_schema::{lab, person};
use uuid::Uuid;

use crate::{lab::common::Fields, links::Links, person::PersonSummary};

#[select]
#[cfg_attr(feature = "app", diesel(table_name = lab))]
pub struct LabSummary {
    id: Uuid,
    #[serde(flatten)]
    #[cfg_attr(feature = "app", diesel(embed))]
    inner: Fields,
    links: Links,
}

#[select]
#[cfg_attr(feature = "app", diesel(table_name = lab, base_query = lab::table.inner_join(person::table)))]
pub struct Lab {
    #[serde(flatten)]
    #[cfg_attr(feature = "app", diesel(embed))]
    summary: PersonSummary,
    #[cfg_attr(feature = "app", diesel(embed))]
    pi: PersonSummary,
}
