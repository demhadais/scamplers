use scamplers_macros::{base_api_model, db_enum, db_insertion};
#[cfg(feature = "backend")]
use scamplers_schema::specimen;
use valid_string::ValidString;

use crate::model::specimen::common::NewSpecimenCommon;

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
#[cfg_attr(feature = "backend", diesel(table_name = specimen))]
pub struct NewFixedTissue {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub inner: NewSpecimenCommon,
    #[serde(skip)]
    pub type_: TissueType,
    #[garde(dive)]
    pub storage_buffer: Option<ValidString>,
    pub fixative: TissueFixative,
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = specimen))]
pub struct NewFrozenTissue {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub inner: NewSpecimenCommon,
    #[serde(skip)]
    pub type_: TissueType,
    #[garde(dive)]
    pub storage_buffer: Option<ValidString>,
    #[garde(custom(super::common::is_true))]
    pub frozen: bool,
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = specimen))]
pub struct NewCryoPreservedTissue {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub inner: NewSpecimenCommon,
    #[serde(skip)]
    pub type_: TissueType,
    #[garde(dive)]
    pub storage_buffer: Option<ValidString>,
    #[garde(custom(super::common::is_true))]
    pub cryopreserved: bool,
}

#[base_api_model]
#[serde(tag = "preservation")]
pub enum NewTissue {
    Cryopreserved(#[garde(dive)] NewCryoPreservedTissue),
    Fixed(#[garde(dive)] NewFixedTissue),
    Frozen(#[garde(dive)] NewFrozenTissue),
}
