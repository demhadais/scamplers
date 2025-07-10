use scamplers_macros::{db_enum, db_insertion};
#[cfg(feature = "backend")]
use scamplers_schema::{chip_loading, chromium_run, gems};
use uuid::Uuid;

use crate::model::chromium_run::common::{
    MAX_GEMS_IN_CHROMIUM_RUN, NewChipLoadingCommon, NewChromiumRunCommon, NewGemsCommon,
};

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = chip_loading))]
pub(super) struct NewSingleplexChipLoading {
    pub suspension_id: Uuid,
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub inner: NewChipLoadingCommon,
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = gems))]
pub(super) struct NewSingleplexGems {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub inner: NewGemsCommon,
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    pub loading: NewSingleplexChipLoading,
}

#[db_enum]
enum SingleplexChromiumChip {
    J,
    H,
    Q,
    GemxFx,
    #[serde(rename = "gemx_3p")]
    #[strum(serialize = "gemx_3p")]
    Gemx3p,
    #[serde(rename = "gemx_ocm_5p")]
    #[strum(serialize = "gemx_ocm_5p")]
    Gemx5p,
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = chromium_run))]
pub(super) struct NewSingleplexChromiumRun {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub inner: NewChromiumRunCommon,
    pub chip: SingleplexChromiumChip,
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    #[garde(dive, length(min = 1, max = MAX_GEMS_IN_CHROMIUM_RUN))]
    pub gems: Vec<NewSingleplexGems>,
}
