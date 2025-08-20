#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{base_model, db_insertion, db_simple_enum};
#[cfg(feature = "python")]
use time::OffsetDateTime;
#[cfg(feature = "python")]
use uuid::Uuid;
#[cfg(feature = "python")]
use valid_string::ValidString;

use crate::db::models::chromium_run::{
    common::{NewChromiumRunCommon, NewGemsCommon},
    singleplex::NewSingleplexChipLoading,
};
#[cfg(feature = "python")]
use crate::db::models::suspension::common::SuspensionMeasurementFields;

const MAX_SUSPENSIONS_IN_OCM_GEMS: usize = 4;
const MAX_GEMS_IN_OCM_RUN: usize = 2;

#[cfg_attr(feature = "python", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[cfg_attr(feature = "python", pyclass(eq, module = "scamplepy.create"))]
#[base_model]
pub struct NewOcmChipLoading(pub NewSingleplexChipLoading);

#[cfg(feature = "python")]
#[pyo3_stub_gen::derive::gen_stub_pymethods]
#[pymethods]
impl NewOcmChipLoading {
    #[new]
    #[pyo3(signature = (*, suspension_id, suspension_volume_loaded, buffer_volume_loaded, notes=None))]
    fn new(
        suspension_id: Uuid,
        suspension_volume_loaded: SuspensionMeasurementFields,
        buffer_volume_loaded: SuspensionMeasurementFields,
        notes: Option<ValidString>,
    ) -> Self {
        use super::common::NewChipLoadingCommon;
        use crate::db::models::chromium_run::singleplex::NewSingleplexChipLoading;

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
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::gems))]
pub struct NewOcmGems {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub inner: NewGemsCommon,
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
    #[garde(length(min = 1, max = MAX_SUSPENSIONS_IN_OCM_GEMS))]
    pub loading: Vec<NewOcmChipLoading>,
}

#[cfg(feature = "python")]
#[pyo3_stub_gen::derive::gen_stub_pymethods]
#[pymethods]
impl NewOcmGems {
    #[new]
    #[pyo3(signature = (*, readable_id, chemistry, loading))]
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

#[db_simple_enum]
#[cfg_attr(feature = "python", pyo3(module = "scamplepy.create"))]
pub enum OcmChromiumChip {
    #[serde(rename = "GEM-X OCM 3'")]
    #[strum(serialize = "GEM-X OCM 3'")]
    GemxOcm3p,
}

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::chromium_run))]
pub struct NewOcmChromiumRun {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub inner: NewChromiumRunCommon,
    pub chip: OcmChromiumChip,
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
    #[garde(length(min = 1, max = MAX_GEMS_IN_OCM_RUN))]
    pub gems: Vec<NewOcmGems>,
}

#[cfg(feature = "python")]
#[pyo3_stub_gen::derive::gen_stub_pymethods]
#[pymethods]
impl NewOcmChromiumRun {
    #[new]
    #[pyo3(signature = (*, readable_id, run_at, succeeded, run_by, chip, gems, notes=None))]
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
                run_by,
                succeeded,
                notes,
            },
            chip,
            gems,
        }
    }
}
