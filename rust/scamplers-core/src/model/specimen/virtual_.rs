use getset::MutGetters;
use scamplers_macros::{db_enum, db_insertion};
#[cfg(feature = "backend")]
use scamplers_schema::specimen;
use uuid::Uuid;

use crate::model::specimen::NewSpecimenCommon;

#[db_enum]
#[derive(Default)]
pub enum SuspensionType {
    #[default]
    Suspension,
}

#[db_enum]
pub enum SuspensionFixative {
    FormaldehydeDerivative,
}

#[db_insertion]
#[derive(MutGetters)]
#[cfg_attr(feature = "backend", diesel(table_name = specimen))]
pub struct NewVirtualSpecimen {
    #[serde(flatten)]
    #[garde(dive)]
    #[getset(get_mut = "pub(super)")]
    #[cfg_attr(feature = "backend", diesel(embed))]
    inner: NewSpecimenCommon,
    #[serde(skip)]
    type_: SuspensionType,
    fixative: SuspensionFixative,
}
