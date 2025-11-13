use bon::bon;
use macro_attributes::insert;
use non_empty_string::NonEmptyString;
#[cfg(feature = "app")]
use scamplers_schema::institutions;
use uuid::Uuid;

use crate::institution::common::Fields;

#[insert]
#[cfg_attr(feature = "app", derive(diesel::AsChangeset))]
#[cfg_attr(feature = "app", diesel(table_name = institutions))]
#[cfg_attr(feature = "schema", schemars(title = "InstitutionCreation"))]
pub struct Creation {
    #[serde(flatten)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub inner: Fields,
}

#[bon]
impl Creation {
    #[builder(on(_, into))]
    pub fn new(id: Uuid, name: NonEmptyString) -> Self {
        Self {
            inner: Fields { id, name },
        }
    }
}
