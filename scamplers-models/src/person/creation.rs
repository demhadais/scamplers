use bon::bon;
use macro_attributes::insert;
use non_empty_string::NonEmptyString;
use uuid::Uuid;

use crate::person::common::Fields;

#[insert]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::person))]
pub struct Creation {
    #[serde(flatten)]
    #[cfg_attr(feature = "app", diesel(embed))]
    inner: Fields,
    email: NonEmptyString,
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
    ) -> Self {
        Self {
            inner: Fields {
                name,
                orcid,
                institution_id,
                ms_user_id,
            },
            email,
        }
    }
}
