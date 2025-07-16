use scamplers_macros::{db_enum, db_insertion};
#[cfg(feature = "backend")]
use scamplers_schema::{chip_loading, chromium_run, gems};
use uuid::Uuid;

use crate::model::chromium_run::common::{
    MAX_GEMS_IN_NON_OCM_RUN, NewChipLoadingCommon, NewChromiumRunCommon, NewGemsCommon,
};

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = chip_loading))]
pub struct NewPoolMultiplexChipLoading {
    pub suspension_pool_id: Uuid,
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub inner: NewChipLoadingCommon,
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = gems))]
pub struct NewPoolMultiplexGems {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub inner: NewGemsCommon,
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    pub loading: NewPoolMultiplexChipLoading,
}

#[db_enum]
pub enum PoolMultiplexChromiumChip {
    #[serde(rename = "Q")]
    #[strum(serialize = "Q")]
    Q,
    #[serde(rename = "GEM-X FX")]
    #[strum(serialize = "GEM-X FX")]
    GemxFx,
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = chromium_run))]
pub struct NewPoolMultiplexChromiumRun {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub inner: NewChromiumRunCommon,
    pub chip: PoolMultiplexChromiumChip,
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    #[garde(dive, length(min = 1, max = MAX_GEMS_IN_NON_OCM_RUN))]
    pub gems: Vec<NewPoolMultiplexGems>,
}
