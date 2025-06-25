#[cfg(feature = "backend")]
use {
    scamplers_macros::{backend_db_enum, backend_insertion},
    scamplers_schema::specimen,
};

use crate::model::specimen::common::NewSpecimenCommon;

#[cfg_attr(feature = "backend", backend_db_enum)]
#[derive(Default)]
pub enum SuspensionType {
    #[default]
    Suspension,
}

#[cfg_attr(feature = "backend", backend_db_enum)]
pub enum SuspensionFixative {
    FormaldehydeDerivative,
}

#[cfg_attr(feature = "backend", backend_insertion(specimen))]
pub struct NewVirtualSuspensionSpecimen {
    #[cfg_attr(feature = "backend", diesel(embed), serde(flatten), garde(dive))]
    pub(super) common: NewSpecimenCommon,
    #[cfg_attr(feature = "backend", serde(skip))]
    type_: SuspensionType,
    fixative: SuspensionFixative,
}
