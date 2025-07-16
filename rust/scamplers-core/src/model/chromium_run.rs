use scamplers_macros::{base_api_model, db_selection};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::model::{
    chromium_run::{
        mutiplex::NewMultiplexChromiumRun, ocm::NewOcmChromiumRun,
        singleplex::NewSingleplexChromiumRun,
    },
    person::PersonHandle,
};
#[cfg(feature = "backend")]
use scamplers_schema::{chromium_run, gems};
pub use {
    common::NewChipLoadingCommon,
    mutiplex::{NewMultiplexChipLoading, NewMultiplexGems},
    ocm::NewOcmGems,
    singleplex::{NewSingleplexChipLoading, NewSingleplexGems},
};

mod common;
mod mutiplex;
mod ocm;
mod singleplex;

#[base_api_model]
#[serde(tag = "plexy")]
pub enum NewChromiumRun {
    Singleplex(#[garde(dive)] NewSingleplexChromiumRun),
    Ocm(#[garde(dive)] NewOcmChromiumRun),
    PoolMultiplex(#[garde(dive)] NewMultiplexChromiumRun),
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = chromium_run))]
pub struct ChromiumRunHandle {
    pub id: Uuid,
    pub link: String,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = chromium_run))]
pub struct ChromiumRunSummary {
    #[serde(flatten)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub handle: ChromiumRunHandle,
    pub readable_id: String,
    pub chip: String,
    pub run_at: OffsetDateTime,
    pub succeeded: bool,
    pub notes: Option<String>,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = chromium_run))]
pub struct ChromiumRunCore {
    #[serde(flatten)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub summary: ChromiumRunSummary,
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub run_by: PersonHandle,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = gems))]
pub struct GemsHandle {
    pub id: Uuid,
    pub link: String,
}

#[base_api_model]
#[cfg_attr(feature = "python", pyo3::pyclass)]
pub struct ChromiumRun {
    #[serde(flatten)]
    pub core: ChromiumRunCore,
    pub gems: Vec<GemsHandle>,
}
