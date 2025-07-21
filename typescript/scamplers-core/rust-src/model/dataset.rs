pub use chromium::{NewChromiumDataset, NewChromiumDatasetCore};
use scamplers_macros::{base_api_model, db_selection};
#[cfg(feature = "backend")]
use scamplers_schema::dataset;
use serde_json::Value;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::model::lab::LabHandle;

mod chromium;
mod common;

#[base_api_model]
#[serde(tag = "type")]
pub enum NewDataset {
    Chromium(#[garde(dive)] NewChromiumDataset),
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = dataset))]
pub struct DatasetHandle {
    pub id: Uuid,
    pub link: String,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = dataset))]
pub struct DatasetSummary {
    #[serde(flatten)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub handle: DatasetHandle,
    pub data_path: String,
    pub delivered_at: OffsetDateTime,
    #[valuable(skip)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub metrics: Option<Value>,
    pub web_summary: Option<String>,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = dataset))]
pub struct DatasetCore {
    #[serde(flatten)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub summary: DatasetSummary,
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub lab: LabHandle,
}
