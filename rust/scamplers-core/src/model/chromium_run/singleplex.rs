#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{db_enum, db_insertion, to_from_json};
#[cfg(feature = "backend")]
use scamplers_schema::{chip_loading, chromium_run, gems};
#[cfg(feature = "python")]
use time::OffsetDateTime;
use uuid::Uuid;
#[cfg(feature = "python")]
use valid_string::ValidString;

use crate::model::chromium_run::common::{
    MAX_GEMS_IN_NON_OCM_RUN, NewChipLoadingCommon, NewChromiumRunCommon, NewGemsCommon,
};
#[cfg(feature = "python")]
use crate::model::suspension::MeasurementDataCore;

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = chip_loading))]
pub struct NewSingleplexChipLoading {
    pub suspension_id: Uuid,
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub inner: NewChipLoadingCommon,
}

#[cfg(feature = "python")]
#[pymethods]
impl NewSingleplexChipLoading {
    #[new]
    #[pyo3(signature = (*, suspension_id, suspension_volume_loaded, buffer_volume_loaded, notes=None))]
    fn new(
        suspension_id: Uuid,
        suspension_volume_loaded: MeasurementDataCore,
        buffer_volume_loaded: MeasurementDataCore,
        notes: Option<ValidString>,
    ) -> Self {
        Self {
            suspension_id,
            inner: NewChipLoadingCommon {
                gems_id: Uuid::default(),
                suspension_volume_loaded,
                buffer_volume_loaded,
                notes,
            },
        }
    }
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

#[cfg(feature = "python")]
#[pymethods]
impl NewSingleplexGems {
    #[new]
    #[pyo3(signature = (*, readable_id, chemistry, loading))]
    fn new(
        readable_id: ValidString,
        chemistry: ValidString,
        loading: NewSingleplexChipLoading,
    ) -> Self {
        Self {
            inner: NewGemsCommon {
                readable_id,
                chemistry,
                chromium_run_id: Uuid::default(),
            },
            loading,
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

#[cfg(feature = "python")]
#[pymethods]
impl NewSingleplexChromiumRun {
    #[new]
    #[pyo3(signature = (*, readable_id, run_at, run_by, succeeded, chip, gems, notes=None))]
    fn new(
        readable_id: ValidString,
        run_at: OffsetDateTime,
        run_by: Uuid,
        succeeded: bool,
        chip: SingleplexChromiumChip,
        gems: Vec<NewSingleplexGems>,
        notes: Option<ValidString>,
    ) -> Self {
        Self {
            inner: NewChromiumRunCommon {
                readable_id,
                run_at,
                run_by,
                succeeded,
                notes,
            },
            chip,
            gems,
        }
    }
}
