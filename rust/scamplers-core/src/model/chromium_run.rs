use crate::model::suspension::MeasurementDataCore;
use scamplers_macros::{base_api_model, db_enum, db_insertion};
use scamplers_schema::{chip_loading, chromium_run, gems};
use time::OffsetDateTime;
use uuid::Uuid;
use valid_string::ValidString;

mod common;
mod mutiplex;
mod ocm;
mod singleplex;

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = gems))]
pub struct NewSingleplexGems {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub inner: NewGemsCore,
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    pub loading: NewSingleplexSuspensionChipLoading,
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = chip_loading))]
pub struct MultiplexedSuspensionChipLoading {
    pub multiplexed_suspension_id: Uuid,
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub inner: ChipLoadingCore,
}

#[base_api_model]
#[cfg_attr(not(target_arch = "wasm32"), pyo3::pyclass)]
#[serde(tag = "plexy")]
pub enum NewGems {
    Singleplexed {
        #[serde(flatten)]
        #[garde(dive)]
        inner: NewGemsCore,
        #[garde(dive, length(min = 1, max = MAX_SUSPENSIONS_OCM))]
        loading: Vec<NewSingleplexSuspensionChipLoading>,
    },
    Multiplexed {
        #[serde(flatten)]
        #[garde(dive)]
        inner: NewGemsCore,
        #[garde(dive)]
        loading: MultiplexedSuspensionChipLoading,
    },
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

#[db_enum]
enum MultiplexChromiumChip {
    #[serde(rename = "gemx_ocm_5p")]
    #[strum(serialize = "gemx_ocm_5p")]
    Gemx5p,
}

#[db_enum]
enum OcmChromiumChip {
    #[serde(rename = "gemx_ocm_3p")]
    #[strum(serialize = "gemx_ocm_3p")]
    GemxOcm3p,
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = chromium_run))]
struct NewChromiumRunCore {
    #[garde(dive)]
    pub readable_id: ValidString,
    pub run_at: OffsetDateTime,
    pub succeeded: bool,
    pub notes: Option<ValidString>,
    pub run_by: Uuid,
}

#[db_enum]
pub enum ChromiumChip {
    J,
    H,
    Q,
    GemxFx,
    #[serde(rename = "gemx_3p")]
    #[strum(serialize = "gemx_3p")]
    Gemx3p,
    #[serde(rename = "gemx_ocm_3p")]
    #[strum(serialize = "gemx_ocm_3p")]
    GemxOcm3p,
    #[serde(rename = "gemx_ocm_5p")]
    #[strum(serialize = "gemx_ocm_5p")]
    Gemx5p,
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = chromium_run))]
struct NewChromiumRun {
    #[garde(dive)]
    pub readable_id: ValidString,
    pub chip: ChromiumChip,
    pub run_at: OffsetDateTime,
    pub succeeded: bool,
    pub notes: Option<ValidString>,
    pub run_by: Uuid,
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    #[garde(dive, length(min = 1, max = 8))]
    pub gems: Vec<NewGems>,
}
