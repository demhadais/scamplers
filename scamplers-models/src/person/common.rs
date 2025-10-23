use macro_attributes::insert_select;
use non_empty_string::NonEmptyString;
use uuid::Uuid;

#[insert_select]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::person))]
pub struct Fields {
    pub(super) name: NonEmptyString,
    pub(super) orcid: Option<NonEmptyString>,
    pub(super) institution_id: Uuid,
    pub(super) ms_user_id: Option<Uuid>,
}
