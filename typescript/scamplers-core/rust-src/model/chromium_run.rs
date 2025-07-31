pub use common::NewChipLoadingCommon;
pub use ocm::{NewOcmChipLoading, NewOcmChromiumRun, NewOcmGems, OcmChromiumChip};
pub use pool_multiplex::{
    NewPoolMultiplexChipLoading, NewPoolMultiplexChromiumRun, NewPoolMultiplexGems,
    PoolMultiplexChromiumChip,
};
#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{base_api_model, db_selection};
#[cfg(feature = "backend")]
use scamplers_schema::{chromium_run, gems};
pub use singleplex::{
    NewSingleplexChipLoading, NewSingleplexChromiumRun, NewSingleplexGems, SingleplexChromiumChip,
};
use time::OffsetDateTime;
use uuid::Uuid;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod common;
mod ocm;
mod pool_multiplex;
mod singleplex;

#[base_api_model]
#[serde(tag = "plexy")]
#[cfg_attr(feature = "python", derive(FromPyObject))]
#[derive(derive_more::TryInto, derive_more::From)]
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
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(scamplers_macros::FromJson, scamplers_macros::ToJson)
)]
#[cfg_attr(not(target_arch = "wasm32"), json(python))]
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
