#[cfg(feature = "app")]
use diesel::prelude::*;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{
    Jsonify, PyJsonify, WasmJsonify, base_model, db_insertion, db_query, db_selection,
};
use time::OffsetDateTime;
use uuid::Uuid;
use valid_string::ValidString;

use crate::{
    db::models::{
        DefaultVec, Links, Pagination, library_type_specification::LibraryType,
        nucleic_acid::common::ElectrophoreticMeasurementData,
    },
    define_ordering_enum, uuid_newtype,
};

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::cdna_measurement))]
pub struct NewCdnaMeasurement {
    #[serde(default)]
    pub cdna_id: Uuid,
    pub measured_by: Uuid,
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
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

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::cdna))]
pub struct NewCdna {
    pub library_type: LibraryType,
    pub readable_id: ValidString,
    pub prepared_at: OffsetDateTime,
    pub gems_id: Uuid,
    #[garde(range(min = 1))]
    pub n_amplification_cycles: i32,
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
    pub volume_µl: f32,
    #[garde(length(min = 1))]
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
    pub preparer_ids: Vec<Uuid>,
    #[garde(dive)]
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
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
    #[pyo3(signature = (*, library_type, readable_id, prepared_at, gems_id, n_amplification_cycles, volume_mcl, preparer_ids, measurements=Vec::new(), storage_location=None, notes=None))]
    fn new(
        library_type: LibraryType,
        readable_id: ValidString,
        prepared_at: OffsetDateTime,
        gems_id: Uuid,
        n_amplification_cycles: i32,
        volume_mcl: f32, // name change due to python
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
            volume_µl: volume_mcl,
            preparer_ids,
            measurements,
            storage_location,
            notes,
        }
    }
}

#[base_model]
#[serde(tag = "group_type")]
#[cfg_attr(feature = "python", derive(FromPyObject))]
pub enum NewCdnaGroup {
    Single(#[garde(dive)] NewCdna),
    Multiple(#[garde(dive)] Vec<NewCdna>),
    Ocm(#[garde(length(min = 1, max = 4))] Vec<NewCdna>),
}

#[db_selection]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::cdna))]
pub struct CdnaSummary {
    pub id: Uuid,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(readonly))]
    pub links: Links,
    pub library_type: LibraryType,
    pub readable_id: String,
    pub prepared_at: OffsetDateTime,
    pub n_amplification_cycles: i32,
    pub storage_location: Option<String>,
    pub notes: Option<String>,
}

#[db_selection]
#[cfg_attr(feature = "app", derive(Associations))]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::cdna_measurement, belongs_to(CdnaSummary, foreign_key = cdna_id)))]
pub struct CdnaMeasurement {
    pub id: Uuid,
    pub cdna_id: Uuid,
    pub measured_by: Uuid,
    #[serde(flatten)]
    pub data: ElectrophoreticMeasurementData,
}

#[base_model]
#[cfg_attr(
    target_arch = "wasm32",
    ::wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)
)]
#[cfg_attr(feature = "python", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[cfg_attr(
    feature = "python",
    pyclass(eq, get_all, module = "scamplepy.responses")
)]
#[derive(Jsonify, WasmJsonify, PyJsonify)]
pub struct Cdna {
    pub summary: CdnaSummary,
    pub measurements: Vec<CdnaMeasurement>,
}

define_ordering_enum! { CdnaOrderBy { PreparedAt, ReadableId }, default = PreparedAt }

#[db_query]
pub struct CdnaQuery {
    #[builder(default)]
    pub ids: Vec<Uuid>,
    #[builder(default)]
    pub order_by: DefaultVec<CdnaOrderBy>,
    #[builder(default)]
    pub pagination: Pagination,
}

uuid_newtype!(CdnaId);
