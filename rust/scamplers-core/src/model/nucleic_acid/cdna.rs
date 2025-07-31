#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{base_api_model, db_insertion, db_selection};
#[cfg(feature = "backend")]
use scamplers_schema::{cdna, cdna_measurement, cdna_preparers};
use time::OffsetDateTime;
use uuid::Uuid;
use valid_string::ValidString;

use crate::model::{
    library_type_specification::LibraryType, nucleic_acid::common::ElectrophoreticMeasurementData,
};

#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(scamplers_macros::FromJson, scamplers_macros::ToJson)
)]
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
    #[pyo3(signature = (*, measured_by, data, cdna_id=Uuid::default()))]
    fn new(measured_by: Uuid, data: ElectrophoreticMeasurementData, cdna_id: Uuid) -> Self {
        Self {
            cdna_id,
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
#[cfg_attr(feature = "backend", diesel(table_name = cdna))]
pub struct NewCdna {
    pub library_type: LibraryType,
    pub readable_id: ValidString,
    pub prepared_at: OffsetDateTime,
    pub gems_id: Uuid,
    #[garde(range(min = 1))]
    pub n_amplification_cycles: i32,
    #[garde(length(min = 1))]
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    pub preparer_ids: Vec<Uuid>,
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    pub measurements: Vec<NewCdnaMeasurement>,
    #[garde(dive)]
    pub storage_location: Option<ValidString>,
    #[garde(dive)]
    pub notes: Option<ValidString>,
}

#[cfg(feature = "python")]
#[pymethods]
impl NewCdna {
    #[new]
    #[pyo3(signature = (*, library_type, readable_id, prepared_at, gems_id, n_amplification_cycles, preparer_ids, measurements=Vec::new(), storage_location=None, notes=None))]
    fn new(
        library_type: LibraryType,
        readable_id: ValidString,
        prepared_at: OffsetDateTime,
        gems_id: Uuid,
        n_amplification_cycles: i32,
        preparer_ids: Vec<Uuid>,
        measurements: Vec<NewCdnaMeasurement>,
        storage_location: Option<ValidString>,
        notes: Option<ValidString>,
    ) -> Self {
        Self {
            library_type,
            readable_id,
            prepared_at,
            gems_id,
            n_amplification_cycles,
            preparer_ids,
            measurements,
            storage_location,
            notes,
        }
    }
}

#[base_api_model]
#[serde(tag = "group_type")]
#[cfg_attr(feature = "python", pyclass(str, get_all, set_all))]
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

#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(scamplers_macros::FromJson, scamplers_macros::ToJson)
)]
#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = cdna))]
pub struct CdnaHandle {
    pub id: Uuid,
    pub link: ValidString,
}
