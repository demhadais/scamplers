use scamplers_macros::{db_insertion, db_json, db_selection, db_simple_enum};
#[cfg(feature = "app")]
use scamplers_schema::{multiplexing_tag, suspension, suspension_measurement};
use uuid::Uuid;
use valid_string::ValidString;

#[cfg(feature = "app")]
mod create;
#[cfg(feature = "app")]
mod read;

#[db_simple_enum]
pub enum MultiplexingTagType {
    FlexBarcode,
    Ocm,
    #[serde(rename = "TotalSeq-A")]
    #[strum(serialize = "TotalSeq-A")]
    TotalSeqA,
    #[serde(rename = "TotalSeq-B")]
    #[strum(serialize = "TotalSeq-B")]
    TotalSeqB,
    #[serde(rename = "TotalSeq-C")]
    #[strum(serialize = "TotalSeq-C")]
    TotalSeqC,
}

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = multiplexing_tag))]
pub struct NewMultiplexingTag {
    #[garde(dive)]
    pub tag_id: ValidString,
    pub type_: MultiplexingTagType,
}

#[db_selection]
#[cfg_attr(feature = "app", diesel(table_name = multiplexing_tag))]
pub struct MultiplexingTag {
    pub id: Uuid,
    pub tag_id: String,
    pub type_: String,
}
