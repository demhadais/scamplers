use std::collections::HashMap;

use any_value::AnyValue;
use derive_more::{From, TryInto};
#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{FromJson, ToJson, base_api_model, db_insertion, db_json};
#[cfg(feature = "backend")]
use scamplers_schema::dataset;
#[cfg(feature = "python")]
use time::OffsetDateTime;
use uuid::Uuid;
use valid_string::ValidString;

use crate::model::dataset::common::NewDatasetCommon;

#[cfg(feature = "python")]
macro_rules! impl_metrics_methods {
    ($metrics_struct:ident) => {
        #[pymethods]
        impl $metrics_struct {
            #[new]
            #[pyo3(signature = (*, filename, raw_contents))]
            fn new(filename: String, raw_contents: ValidString) -> Self {
                Self {
                    filename,
                    raw_contents,
                    contents: Default::default(),
                }
            }

            #[setter]
            fn set_filename(&mut self, filename: String) {
                self.filename = filename;
            }

            #[setter]
            fn set_raw_contents(&mut self, raw_contents: ValidString) {
                self.raw_contents = raw_contents
            }
        }
    };
}

#[db_json]
pub struct SingleRowCsvMetricsFile {
    #[garde(pattern(r#"^[0-9A-Za-z_-]*summary\.csv$"#))]
    pub filename: String,
    #[garde(dive)]
    pub raw_contents: ValidString,
    #[serde(skip)]
    pub contents: HashMap<String, AnyValue>,
}

#[cfg(feature = "python")]
impl_metrics_methods!(SingleRowCsvMetricsFile);

#[db_json]
pub struct MultiRowCsvMetricsFile {
    #[garde(pattern(r#"^[0-9A-Za-z_-]+/[[:alnum:]]+summary\.csv$"#))]
    pub filename: String,
    #[garde(dive)]
    pub raw_contents: ValidString,
    #[serde(skip)]
    pub contents: Vec<HashMap<String, AnyValue>>,
}

#[cfg(feature = "python")]
impl_metrics_methods!(MultiRowCsvMetricsFile);

#[db_json]
#[serde(transparent)]
#[garde(transparent)]
#[cfg_attr(
    feature = "python",
    pyo3(sequence, name = "_MultiRowCsvMetricsFileGroup")
)]
pub struct MultiRowCsvMetricsFileGroup {
    #[garde(dive, length(min = 1))]
    inner: Vec<MultiRowCsvMetricsFile>,
}

impl MultiRowCsvMetricsFileGroup {
    #[must_use]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn as_mut_slice(&mut self) -> &mut [MultiRowCsvMetricsFile] {
        self.inner.as_mut_slice()
    }
}

impl From<Vec<MultiRowCsvMetricsFile>> for MultiRowCsvMetricsFileGroup {
    fn from(inner: Vec<MultiRowCsvMetricsFile>) -> Self {
        Self { inner }
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

#[cfg(feature = "python")]
impl_metrics_methods!(JsonMetricsFile);

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = dataset))]
#[cfg_attr(feature = "python", pyo3(name = "_NewChromiumDatasetCore"))]
pub struct NewChromiumDatasetCore {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub inner: NewDatasetCommon,
    pub gems_id: Uuid,
    #[garde(custom(validate_html))]
    pub web_summary: String,
}

#[cfg(feature = "python")]
macro_rules! impl_dataset_constructor {
    ($dataset_struct:path, $metrics_struct:path) => {
        #[pymethods]
        impl $dataset_struct {
            #[new]
            #[pyo3(signature = (*, name, lab_id, data_path, delivered_at, gems_id, web_summary, metrics))]
            fn new(
                name: ValidString,
                lab_id: Uuid,
                data_path: ValidString,
                delivered_at: OffsetDateTime,
                gems_id: Uuid,
                web_summary: String,
                metrics: $metrics_struct,
            ) -> Self {
                Self {
                    core: NewChromiumDatasetCore {
                        inner: NewDatasetCommon {
                            name,
                            lab_id,
                            data_path,
                            delivered_at,
                        },
                        gems_id,
                        web_summary,
                    },
                    metrics: metrics.into(),
                }
            }
        }
    };
}

