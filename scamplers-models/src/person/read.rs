#[cfg(feature = "app")]
use diesel::prelude::*;
use macro_attributes::select;
#[cfg(feature = "app")]
use scamplers_schema::{institution, person};
use uuid::Uuid;

use crate::{institution::Institution, links::Links, person::common::Fields};

#[select]
#[cfg_attr(feature = "app", diesel(table_name = person))]
pub struct PersonSummary {
    id: Uuid,
    #[serde(flatten)]
    #[cfg_attr(feature = "app", diesel(embed))]
    inner: Fields,
    email: Option<String>,
    links: Links,
}

#[select]
#[cfg_attr(feature = "app", diesel(table_name = person, base_query = person::table.inner_join(institution::table)))]
pub struct Person {
    #[serde(flatten)]
    #[cfg_attr(feature = "app", diesel(embed))]
    summary: PersonSummary,
    #[cfg_attr(feature = "app", diesel(embed))]
    institution: Institution,
}
