#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{base_api_model, db_insertion, db_selection, to_from_json};
#[cfg(feature = "backend")]
use scamplers_schema::{cdna, cdna_measurement, cdna_preparers};
use time::OffsetDateTime;
use uuid::Uuid;
use valid_string::ValidString;
#[cfg(feature = "python")]
use {
    super::common::{Concentration, ElectrophoreticSizingRange},
    crate::model::units::{MassUnit, VolumeUnit},
};

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

#[cfg(feature = "python")]
#[pymethods]
impl NewCdnaMeasurement {
    #[new]
    #[pyo3(signature = (measured_by, measured_at, instrument_name, mean_library_size_bp, sizing_range, concentration_value, concentration_unit, cdna_id=Uuid::default()))]
    fn new(
        measured_by: Uuid,
        measured_at: OffsetDateTime,
        instrument_name: ValidString,
        mean_library_size_bp: f32,
        sizing_range: (i32, i32),
        concentration_value: f32,
        concentration_unit: (MassUnit, VolumeUnit),
        cdna_id: Uuid,
    ) -> Self {
        Self {
            cdna_id,
            measured_by,
            data: ElectrophoreticMeasurementData {
                measured_at,
                instrument_name,
                mean_library_size_bp,
                sizing_range: ElectrophoreticSizingRange(sizing_range.0, sizing_range.1),
                concentration: Concentration {
                    value: concentration_value,
                    unit: concentration_unit,
                },
            },
        }
    }
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

#[cfg(feature = "python")]
#[pymethods]
impl NewCdna {
    #[new]
    fn new(
        library_type: LibraryType,
        readable_id: ValidString,
        prepared_at: OffsetDateTime,
        gems_id: Uuid,
        n_amplification_cycles: i32,
        measurements: Vec<NewCdnaMeasurement>,
        preparer_ids: Vec<Uuid>,
        storage_location: Option<ValidString>,
        notes: Option<ValidString>,
    ) -> Self {
        Self {
            library_type,
            readable_id,
            prepared_at,
            gems_id,
            n_amplification_cycles,
            storage_location,
            notes,
            measurements,
            preparer_ids,
        }
    }
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
