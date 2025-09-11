pub use common::{NewChipLoadingCommon, NewChromiumRunCommon, NewGemsCommon};
#[cfg(feature = "app")]
use diesel::Associations;
pub use ocm::{NewOcmChromiumRun, NewOcmGems};
pub use pool_multiplex::{
    NewPoolMultiplexChipLoading, NewPoolMultiplexChromiumRun, NewPoolMultiplexGems,
};
#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3_stub_gen::impl_stub_type;
use scamplers_macros::{base_model, db_query, db_selection};
pub use singleplex::{NewSingleplexChipLoading, NewSingleplexChromiumRun, NewSingleplexGems};
use time::OffsetDateTime;
use uuid::Uuid;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::{
    db::models::{
        DefaultVec, Links, Pagination,
        tenx_assay::{TenxAssay, TenxAssayQuery},
    },
    define_ordering_enum, uuid_newtype,
};

mod common;
#[cfg(feature = "app")]
mod create;
pub mod ocm;
pub mod pool_multiplex;
#[cfg(feature = "app")]
mod read;
pub mod singleplex;

#[base_model]
#[serde(tag = "plexy")]
#[cfg_attr(feature = "python", derive(FromPyObject))]
pub enum NewChromiumRun {
    Singleplex(#[garde(dive)] NewSingleplexChromiumRun),
    Ocm(#[garde(dive)] NewOcmChromiumRun),
    PoolMultiplex(#[garde(dive)] NewPoolMultiplexChromiumRun),
}

#[cfg(feature = "python")]
impl_stub_type!(
    NewChromiumRun = NewSingleplexChromiumRun | NewOcmChromiumRun | NewPoolMultiplexChromiumRun
);

#[db_selection]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::chromium_run))]
pub struct ChromiumRunSummaryWithParents {
    pub id: Uuid,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub links: Links,
    pub readable_id: String,
    pub run_at: OffsetDateTime,
    pub run_by: Uuid,
    pub succeeded: bool,
    pub notes: Option<String>,
    #[cfg_attr(feature = "app", diesel(embed))]
    pub assay: TenxAssay,
}

#[db_selection]
#[cfg_attr(feature = "app", derive(Associations))]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::gems, belongs_to(ChromiumRunSummaryWithParents, foreign_key = chromium_run_id)))]
pub struct Gems {
    pub id: Uuid,
    pub readable_id: String,
    pub chromium_run_id: Uuid,
}

#[base_model]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[cfg_attr(feature = "python", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[cfg_attr(feature = "python", pyclass(eq, module = "scamplepy.responses"))]
pub struct ChromiumRun {
    #[serde(flatten)]
    pub info: ChromiumRunSummaryWithParents,
    pub gems: Vec<Gems>,
}

define_ordering_enum! {ChromiumRunOrderBy {RunAt}, default = RunAt}

#[db_query]
pub struct ChromiumRunQuery {
    #[builder(default)]
    pub ids: Vec<Uuid>,
    #[builder(default)]
    pub readable_ids: Vec<String>,
    pub assay: Option<TenxAssayQuery>,
    pub run_before: Option<OffsetDateTime>,
    pub run_after: Option<OffsetDateTime>,
    pub succeeded: Option<bool>,
    #[builder(default)]
    pub notes: Vec<String>,
    #[builder(default)]
    pub order_by: DefaultVec<ChromiumRunOrderBy>,
    #[builder(default)]
    pub pagination: Pagination,
}

uuid_newtype!(ChromiumRunId);
