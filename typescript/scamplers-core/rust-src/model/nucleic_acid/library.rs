use scamplers_macros::{db_insertion, db_json, db_selection, to_from_json};
#[cfg(feature = "backend")]
use scamplers_schema::{library, library_measurement, library_preparers};
use time::OffsetDateTime;
use uuid::Uuid;
use valid_string::ValidString;

use crate::model::nucleic_acid::common::{Concentration, ElectrophoreticMeasurementData};

#[db_json]
#[serde(tag = "type")]
pub enum MeasurementData {
    Electrophoretic(ElectrophoreticMeasurementData),
    Fluorometric {
        measured_at: OffsetDateTime,
        #[garde(dive)]
        instrument_name: ValidString,
        #[garde(dive)]
        concentration: Concentration,
    },
}

#[to_from_json(python)]
#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = library_measurement))]
pub struct NewLibraryMeasurement {
    #[serde(default)]
    pub library_id: Uuid,
    pub measured_by: Uuid,
    #[serde(flatten)]
    #[garde(dive)]
    pub data: MeasurementData,
}

#[to_from_json(python)]
#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = library))]
pub struct NewLibrary {
    #[garde(dive)]
    pub readable_id: ValidString,
    pub cdna_id: Uuid,
    #[garde(dive)]
    pub single_index_set_name: Option<ValidString>,
    #[garde(dive)]
    pub dual_index_set_name: Option<ValidString>,
    #[garde(range(min = 1))]
    pub number_of_sample_index_pcr_cycles: i32,
    #[garde(range(min = 1000))]
    pub target_reads_per_cell: i32,
    pub prepared_at: OffsetDateTime,
    #[garde(dive)]
    pub notes: Option<ValidString>,
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    pub measurements: Vec<NewLibraryMeasurement>,
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    #[garde(length(min = 1))]
    pub preparer_ids: Vec<Uuid>,
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = library_preparers))]
pub struct NewLibraryPreparer {
    pub library_id: Uuid,
    pub prepared_by: Uuid,
}

#[to_from_json(python)]
#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = library))]
pub struct LibraryHandle {
    pub id: Uuid,
    pub link: String,
}