#[base_api_model]
#[cfg_attr(feature = "python", pyclass(get_all, set_all, str, eq))]
#[derive(FromJson, ToJson)]
#[cfg_attr(not(target_arch = "wasm32"), json(wrapper = NewChromiumDataset, python))]
pub struct CellrangerarcCountDataset {
    #[serde(flatten)]
    #[garde(dive)]
    pub core: NewChromiumDatasetCore,
    pub metrics: SingleRowCsvMetricsFile,
}
#[cfg(feature = "python")]
impl_dataset_constructor!(CellrangerarcCountDataset, SingleRowCsvMetricsFile);

#[base_api_model]
#[cfg_attr(feature = "python", pyclass(get_all, set_all, str, eq))]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(scamplers_macros::FromJson, scamplers_macros::ToJson)
)]
#[cfg_attr(not(target_arch = "wasm32"), json(wrapper = NewChromiumDataset, python))]
pub struct CellrangeratacCountDataset {
    #[serde(flatten)]
    #[garde(dive)]
    pub core: NewChromiumDatasetCore,
    #[garde(dive)]
    pub metrics: JsonMetricsFile,
}
#[cfg(feature = "python")]
impl_dataset_constructor!(CellrangeratacCountDataset, JsonMetricsFile);

#[base_api_model]
#[cfg_attr(feature = "python", pyclass(get_all, set_all, str, eq))]
#[derive(FromJson, ToJson)]
#[cfg_attr(not(target_arch = "wasm32"), json(wrapper = NewChromiumDataset, python))]
pub struct CellrangerCountDataset {
    #[serde(flatten)]
    #[garde(dive)]
    pub core: NewChromiumDatasetCore,
    pub metrics: SingleRowCsvMetricsFile,
}
#[cfg(feature = "python")]
impl_dataset_constructor!(CellrangerCountDataset, SingleRowCsvMetricsFile);

#[base_api_model]
#[cfg_attr(feature = "python", pyclass(get_all, set_all, str, eq))]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(scamplers_macros::FromJson, scamplers_macros::ToJson)
)]
#[cfg_attr(not(target_arch = "wasm32"), json(wrapper = NewChromiumDataset, python))]
pub struct CellrangerMultiDataset {
    #[serde(flatten)]
    #[garde(dive)]
    pub core: NewChromiumDatasetCore,
    #[garde(dive)]
    pub metrics: MultiRowCsvMetricsFileGroup,
}
#[cfg(feature = "python")]
impl_dataset_constructor!(CellrangerMultiDataset, Vec<MultiRowCsvMetricsFile>);

#[base_api_model]
#[cfg_attr(feature = "python", pyclass(get_all, set_all, str, eq))]
#[derive(FromJson, ToJson)]
#[cfg_attr(not(target_arch = "wasm32"), json(wrapper = NewChromiumDataset, python))]
pub struct CellrangerVdjDataset {
    #[serde(flatten)]
    #[garde(dive)]
    pub core: NewChromiumDatasetCore,
    pub metrics: SingleRowCsvMetricsFile,
}
#[cfg(feature = "python")]
impl_dataset_constructor!(CellrangerVdjDataset, SingleRowCsvMetricsFile);

#[base_api_model]
#[derive(strum::IntoStaticStr, TryInto, From)]
#[serde(tag = "cmdline")]
#[cfg_attr(feature = "python", pyclass(get_all, set_all, str))]
pub enum NewChromiumDataset {
    #[serde(rename = "cellranger-arc count")]
    #[strum(serialize = "cellranger-arc count")]
    CellrangerarcCount(CellrangerarcCountDataset),

    #[serde(rename = "cellranger-atac count")]
    #[strum(serialize = "cellranger-atac count")]
    CellrangeratacCount(CellrangeratacCountDataset),

    #[serde(rename = "cellranger count")]
    #[strum(serialize = "cellranger count")]
    CellrangerCount(CellrangerCountDataset),

    #[serde(rename = "cellranger multi")]
    #[strum(serialize = "cellranger multi")]
    CellrangerMulti(CellrangerMultiDataset),

    #[serde(rename = "cellranger vdj")]
    #[strum(serialize = "cellranger vdj")]
    CellrangerVdj(CellrangerVdjDataset),
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn validate_html(document: &str, _: &()) -> garde::Result {
    let result = scraper::Html::parse_document(document);
    if !result.errors.is_empty() {
        return Err(garde::Error::new("invalid HTML"));
    }

    Ok(())
}
