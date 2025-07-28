#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{base_api_model, db_enum, db_insertion, to_from_json};
#[cfg(feature = "backend")]
use scamplers_schema::{chromium_run, gems};
#[cfg(feature = "python")]
use time::OffsetDateTime;
#[cfg(feature = "python")]
use uuid::Uuid;
#[cfg(feature = "python")]
use valid_string::ValidString;

use crate::model::chromium_run::{
    common::{NewChromiumRunCommon, NewGemsCommon},
    singleplex::NewSingleplexChipLoading,
};
#[cfg(feature = "python")]
use crate::model::suspension::MeasurementDataCore;

#[cfg_attr(feature = "python", pyclass)]
#[base_api_model]
pub struct NewOcmChipLoading(pub NewSingleplexChipLoading);

#[cfg(feature = "python")]
#[pymethods]
impl NewOcmChipLoading {
    #[new]
    fn new(
        suspension_id: Uuid,
        suspension_volume_loaded: MeasurementDataCore,
        buffer_volume_loaded: MeasurementDataCore,
        notes: Option<ValidString>,
    ) -> Self {
        use super::common::NewChipLoadingCommon;

        Self(NewSingleplexChipLoading {
            suspension_id,
            inner: NewChipLoadingCommon {
                gems_id: Uuid::default(),
                suspension_volume_loaded,
                buffer_volume_loaded,
                notes,
            },
        })
    }
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = gems))]
pub struct NewOcmGems {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub inner: NewGemsCommon,
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    #[garde(length(min = 1, max = 4))]
    pub loading: Vec<NewOcmChipLoading>,
}

#[cfg(feature = "python")]
#[pymethods]
impl NewOcmGems {
    #[new]
    fn new(
        readable_id: ValidString,
        chemistry: ValidString,
        loading: Vec<NewOcmChipLoading>,
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
pub enum OcmChromiumChip {
    #[serde(rename = "GEM-X OCM 3'")]
    #[strum(serialize = "GEM-X OCM 3'")]
    GemxOcm3p,
}

#[to_from_json(python)]
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

#[cfg(feature = "python")]
#[pymethods]
impl NewOcmChromiumRun {
    #[new]
    fn new(
        readable_id: ValidString,
        run_at: OffsetDateTime,
        succeeded: bool,
        run_by: Uuid,
        chip: OcmChromiumChip,
        gems: Vec<NewOcmGems>,
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
