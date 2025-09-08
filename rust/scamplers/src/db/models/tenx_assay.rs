use scamplers_macros::{base_model, db_query, db_selection};
use uuid::Uuid;

use crate::{
    db::models::{
        DefaultVec, Pagination,
        tenx_assay::chromium::{LibraryType, NewChromiumAssay, SampleMultiplexing},
    },
    define_ordering_enum, uuid_newtype,
};

pub mod chromium;
mod common;
#[cfg(feature = "app")]
mod create;
#[cfg(feature = "app")]
mod read;

#[base_model]
#[serde(tag = "platform", rename_all = "snake_case")]
pub enum NewTenxAssay {
    Chromium(#[garde(dive)] NewChromiumAssay),
}

#[db_selection]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::tenx_assay))]
pub struct TenxAssay {
    pub id: Uuid,
    pub name: String,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub library_types: Option<Vec<Option<String>>>,
    pub sample_multiplexing: Option<String>,
    pub chemistry_version: String,
    pub protocol_url: String,
    pub chromium_chip: Option<String>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub cmdlines: Option<Vec<Option<String>>>,
}

define_ordering_enum! {TenxAssayOrderBy { Name }, default = Name}

#[db_query]
pub struct TenxAssayQuery {
    #[builder(default)]
    pub ids: Vec<Uuid>,
    #[builder(default)]
    pub names: Vec<String>,
    #[builder(default)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub library_types: Vec<Vec<LibraryType>>,
    #[builder(default)]
    pub sample_multiplexing: Vec<SampleMultiplexing>,
    #[builder(default)]
    pub chemistry_versions: Vec<String>,
    #[builder(default)]
    pub chromium_chips: Vec<String>,
    #[builder(default)]
    pub order_by: DefaultVec<TenxAssayOrderBy>,
    #[builder(default)]
    pub pagination: Pagination,
}

uuid_newtype!(TenxAssayId);
