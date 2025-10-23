use macro_attributes::select;
use uuid::Uuid;

use crate::institution::common::Fields;

#[select]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::institution))]
pub struct Institution {
    #[serde(flatten)]
    #[cfg_attr(feature = "app", diesel(embed))]
    inner: Fields,
}

impl Institution {
    pub fn id(&self) -> &Uuid {
        &self.inner.id
    }

    pub fn name(&self) -> &str {
        self.inner.name.as_ref()
    }
}
