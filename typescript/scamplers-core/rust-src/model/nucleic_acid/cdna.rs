use scamplers_macros::{db_insertion, db_selection};
use time::OffsetDateTime;
use uuid::Uuid;
use valid_string::ValidString;

use crate::model::{
    library_type_specification::LibraryType, nucleic_acid::common::ElectrophoreticMeasurementData,
};

#[cfg(feature = "backend")]
use scamplers_schema::{cdna, cdna_measurement, cdna_preparers};

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

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = cdna_preparers))]
pub struct NewCdnaPreparer {
    pub cdna_id: Uuid,
    pub prepared_by: Uuid,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = cdna))]
pub struct CdnaHandle {
    pub id: Uuid,
    pub link: ValidString,
}
