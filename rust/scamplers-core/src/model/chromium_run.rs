use scamplers_macros::{base_api_model, db_enum, db_insertion};
use scamplers_schema::{chip_loading, chromium_run, gems};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::model::suspension::MeasurementDataCore;

const MAX_SUSPENSIONS_OCM: usize = 4;

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = gems))]
struct NewGemsCore {
    readable_id: String,
    #[serde(skip)]
    n_samples: i32,
    chemistry: String,
    #[serde(skip)]
    chromium_run_id: Uuid,
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = chip_loading))]
struct ChipLoadingCore {
    #[serde(default)]
    gems_id: Uuid,
    #[garde(dive)]
    suspension_volume_loaded: MeasurementDataCore,
    #[garde(dive)]
    buffer_volume_loaded: MeasurementDataCore,
    notes: Option<String>,
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = chip_loading))]
struct SingleplexedSuspensionChipLoading {
    suspension_id: Uuid,
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    inner: ChipLoadingCore,
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = chip_loading))]
struct MultiplexedSuspensionChipLoading {
    multiplexed_suspension_id: Uuid,
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    inner: ChipLoadingCore,
}

#[base_api_model]
#[serde(tag = "plexy")]
enum NewGems {
    Singleplexed {
        #[serde(flatten)]
        #[garde(dive)]
        inner: NewGemsCore,
        #[garde(dive, length(min = 1, max = MAX_SUSPENSIONS_OCM))]
        loading: Vec<SingleplexedSuspensionChipLoading>,
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
enum ChromiumChip {
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
    readable_id: String,
    chip: ChromiumChip,
    run_at: OffsetDateTime,
    succeeded: bool,
    notes: Option<String>,
    run_by: Uuid,
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    #[garde(dive, length(min = 1, max = 8))]
    gems: Vec<NewGems>,
}
