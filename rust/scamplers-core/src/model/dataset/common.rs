use scamplers_macros::db_insertion;
#[cfg(feature = "backend")]
use scamplers_schema::dataset;
use time::OffsetDateTime;
use uuid::Uuid;
use valid_string::ValidString;

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = dataset))]
#[cfg_attr(feature = "python", pyo3(name = "_NewDatasetCommon"))]
pub struct NewDatasetCommon {
    #[garde(dive)]
    pub name: ValidString,
    pub lab_id: Uuid,
    #[garde(dive)]
    pub data_path: ValidString,
    pub delivered_at: OffsetDateTime,
}
