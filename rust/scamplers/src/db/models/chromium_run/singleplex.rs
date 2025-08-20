#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{db_insertion, db_simple_enum};
#[cfg(feature = "python")]
use time::OffsetDateTime;
use uuid::Uuid;
use valid_string::ValidString;

use crate::db::models::chromium_run::common::{
    MAX_GEMS_IN_NON_OCM_RUN, NewChipLoadingCommon, NewChromiumRunCommon, NewGemsCommon,
};
#[cfg(feature = "python")]
use crate::db::models::suspension::common::SuspensionMeasurementFields;

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::chip_loading))]
#[cfg_attr(feature = "python", pyo3(name = "_NewSingleplexChipLoading"))]
pub struct NewSingleplexChipLoading {
    pub suspension_id: Uuid,
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub inner: NewChipLoadingCommon,
}

#[cfg(feature = "python")]
#[pyo3_stub_gen::derive::gen_stub_pymethods]
#[pymethods]
impl NewSingleplexChipLoading {
    #[new]
    #[pyo3(signature = (*, suspension_id, suspension_volume_loaded, buffer_volume_loaded, notes=None))]
    fn new(
        suspension_id: Uuid,
        suspension_volume_loaded: SuspensionMeasurementFields,
        buffer_volume_loaded: SuspensionMeasurementFields,
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
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::gems))]
pub struct NewSingleplexGems {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub inner: NewGemsCommon,
    #[garde(dive)]
    pub chemistry: Option<ValidString>,
    #[garde(dive, length(min = 1, max = 1))]
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
    pub loading: Vec<NewSingleplexChipLoading>,
}

#[cfg(feature = "python")]
#[pymethods]
impl NewSingleplexGems {
    #[new]
    #[pyo3(signature = (*, readable_id, chemistry, suspension_id, suspension_volume_loaded, buffer_volume_loaded, notes=None))]
    fn new(
        readable_id: ValidString,
        chemistry: Option<ValidString>,
        suspension_id: Uuid,
        suspension_volume_loaded: SuspensionMeasurementFields,
        buffer_volume_loaded: SuspensionMeasurementFields,
        notes: Option<ValidString>,
    ) -> Self {
        Self {
            inner: NewGemsCommon {
                readable_id,
                chromium_run_id: Uuid::default(),
            },
            chemistry,
            loading: vec![NewSingleplexChipLoading {
                suspension_id,
                inner: NewChipLoadingCommon {
                    gems_id: Uuid::default(),
                    suspension_volume_loaded,
                    buffer_volume_loaded,
                    notes,
                },
            }],
        }
    }
}

#[db_simple_enum]
#[derive(strum::Display)]
#[cfg_attr(feature = "python", pyo3(module = "scamplepy.create"))]
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

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::chromium_run))]
pub struct NewSingleplexChromiumRun {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub inner: NewChromiumRunCommon,
    pub chip: SingleplexChromiumChip,
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
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
