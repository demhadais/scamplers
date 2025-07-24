use chromium::{JsonMetricsFile, MultiRowCsvMetricsFileGroup, SingleRowCsvMetricsFile};
pub use chromium::{NewChromiumDataset, NewChromiumDatasetCore};
use scamplers_macros::{base_api_model, db_json, db_selection, to_json};
#[cfg(feature = "backend")]
use scamplers_schema::dataset;
use time::OffsetDateTime;
use uuid::Uuid;

pub mod chromium;
mod common;

#[base_api_model]
#[serde(tag = "type")]
pub enum NewDataset {
    Chromium(#[garde(dive)] NewChromiumDataset),
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

#[to_json(python)]
#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = dataset))]
pub struct DatasetHandle {
    pub id: Uuid,
    pub link: String,
}

#[to_json(python)]
#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = dataset))]
pub struct DatasetSummary {
    #[serde(flatten)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub handle: DatasetHandle,
    pub data_path: String,
    pub delivered_at: OffsetDateTime,
    pub web_summary: Option<String>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub metrics: Option<ParsedMetricsFile>,
}
