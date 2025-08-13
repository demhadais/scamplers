#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{db_insertion, db_json, db_selection};
#[cfg(feature = "backend")]
use scamplers_schema::{library, library_measurement, library_preparers};
use time::OffsetDateTime;
use uuid::Uuid;
use valid_string::ValidString;

use crate::model::nucleic_acid::common::{Concentration, ElectrophoreticMeasurementData};

#[db_json]
#[serde(tag = "type")]
#[cfg_attr(feature = "python", pyo3(name = "LibraryMeasurementData", set_all))]
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

#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(scamplers_macros::FromJson, scamplers_macros::ToJson)
)]
#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = library_measurement))]
pub struct NewLibraryMeasurement {
    #[serde(default)]
    pub library_id: Uuid,
    pub measured_by: Uuid,
    #[serde(flatten)]
    #[garde(dive)]
    pub data: MeasurementData,
}

#[cfg(feature = "python")]
#[pymethods]
impl NewLibraryMeasurement {
    #[new]
    #[pyo3(signature = (*, measured_by, data, library_id=Uuid::default()))]
    fn new(measured_by: Uuid, data: MeasurementData, library_id: Uuid) -> Self {
        Self {
            library_id,
            measured_by,
            data,
        }
    }
}

#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(scamplers_macros::FromJson, scamplers_macros::ToJson)
)]
#[cfg_attr(not(target_arch = "wasm32"), json(python))]
#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = library))]
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
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
    #[garde(length(min = 1))]
    pub preparer_ids: Vec<Uuid>,
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
    pub measurements: Vec<NewLibraryMeasurement>,
    #[garde(dive)]
    pub notes: Option<ValidString>,
}

#[cfg(feature = "python")]
#[pymethods]
impl NewLibrary {
    #[new]
    #[pyo3(signature = (*, readable_id, cdna_id, number_of_sample_index_pcr_cycles, target_reads_per_cell, prepared_at, preparer_ids, single_index_set_name=None, dual_index_set_name=None, measurements=Vec::new(), notes=None))]
    fn new(
        readable_id: ValidString,
        cdna_id: Uuid,
        number_of_sample_index_pcr_cycles: i32,
        target_reads_per_cell: i32,
        prepared_at: OffsetDateTime,
        preparer_ids: Vec<Uuid>,
        single_index_set_name: Option<ValidString>,
        dual_index_set_name: Option<ValidString>,
        measurements: Vec<NewLibraryMeasurement>,
        notes: Option<ValidString>,
    ) -> Self {
        Self {
            readable_id,
            cdna_id,
            single_index_set_name,
            dual_index_set_name,
            number_of_sample_index_pcr_cycles,
            target_reads_per_cell,
            prepared_at,
            preparer_ids,
            measurements,
            notes,
        }
    }
}

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = library_preparers))]
pub struct NewLibraryPreparer {
    pub library_id: Uuid,
    pub prepared_by: Uuid,
}

#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(scamplers_macros::FromJson, scamplers_macros::ToJson)
)]
#[db_selection]
#[cfg_attr(feature = "app", diesel(table_name = library))]
pub struct LibraryHandle {
    pub id: Uuid,
    pub link: String,
}
