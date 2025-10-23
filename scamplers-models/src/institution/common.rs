use macro_attributes::insert_select;
use non_empty_string::NonEmptyString;
use uuid::Uuid;

#[insert_select]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::institution))]
pub struct Fields {
    pub(super) id: Uuid,
    pub(super) name: NonEmptyString,
}
