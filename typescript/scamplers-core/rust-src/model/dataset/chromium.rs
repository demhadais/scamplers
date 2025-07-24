use std::collections::HashMap;

use any_value::AnyValue;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{base_api_model, db_insertion, db_json, to_json};
#[cfg(feature = "backend")]
use scamplers_schema::dataset;
use uuid::Uuid;
use valid_string::ValidString;

use crate::model::dataset::common::NewDatasetCommon;

#[db_json]
pub struct SingleRowCsvMetricsFile {
    #[garde(pattern(r#"^[0-9A-Za-z_-]*summary\.csv$"#))]
    pub filename: String,
    #[garde(dive)]
    pub raw_contents: ValidString,
    #[serde(skip)]
    pub contents: HashMap<String, AnyValue>,
}

#[db_json]
pub struct MultiRowCsvMetricsFile {
    #[garde(pattern(r#"^[0-9A-Za-z_-]+/[[:alnum:]]+summary\.csv$"#))]
    pub filename: String,
    #[garde(dive)]
    pub raw_contents: ValidString,
    #[serde(skip)]
    pub contents: Vec<HashMap<String, AnyValue>>,
}

#[db_json]
#[serde(transparent)]
#[garde(transparent)]
pub struct MultiRowCsvMetricsFileGroup(#[garde(dive, length(min = 1))] Vec<MultiRowCsvMetricsFile>);
impl MultiRowCsvMetricsFileGroup {
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn as_mut_slice(&mut self) -> &mut [MultiRowCsvMetricsFile] {
        self.0.as_mut_slice()
    }
}

#[db_json]
pub struct JsonMetricsFile {
    #[garde(pattern(r#"^[0-9A-Za-z_-]*summary\.json$"#))]
    pub filename: String,
    #[garde(dive)]
    pub raw_contents: ValidString,
    #[serde(skip)]
    pub contents: AnyValue,
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = dataset))]
pub struct NewChromiumDatasetCore {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub inner: NewDatasetCommon,
    pub gems_id: Uuid,
    #[garde(custom(validate_html))]
    pub web_summary: String,
}

#[cfg_attr(feature = "python", pyclass)]
#[to_json(python)]
#[base_api_model]
pub struct CellrangerarcvdjCountDataset {
    #[serde(flatten)]
    #[garde(dive)]
    pub core: NewChromiumDatasetCore,
    pub metrics: SingleRowCsvMetricsFile,
}

#[cfg_attr(feature = "python", pyclass)]
#[to_json(python)]
#[base_api_model]
pub struct CellrangerMultiDataset {
    #[serde(flatten)]
    #[garde(dive)]
    pub core: NewChromiumDatasetCore,
    #[garde(dive)]
    pub metrics: MultiRowCsvMetricsFileGroup,
}
#[cfg_attr(feature = "python", pyclass)]
#[to_json(python)]
#[base_api_model]
pub struct CellrangeratacCountDataset {
    #[serde(flatten)]
    #[garde(dive)]
    pub core: NewChromiumDatasetCore,
    #[garde(dive)]
    pub metrics: JsonMetricsFile,
}

#[base_api_model]
#[derive(strum::IntoStaticStr)]
#[serde(tag = "cmdline")]
pub enum NewChromiumDataset {
    #[serde(rename = "cellranger-arc count")]
    #[strum(serialize = "cellranger-arc count")]
    CellrangerarcCount(CellrangerarcvdjCountDataset),

    #[serde(rename = "cellranger-atac count")]
    #[strum(serialize = "cellranger-atac count")]
    CellrangeratacCount(CellrangeratacCountDataset),

    #[serde(rename = "cellranger count")]
    #[strum(serialize = "cellranger count")]
    CellrangerCount(CellrangerarcvdjCountDataset),

    #[serde(rename = "cellranger multi")]
    #[strum(serialize = "cellranger multi")]
    CellrangerMulti(CellrangerMultiDataset),

    #[serde(rename = "cellranger vdj")]
    #[strum(serialize = "cellranger vdj")]
    CellrangerVdj(CellrangerarcvdjCountDataset),
}

fn validate_html(document: &str, _: &()) -> garde::Result {
    let result = scraper::Html::parse_document(document);
    if !result.errors.is_empty() {
        return Err(garde::Error::new("invalid HTML"));
    }

    Ok(())
}
