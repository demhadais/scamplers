#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::db_insertion;
#[cfg(feature = "python")]
use time::OffsetDateTime;
use uuid::Uuid;
#[cfg(feature = "python")]
use valid_string::ValidString;

use crate::db::models::chromium_run::common::{
    MAX_GEMS_IN_NON_OCM_RUN, NewChipLoadingCommon, NewChromiumRunCommon, NewGemsCommon,
};
#[cfg(feature = "python")]
use crate::db::models::suspension::common::SuspensionMeasurementFields;

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::chip_loading))]
pub struct NewPoolMultiplexChipLoading {
    pub suspension_pool_id: Uuid,
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub inner: NewChipLoadingCommon,
}

#[cfg(feature = "python")]
#[pyo3_stub_gen::derive::gen_stub_pymethods]
#[pymethods]
impl NewPoolMultiplexChipLoading {
    #[new]
    #[pyo3(signature = (*, suspension_pool_id, suspension_volume_loaded, buffer_volume_loaded, notes=None))]
    fn new(
        suspension_pool_id: Uuid,
        suspension_volume_loaded: SuspensionMeasurementFields,
        buffer_volume_loaded: SuspensionMeasurementFields,
        notes: Option<ValidString>,
    ) -> Self {
        Self {
            suspension_pool_id,
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
pub struct NewPoolMultiplexGems {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub inner: NewGemsCommon,
    #[garde(dive, length(min = 1, max = 1))]
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
    pub loading: Vec<NewPoolMultiplexChipLoading>,
}

#[cfg(feature = "python")]
#[pyo3_stub_gen::derive::gen_stub_pymethods]
#[pymethods]
impl NewPoolMultiplexGems {
    #[new]
    #[pyo3(signature = (*, readable_id, suspension_pool_id, suspension_volume_loaded, buffer_volume_loaded, notes=None))]
    fn new(
        readable_id: ValidString,
        suspension_pool_id: Uuid,
        suspension_volume_loaded: SuspensionMeasurementFields,
        buffer_volume_loaded: SuspensionMeasurementFields,
        notes: Option<ValidString>,
    ) -> Self {
        Self {
            inner: NewGemsCommon {
                readable_id,
                chromium_run_id: Uuid::default(),
            },
            loading: vec![NewPoolMultiplexChipLoading {
                suspension_pool_id,
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

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::chromium_run))]
pub struct NewPoolMultiplexChromiumRun {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub inner: NewChromiumRunCommon,
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
    #[garde(dive, length(min = 1, max = MAX_GEMS_IN_NON_OCM_RUN))]
    pub gems: Vec<NewPoolMultiplexGems>,
}

#[cfg(feature = "python")]
#[pyo3_stub_gen::derive::gen_stub_pymethods]
#[pymethods]
impl NewPoolMultiplexChromiumRun {
    #[new]
    #[pyo3(signature = (*, readable_id, assay_id, run_at, run_by, succeeded, gems, notes=None))]
    fn new(
        readable_id: ValidString,
        assay_id: Uuid,
        run_at: OffsetDateTime,
        run_by: Uuid,
        succeeded: bool,
        gems: Vec<NewPoolMultiplexGems>,
        notes: Option<ValidString>,
    ) -> Self {
        Self {
            inner: NewChromiumRunCommon {
                readable_id,
                assay_id,
                run_at,
                run_by,
                succeeded,
                notes,
            },
            gems,
        }
    }
}
