#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{db_insertion, db_simple_enum};
#[cfg(feature = "python")]
use time::OffsetDateTime;
#[cfg(feature = "python")]
use uuid::Uuid;
use valid_string::ValidString;

use crate::db::models::chromium_run::{
    common::{NewChromiumRunCommon, NewGemsCommon},
    singleplex::NewSingleplexChipLoading,
};

const MAX_SUSPENSIONS_IN_OCM_GEMS: usize = 4;
const MAX_GEMS_IN_OCM_RUN: usize = 2;

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::gems))]
pub struct NewOcmGems {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub inner: NewGemsCommon,
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
    #[garde(dive, length(min = 1, max = MAX_SUSPENSIONS_IN_OCM_GEMS))]
    pub loading: Vec<NewSingleplexChipLoading>,
}

#[cfg(feature = "python")]
#[pyo3_stub_gen::derive::gen_stub_pymethods]
#[pymethods]
impl NewOcmGems {
    #[new]
    #[pyo3(signature = (*, readable_id, loading))]
    fn new(readable_id: ValidString, loading: Vec<NewSingleplexChipLoading>) -> Self {
        Self {
            inner: NewGemsCommon {
                readable_id,
                chromium_run_id: Uuid::default(),
            },
            loading,
        }
    }
}

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::chromium_run))]
pub struct NewOcmChromiumRun {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub inner: NewChromiumRunCommon,
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
    #[garde(dive, length(min = 1, max = MAX_GEMS_IN_OCM_RUN))]
    pub gems: Vec<NewOcmGems>,
}

#[cfg(feature = "python")]
#[pyo3_stub_gen::derive::gen_stub_pymethods]
#[pymethods]
impl NewOcmChromiumRun {
    #[new]
    #[pyo3(signature = (*, readable_id, assay_id, run_at, succeeded, run_by, gems, notes=None))]
    fn new(
        readable_id: ValidString,
        assay_id: Uuid,
        run_at: OffsetDateTime,
        succeeded: bool,
        run_by: Uuid,
        gems: Vec<NewOcmGems>,
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
