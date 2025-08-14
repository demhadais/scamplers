use scamplers_macros::db_selection;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::db::models::Links;

#[db_selection]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::specimen))]
pub struct SpecimenSummary {
    pub id: Uuid,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(readonly))]
    pub links: Links,
    pub readable_id: String,
    pub name: String,
    pub received_at: OffsetDateTime,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub species: Vec<Option<String>>, // Option<String> doesn't implement VectorFromWasmAbi :)
    pub notes: Option<String>,
    pub returned_at: Option<OffsetDateTime>,
    pub type_: String,
    pub embedded_in: Option<String>,
    pub fixative: Option<String>,
    pub frozen: bool,
    pub cryopreserved: bool,
    pub storage_buffer: Option<String>,
}
