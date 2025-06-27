use getset::MutGetters;
use scamplers_macros::{base_api_model, db_enum, db_insertion};
#[cfg(feature = "backend")]
use scamplers_schema::specimen;
use uuid::Uuid;

use crate::{
    model::specimen::common::{NewSpecimenCommon, NewSpecimenMeasurement},
    string::NonEmptyString,
};

#[db_enum]
#[derive(Default)]
pub enum TissueType {
    #[default]
    Tissue,
}

#[db_enum]
pub enum TissueFixative {
    DithiobisSuccinimidylropionate,
}

#[db_insertion]
#[derive(MutGetters)]
#[cfg_attr(feature = "backend", diesel(table_name = specimen))]
pub struct NewFixedTissue {
    #[serde(flatten)]
    #[garde(dive)]
    #[getset(get_mut)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    inner: NewSpecimenCommon,
    #[serde(skip)]
    type_: TissueType,
    #[garde(dive)]
    storage_buffer: Option<NonEmptyString>,
    fixative: TissueFixative,
}

#[db_insertion]
#[derive(MutGetters)]
#[cfg_attr(feature = "backend", diesel(table_name = specimen))]
pub struct NewFrozenTissue {
    #[serde(flatten)]
    #[garde(dive)]
    #[getset(get_mut)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    inner: NewSpecimenCommon,
    #[serde(skip)]
    type_: TissueType,
    #[garde(dive)]
    storage_buffer: Option<NonEmptyString>,
    #[garde(custom(super::common::is_true))]
    frozen: bool,
}

#[db_insertion]
#[derive(MutGetters)]
#[cfg_attr(feature = "backend", diesel(table_name = specimen))]
pub struct NewCryoPreservedTissue {
    #[serde(flatten)]
    #[garde(dive)]
    #[getset(get_mut)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    inner: NewSpecimenCommon,
    #[serde(skip)]
    type_: TissueType,
    #[garde(dive)]
    storage_buffer: Option<NonEmptyString>,
    #[garde(custom(super::common::is_true))]
    cryopreserved: bool,
}

#[base_api_model]
#[serde(tag = "preservation")]
pub enum NewTissue {
    Cryopreserved(#[garde(dive)] NewCryoPreservedTissue),
    Fixed(#[garde(dive)] NewFixedTissue),
    Frozen(#[garde(dive)] NewFrozenTissue),
}
impl NewTissue {
    pub(super) fn inner_mut(&mut self) -> &mut NewSpecimenCommon {
        match self {
            Self::Cryopreserved(t) => t.inner_mut(),
            Self::Fixed(t) => t.inner_mut(),
            Self::Frozen(t) => t.inner_mut(),
        }
    }
}
