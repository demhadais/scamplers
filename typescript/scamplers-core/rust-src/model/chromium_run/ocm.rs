use scamplers_macros::{db_enum, db_insertion, to_json};
#[cfg(feature = "backend")]
use scamplers_schema::{chromium_run, gems};

use crate::model::chromium_run::{
    common::{NewChromiumRunCommon, NewGemsCommon},
    singleplex::NewSingleplexChipLoading,
};

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = gems))]
pub struct NewOcmGems {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub inner: NewGemsCommon,
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    #[garde(length(min = 1, max = 4))]
    pub loading: Vec<NewSingleplexChipLoading>,
}

#[db_enum]
pub enum OcmChromiumChip {
    #[serde(rename = "GEM-X OCM 3'")]
    #[strum(serialize = "GEM-X OCM 3'")]
    GemxOcm3p,
}

#[to_json(python)]
#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = chromium_run))]
pub struct NewOcmChromiumRun {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub inner: NewChromiumRunCommon,
    pub chip: OcmChromiumChip,
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    #[garde(length(min = 1, max = 2))]
    pub gems: Vec<NewOcmGems>,
}
