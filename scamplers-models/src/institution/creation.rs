#[cfg(feature = "builder")]
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
    inner: Fields,
}

#[cfg_attr(feature = "builder", bon)]
impl Creation {
    #[cfg_attr(feature = "builder", builder(on(_, into)))]
    #[must_use]
    pub fn new(id: Uuid, name: NonEmptyString) -> Self {
        Self {
            inner: Fields { id, name },
        }
    }
}
