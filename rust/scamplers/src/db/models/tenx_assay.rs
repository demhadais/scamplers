use scamplers_macros::db_selection;
use uuid::Uuid;

pub mod chromium;
mod common;
#[cfg(feature = "app")]
pub mod create;

#[db_selection]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::tenx_assay))]
pub struct TenxAssay {
    pub id: Uuid,
    pub name: String,
    pub library_types: Option<Vec<Option<String>>>,
    pub sample_multiplexing: Option<String>,
    pub chemistry_version: String,
    pub protocol_url: String,
    pub chromium_chip: Option<String>,
    pub cmdline: Option<String>,
}
