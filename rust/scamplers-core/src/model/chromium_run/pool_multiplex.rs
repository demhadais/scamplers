use scamplers_macros::{db_enum, db_insertion};
#[cfg(feature = "backend")]
use scamplers_schema::{chip_loading, chromium_run, gems};
#[cfg(feature = "python")]
use time::OffsetDateTime;
use uuid::Uuid;
#[cfg(feature = "python")]
use {pyo3::prelude::*, valid_string::ValidString};

use crate::model::chromium_run::common::{
    MAX_GEMS_IN_NON_OCM_RUN, NewChipLoadingCommon, NewChromiumRunCommon, NewGemsCommon,
};
#[cfg(feature = "python")]
use crate::model::suspension::MeasurementDataCore;

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = chip_loading))]
pub struct NewPoolMultiplexChipLoading {
    pub suspension_pool_id: Uuid,
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub inner: NewChipLoadingCommon,
}

#[cfg(feature = "python")]
#[pymethods]
impl NewPoolMultiplexChipLoading {
    #[new]
    #[pyo3(signature = (*, suspension_pool_id, suspension_volume_loaded, buffer_volume_loaded, notes=None))]
    fn new(
        suspension_pool_id: Uuid,
        suspension_volume_loaded: MeasurementDataCore,
        buffer_volume_loaded: MeasurementDataCore,
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
#[cfg_attr(feature = "app", diesel(table_name = gems))]
pub struct NewPoolMultiplexGems {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub inner: NewGemsCommon,
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
    pub loading: NewPoolMultiplexChipLoading,
}

#[cfg(feature = "python")]
#[pymethods]
impl NewPoolMultiplexGems {
    #[new]
    #[pyo3(signature = (*, readable_id, chemistry, loading))]
    fn new(
        readable_id: ValidString,
        chemistry: ValidString,
        loading: NewPoolMultiplexChipLoading,
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
pub enum PoolMultiplexChromiumChip {
    #[serde(rename = "Q")]
    #[strum(serialize = "Q")]
    Q,
    #[serde(rename = "GEM-X FX")]
    #[strum(serialize = "GEM-X FX")]
    GemxFx,
}

#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(scamplers_macros::FromJson, scamplers_macros::ToJson)
)]
#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = chromium_run))]
#[cfg_attr(not(target_arch = "wasm32"), json(wrapper = super::NewChromiumRun, python))]
pub struct NewPoolMultiplexChromiumRun {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub inner: NewChromiumRunCommon,
    pub chip: PoolMultiplexChromiumChip,
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
    #[garde(dive, length(min = 1, max = MAX_GEMS_IN_NON_OCM_RUN))]
    pub gems: Vec<NewPoolMultiplexGems>,
}

#[cfg(feature = "python")]
#[pymethods]
impl NewPoolMultiplexChromiumRun {
    #[new]
    #[pyo3(signature = (*, readable_id, run_at, run_by, succeeded, chip, gems, notes=None))]
    fn new(
        readable_id: ValidString,
        run_at: OffsetDateTime,
        run_by: Uuid,
        succeeded: bool,
        chip: PoolMultiplexChromiumChip,
        gems: Vec<NewPoolMultiplexGems>,
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
