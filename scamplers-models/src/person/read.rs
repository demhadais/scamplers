#[cfg(feature = "app")]
use diesel::prelude::*;
use macro_attributes::{base_model, select};
use macros::uuid_newtype;
#[cfg(feature = "app")]
use scamplers_schema::{institutions, people};
use uuid::Uuid;

use crate::{institution::Institution, links::Links, person::common::Fields};

#[select]
#[cfg_attr(feature = "app", diesel(table_name = people))]
pub struct PersonSummary {
    id: Uuid,
    #[serde(flatten)]
    #[cfg_attr(feature = "app", diesel(embed))]
    inner: Fields,
    email: Option<String>,
    links: Links,
}

#[select]
#[cfg_attr(feature = "app", diesel(table_name = people, base_query = people::table.inner_join(institutions::table)))]
pub struct Person {
    #[serde(flatten)]
    #[cfg_attr(feature = "app", diesel(embed))]
    summary: PersonSummary,
    #[cfg_attr(feature = "app", diesel(embed))]
    institution: Institution,
}

#[base_model]
pub struct CreatedUser {
    #[serde(flatten)]
    pub inner: PersonSummary,
    pub api_key: String,
}

uuid_newtype!(PersonId);
