pub use common::NewChipLoadingCommon;
pub use ocm::NewOcmGems;
pub use pool_multiplex::{NewPoolMultiplexChipLoading, NewPoolMultiplexGems};
use scamplers_macros::{base_api_model, db_selection, getters_impl};
#[cfg(feature = "backend")]
use scamplers_schema::{chromium_run, gems};
pub use singleplex::{NewSingleplexChipLoading, NewSingleplexGems};
use time::OffsetDateTime;
use uuid::Uuid;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::model::chromium_run::{
    ocm::NewOcmChromiumRun, pool_multiplex::NewPoolMultiplexChromiumRun,
    singleplex::NewSingleplexChromiumRun,
};

mod common;
mod ocm;
mod pool_multiplex;
mod singleplex;

#[base_api_model]
#[serde(tag = "plexy")]
pub enum NewChromiumRun {
    Singleplex(#[garde(dive)] NewSingleplexChromiumRun),
    Ocm(#[garde(dive)] NewOcmChromiumRun),
    PoolMultiplex(#[garde(dive)] NewPoolMultiplexChromiumRun),
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

#[getters_impl]
impl ChromiumRunSummary {
    #[must_use]
    pub fn id(&self) -> Uuid {
        self.handle.id
    }
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = gems))]
pub struct GemsHandle {
    pub id: Uuid,
    pub link: String,
}

#[base_api_model]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[cfg_attr(feature = "python", pyo3::pyclass)]
pub struct ChromiumRun {
    #[serde(flatten)]
    pub summary: ChromiumRunSummary,
    pub gems: Vec<GemsHandle>,
}

#[getters_impl]
impl ChromiumRun {
    #[must_use]
    pub fn id(&self) -> Uuid {
        self.summary.id()
    }
}
