use scamplers_macros::{db_enum, db_insertion};
#[cfg(feature = "backend")]
use scamplers_schema::specimen;

use super::common::NewSpecimenCommon;

#[db_enum]
#[derive(Default)]
pub enum SuspensionType {
    #[default]
    Suspension,
}

#[db_enum]
#[derive(Default)]
pub enum SuspensionFixative {
    #[default]
    FormaldehydeDerivative,
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = specimen))]
pub struct NewVirtualSpecimen {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub inner: NewSpecimenCommon,
    #[serde(skip)]
    pub type_: SuspensionType,
    pub fixative: SuspensionFixative,
}
