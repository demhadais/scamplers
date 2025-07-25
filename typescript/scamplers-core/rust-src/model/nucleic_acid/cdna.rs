use scamplers_macros::{base_api_model, db_insertion, db_selection, to_from_json};
#[cfg(feature = "backend")]
use scamplers_schema::{cdna, cdna_measurement, cdna_preparers};
use time::OffsetDateTime;
use uuid::Uuid;
use valid_string::ValidString;

use crate::model::{
    library_type_specification::LibraryType, nucleic_acid::common::ElectrophoreticMeasurementData,
};

#[to_from_json(python)]
#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = cdna_measurement))]
pub struct NewCdnaMeasurement {
    #[serde(default)]
    pub cdna_id: Uuid,
    pub measured_by: Uuid,
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    pub data: ElectrophoreticMeasurementData,
}

#[to_from_json(python)]
#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = cdna))]
pub struct NewCdna {
    pub library_type: LibraryType,
    pub readable_id: ValidString,
    pub prepared_at: OffsetDateTime,
    pub gems_id: Uuid,
    #[garde(range(min = 1))]
    pub n_amplification_cycles: i32,
    #[garde(dive)]
    pub storage_location: Option<ValidString>,
    #[garde(dive)]
    pub notes: Option<ValidString>,
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    pub measurements: Vec<NewCdnaMeasurement>,
    #[garde(length(min = 1))]
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    pub preparer_ids: Vec<Uuid>,
}

#[base_api_model]
#[serde(tag = "group_type")]
pub enum NewCdnaGroup {
    Single(#[garde(dive)] NewCdna),
    Multiple(#[garde(dive)] Vec<NewCdna>),
    Ocm(#[garde(length(min = 1, max = 4))] Vec<NewCdna>),
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = cdna_preparers))]
pub struct NewCdnaPreparer {
    pub cdna_id: Uuid,
    pub prepared_by: Uuid,
}

#[to_from_json(python)]
#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = cdna))]
pub struct CdnaHandle {
    pub id: Uuid,
    pub link: ValidString,
}
