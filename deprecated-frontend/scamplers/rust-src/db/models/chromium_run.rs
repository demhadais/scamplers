use any_value::AnyValue;
pub use common::{NewChipLoadingCommon, NewChromiumRunCommon, NewGemsCommon};
#[cfg(feature = "app")]
use diesel::{Associations, prelude::*};
pub use ocm::{NewOcmChromiumRun, NewOcmGems};
pub use pool_multiplex::{
    NewPoolMultiplexChipLoading, NewPoolMultiplexChromiumRun, NewPoolMultiplexGems,
};
#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{base_model, db_query, db_selection};
#[cfg(feature = "app")]
use scamplers_schema::{chromium_run, tenx_assay};
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

#[db_selection]
#[cfg_attr(feature = "app", diesel(table_name = chromium_run, base_query = chromium_run::table.inner_join(tenx_assay::table)))]
pub struct ChromiumRunSummaryWithParents {
    pub id: Uuid,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(readonly))]
    pub links: Links,
    pub readable_id: String,
    pub run_at: OffsetDateTime,
    pub run_by: Uuid,
    pub succeeded: bool,
    #[cfg_attr(feature = "app", diesel(embed))]
    pub assay: TenxAssay,
    pub additional_data: Option<AnyValue>,
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
#[cfg_attr(
    feature = "python",
    pyclass(eq, get_all, module = "scamplepy.responses")
)]
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
    pub additional_data: Vec<AnyValue>,
    #[builder(default)]
    pub order_by: DefaultVec<ChromiumRunOrderBy>,
    #[builder(default)]
    pub pagination: Pagination,
}

#[cfg(feature = "python")]
#[pymethods]
impl ChromiumRunQuery {
    #[new]
    #[pyo3(signature = (*, ids=Vec::default(), readable_ids=Vec::default(), assay=None, run_before=None, run_after=None, succeeded=None, additional_data=Vec::new(), order_by=DefaultVec::default(), limit=Pagination::default().limit, offset=Pagination::default_offset()))]
    fn new(
        ids: Vec<Uuid>,
        readable_ids: Vec<String>,
        assay: Option<TenxAssayQuery>,
        run_before: Option<OffsetDateTime>,
        run_after: Option<OffsetDateTime>,
        succeeded: Option<bool>,
        additional_data: Vec<AnyValue>,
        order_by: DefaultVec<ChromiumRunOrderBy>,
        limit: i64,
        offset: i64,
    ) -> Self {
        Self {
            ids,
            readable_ids,
            assay,
            run_before,
            run_after,
            succeeded,
            additional_data,
            order_by,
            pagination: Pagination { limit, offset },
        }
    }
}

uuid_newtype!(ChromiumRunId);
