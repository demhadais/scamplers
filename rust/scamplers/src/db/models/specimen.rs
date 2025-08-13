use scamplers_macros::db_selection;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::db::models::Links;

#[db_selection]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::specimen))]
pub struct SpecimenSummary {
    pub id: Uuid,
    pub links: Links,
    pub readable_id: String,
    pub name: String,
    pub received_at: OffsetDateTime,
    pub species: Vec<Option<String>>,
    pub notes: Option<String>,
    pub returned_at: Option<OffsetDateTime>,
    pub type_: String,
    pub embedded_in: Option<String>,
    pub fixative: Option<String>,
    pub frozen: bool,
    pub cryopreserved: bool,
    pub storage_buffer: Option<String>,
}
