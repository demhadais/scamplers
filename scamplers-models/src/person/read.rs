#[cfg(feature = "app")]
use diesel::prelude::*;
use macro_attributes::{base_model, select};
#[cfg(feature = "app")]
use scamplers_schema::{institutions, people};
use uuid::Uuid;

use crate::{
    institution::Institution,
    links::Links,
    person::{UserRole, common::Fields},
};

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
pub struct PersonSummaryWithParents {
    #[serde(flatten)]
    #[cfg_attr(feature = "app", diesel(embed))]
    summary: PersonSummary,
    #[cfg_attr(feature = "app", diesel(embed))]
    institution: Institution,
}

#[base_model]
#[derive(bon::Builder)]
pub struct Person {
    #[serde(flatten)]
    info: PersonSummaryWithParents,
    roles: Vec<UserRole>,
}

#[base_model]
#[derive(bon::Builder)]
#[builder(on(_, into))]
pub struct CreatedUser {
    #[serde(flatten)]
    inner: Person,
    api_key: String,
}
