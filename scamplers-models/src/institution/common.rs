use macro_attributes::insert_select;
use non_empty_string::NonEmptyString;
#[cfg(feature = "app")]
use scamplers_schema::institutions;
use uuid::Uuid;

#[insert_select]
#[cfg_attr(feature = "app", diesel(table_name = institutions))]
pub struct Fields {
    pub(super) id: Uuid,
    pub(super) name: NonEmptyString,
}
