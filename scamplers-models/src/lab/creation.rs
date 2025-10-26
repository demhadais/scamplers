use bon::bon;
use macro_attributes::insert;
use non_empty_string::NonEmptyString;
#[cfg(feature = "app")]
use scamplers_schema::labs;
use uuid::Uuid;

use crate::lab::common::Fields;

#[insert]
#[cfg_attr(feature = "app", diesel(table_name = labs))]
#[cfg_attr(feature = "schema", schemars(title = "LabCreation"))]
pub struct Creation {
    #[serde(flatten)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub inner: Fields,
}

#[bon]
impl Creation {
    #[builder(on(_, into))]
    pub fn new(name: NonEmptyString, pi_id: Uuid, delivery_dir: NonEmptyString) -> Self {
        Self {
            inner: Fields {
                name,
                pi_id,
                delivery_dir,
            },
        }
    }
}
