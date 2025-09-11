use scamplers_macros::{db_selection, db_simple_enum};
use uuid::Uuid;
use valid_string::ValidString;

use crate::uuid_newtype;

#[cfg(feature = "app")]
mod create;
#[cfg(feature = "app")]
mod read;

#[db_simple_enum]
#[cfg_attr(feature = "python", pyo3(module = "scamplepy.create"))]
pub enum MultiplexingTagType {
    FlexBarcode,
    OnChipMultiplexing,
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

#[scamplers_macros::base_model]
#[cfg_attr(
    feature = "app",
    derive(diesel::Insertable),
    diesel(check_for_backend(::diesel::pg::Pg), table_name = scamplers_schema::multiplexing_tag)
)]
#[derive(scamplers_macros::Jsonify, bon::Builder)]
#[builder(on(_, into), derive(Clone, Debug, Into))]
pub struct NewMultiplexingTag {
    #[garde(dive)]
    pub tag_id: ValidString,
    pub type_: MultiplexingTagType,
}

#[db_selection]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::multiplexing_tag))]
pub struct MultiplexingTag {
    pub id: Uuid,
    pub tag_id: String,
    pub type_: String,
}

uuid_newtype!(MultiplexingTagId);
