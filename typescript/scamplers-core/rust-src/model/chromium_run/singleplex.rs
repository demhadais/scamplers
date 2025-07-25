use scamplers_macros::{db_enum, db_insertion, to_from_json};
#[cfg(feature = "backend")]
use scamplers_schema::{chip_loading, chromium_run, gems};
use time::OffsetDateTime;
use uuid::Uuid;
use valid_string::ValidString;

use crate::model::{
    chromium_run::common::{
        MAX_GEMS_IN_NON_OCM_RUN, NewChipLoadingCommon, NewChromiumRunCommon, NewGemsCommon,
    },
    suspension::MeasurementDataCore,
};

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = chip_loading))]
pub struct NewSingleplexChipLoading {
    pub suspension_id: Uuid,
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub inner: NewChipLoadingCommon,
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = gems))]
pub struct NewSingleplexGems {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub inner: NewGemsCommon,
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    pub loading: NewSingleplexChipLoading,
}

impl NewSingleplexGems {
    fn new(
        readable_id: ValidString,
        chemistry: ValidString,
        suspension_id: Uuid,
        suspension_volume_loaded: MeasurementDataCore,
        buffer_volume_loaded: MeasurementDataCore,
        notes: Option<ValidString>,
    ) -> Self {
        Self {
            inner: NewGemsCommon {
                readable_id,
                chemistry,
                chromium_run_id: Uuid::default(),
            },
            loading: NewSingleplexChipLoading {
                suspension_id,
                inner: NewChipLoadingCommon {
                    gems_id: Uuid::default(),
                    suspension_volume_loaded,
                    buffer_volume_loaded,
                    notes,
                },
            },
        }
    }
}

#[db_enum]
pub enum SingleplexChromiumChip {
    #[serde(rename = "J")]
    #[strum(serialize = "J")]
    J,
    #[serde(rename = "H")]
    #[strum(serialize = "H")]
    H,
    #[serde(rename = "GEM-X FX")]
    #[strum(serialize = "GEM-X FX")]
    GemxFx,
    #[serde(rename = "GEM-X 3'")]
    #[strum(serialize = "GEM-X 3'")]
    Gemx3p,
    #[serde(rename = "GEM-X 5'")]
    #[strum(serialize = "GEM-X 5'")]
    Gemx5p,
}

#[to_from_json(python)]
#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = chromium_run))]
pub struct NewSingleplexChromiumRun {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub inner: NewChromiumRunCommon,
    pub chip: SingleplexChromiumChip,
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    #[garde(dive, length(min = 1, max = MAX_GEMS_IN_NON_OCM_RUN))]
    pub gems: Vec<NewSingleplexGems>,
}

impl NewSingleplexChromiumRun {
    fn new(
        readable_id: ValidString,
        run_at: OffsetDateTime,
        succeeded: bool,
        run_by: Uuid,
        chip: SingleplexChromiumChip,
        gems: Vec<NewSingleplexGems>,
        notes: Option<ValidString>,
    ) -> Self {
        Self {
            inner: NewChromiumRunCommon {
                readable_id,
                run_at,
                succeeded,
                notes,
                run_by,
            },
            chip,
            gems,
        }
    }
}
