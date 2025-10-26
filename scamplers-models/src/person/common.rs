use macro_attributes::{insert_select, simple_enum};
use macros::{impl_enum_from_sql, impl_enum_to_sql};
use non_empty_string::NonEmptyString;
#[cfg(feature = "app")]
use scamplers_schema::people;
use uuid::Uuid;

#[cfg(feature = "app")]
use crate::utils::{EnumFromSql, EnumToSql};

#[simple_enum]
pub enum UserRole {
    AppAdmin,
    BiologyStaff,
    ComputationalStaff,
}

#[cfg(feature = "app")]
impl EnumFromSql for UserRole {}
impl_enum_from_sql!(UserRole);

#[cfg(feature = "app")]
impl EnumToSql for UserRole {}
impl_enum_to_sql!(UserRole);

#[insert_select]
#[cfg_attr(feature = "app", diesel(table_name = people))]
pub struct Fields {
    pub name: NonEmptyString,
    pub orcid: Option<NonEmptyString>,
    pub institution_id: Uuid,
    pub ms_user_id: Option<Uuid>,
}
