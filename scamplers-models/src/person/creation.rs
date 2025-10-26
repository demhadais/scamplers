use bon::bon;
use macro_attributes::insert;
use non_empty_string::NonEmptyString;
#[cfg(feature = "app")]
use scamplers_schema::people;
use uuid::Uuid;

use crate::person::common::{Fields, UserRole};

#[insert]
#[cfg_attr(feature = "app", diesel(table_name = people))]
#[cfg_attr(feature = "schema", schemars(title = "PersonCreation"))]
pub struct Creation {
    #[serde(flatten)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub inner: Fields,
    pub email: NonEmptyString,
    #[serde(default)]
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
    pub roles: Vec<UserRole>,
}

#[bon]
impl Creation {
    #[builder]
    pub fn new(
        name: NonEmptyString,
        email: NonEmptyString,
        orcid: Option<NonEmptyString>,
        institution_id: Uuid,
        ms_user_id: Option<Uuid>,
        roles: Vec<UserRole>,
    ) -> Self {
        Self {
            inner: Fields {
                name,
                orcid,
                institution_id,
                ms_user_id,
            },
            roles,
            email,
        }
    }
}
