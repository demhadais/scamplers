use macro_attributes::insert_select;
use non_empty_string::NonEmptyString;
use uuid::Uuid;

#[insert_select]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::lab))]
pub struct Fields {
    pub(super) name: NonEmptyString,
    pub(super) pi_id: Uuid,
    pub(super) delivery_dir: NonEmptyString,
}
