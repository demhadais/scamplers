use chromium::{JsonMetricsFile, MultiRowCsvMetricsFileGroup, SingleRowCsvMetricsFile};
pub use chromium::{NewChromiumDataset, NewChromiumDatasetCore};
#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{base_api_model, db_json, db_selection};
#[cfg(feature = "backend")]
use scamplers_schema::dataset;
use time::OffsetDateTime;
use uuid::Uuid;

pub mod chromium;
mod common;

#[base_api_model]
#[serde(tag = "type")]
#[cfg_attr(feature = "python", pyclass(get_all, set_all, str))]
pub enum NewDataset {
    Chromium(#[garde(dive)] NewChromiumDataset),
}

impl<T> From<T> for NewDataset
where
    NewChromiumDataset: From<T>,
{
    fn from(value: T) -> Self {
        Self::Chromium(value.into())
    }
}

#[db_json]
#[serde(tag = "format")]
pub enum ParsedMetricsFile {
    #[serde(rename = "10x_single_row_csv")]
    TenxSingleRowCsv(SingleRowCsvMetricsFile),
    #[serde(rename = "10x_multi_row_csv_group")]
    TenxMultiRowCsvGroup(MultiRowCsvMetricsFileGroup),
    #[serde(rename = "10x_json")]
    TenxJson(JsonMetricsFile),
}

#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(scamplers_macros::FromJson, scamplers_macros::ToJson)
)]
#[db_selection]
#[cfg_attr(feature = "app", diesel(table_name = dataset))]
pub struct DatasetHandle {
    pub id: Uuid,
    pub link: String,
}

#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(scamplers_macros::FromJson, scamplers_macros::ToJson)
)]
#[db_selection]
#[cfg_attr(feature = "app", diesel(table_name = dataset))]
pub struct DatasetSummary {
    #[serde(flatten)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub handle: DatasetHandle,
    pub data_path: String,
    pub delivered_at: OffsetDateTime,
    pub web_summary: Option<String>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub metrics: Option<ParsedMetricsFile>,
}
