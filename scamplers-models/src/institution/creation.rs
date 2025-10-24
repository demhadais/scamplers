use bon::bon;
use macro_attributes::insert;
use non_empty_string::NonEmptyString;
use uuid::Uuid;

use crate::institution::common::Fields;

#[insert]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::institution))]
#[cfg_attr(feature = "schema", schemars(title = "InstitutionCreation"))]
pub struct Creation {
    #[serde(flatten)]
    #[cfg_attr(feature = "app", diesel(embed))]
    inner: Fields,
}

#[bon]
impl Creation {
    #[builder(on(_, into))]
    pub fn new(id: Uuid, name: NonEmptyString) -> Self {
        Self {
            inner: Fields { id, name },
        }
    }

    #[must_use]
    pub fn id(&self) -> &Uuid {
        &self.inner.id
    }

    #[must_use]
    pub fn name(&self) -> &str {
        self.inner.name.as_ref()
    }
}
