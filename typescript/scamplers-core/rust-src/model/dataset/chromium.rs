use std::collections::HashMap;

use any_value::AnyValue;
use scamplers_macros::{base_api_model, db_insertion, db_json};
#[cfg(feature = "backend")]
use scamplers_schema::dataset;
use uuid::Uuid;
use valid_string::ValidString;

use crate::model::dataset::common::NewDatasetCommon;

#[db_json]
pub struct SingleRowCsv {
    #[garde(pattern(r#"^[0-9A-Za-z_-]*summary\.csv$"#))]
    pub filename: String,
    #[garde(dive)]
    pub raw_contents: ValidString,
    #[serde(skip)]
    pub contents: HashMap<String, AnyValue>,
}

#[db_json]
pub struct MultiRowCsv {
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
pub struct MultiRowCsvGroup(#[garde(dive, length(min = 1))] Vec<MultiRowCsv>);
impl MultiRowCsvGroup {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[db_json]
pub struct Json {
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

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = dataset))]
pub struct CellrangerarcvdjCountDataset {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub core: NewChromiumDatasetCore,
    pub metrics: SingleRowCsv,
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = dataset))]
pub struct CellrangerMultiDataset {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub core: NewChromiumDatasetCore,
    #[garde(dive)]
    pub metrics: MultiRowCsvGroup,
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = dataset))]
pub struct CellrangeratacCountDataset {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub core: NewChromiumDatasetCore,
    #[garde(dive)]
    pub metrics: Json,
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

// fn deserialize_csv<'de, D>(deserializer: D) -> Result<Vec<HashMap<String,
// AnyValue>>, D::Error> where
//     D: Deserializer<'de>,
// {
//     let raw = String::deserialize(deserializer)?;
//     let rdr = csv::Reader::from_reader(raw.as_bytes());
//     let records = rdr.into_deserialize();
//     let records: csv::Result<Vec<HashMap<String, AnyValue>>> =
// records.collect();

//     records.map_err(serde::de::Error::custom)
// }

// fn deserialize_10x_single_row_csv<'de, D>(
//     deserializer: D,
// ) -> Result<HashMap<String, AnyValue>, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let mut records = deserialize_csv(deserializer)?;

//     if records.len() != 1 {
//         return Err(serde::de::Error::custom(
//             "expected CSV with exactly one row",
//         ));
//     }

//     let map = records.remove(0);

//     Ok(map.parse())
// }

// fn deserialize_10x_multi_row_csv<'de, D>(
//     deserializer: D,
// ) -> Result<Vec<HashMap<String, AnyValue>>, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let records = deserialize_csv(deserializer)?;
//     Ok(records.into_iter().map(TenxCsv::parse).collect())
// }

// fn deserialize_10x_json<'de, D>(deserializer: D) -> Result<AnyValue,
// D::Error> where
//     D: Deserializer<'de>,
// {
//     let raw_data = String::deserialize(deserializer)?;
//     serde_json::from_str(&raw_data).map_err(serde::de::Error::custom)
// }

// #[cfg(feature = "backend")]
// trait TenxCsv {
//     fn parse(self) -> HashMap<String, AnyValue>;
// }

// #[cfg(feature = "backend")]
// impl TenxCsv for HashMap<String, AnyValue> {
//     fn parse(self) -> HashMap<String, AnyValue> {
//         let mut new_map = HashMap::with_capacity(self.len());

//         let number_regex = Regex::new(r"^(\d+)\s\(\d{1,3}\.\d+\)$").unwrap();

//         for (key, mut value) in self {
//             let key = heck::AsSnakeCase(key).to_string();

//             // if we were able to parse it as a non-string, return that
//             if !value.is_string() {
//                 new_map.insert(key, value);
//                 continue;
//             }

//             // if not, convert it to a string and remove the comma
//             let value_as_string = value.to_string();
//             let formatted = value_as_string.replace([',', '%', '"'], "");

//             let matches = number_regex.captures(&formatted);

//             let extracted_number = match matches {
//                 Some(capture_group) => {
//                     let (_, [number]) = capture_group.extract();
//                     number
//                 }
//                 None => &formatted,
//             };

//             if let Ok(n) = serde_json::Number::from_str(extracted_number) {
//                 // if the original string had a '%' in it, we want to divide
// by 100                 if value_as_string.contains('%') && extracted_number
// == formatted {                     value = AnyValue::from(n.as_f64().unwrap()
// / 100.0);                 } else {
//                     value = AnyValue::from(n.as_f64().unwrap());
//                 }
//             }

//             new_map.insert(key, value);
//         }

//         new_map
//     }
// }

fn validate_html(document: &str, _: &()) -> garde::Result {
    let result = scraper::Html::parse_document(document);
    if !result.errors.is_empty() {
        return Err(garde::Error::new("invalid HTML"));
    }

    Ok(())
}
