#[cfg(feature = "app")]
use diesel::prelude::*;
#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3_stub_gen::derive::{gen_stub_pyclass_complex_enum, gen_stub_pymethods};
use scamplers_macros::{
    Jsonify, PyJsonify, WasmJsonify, base_model, db_insertion, db_json, db_query, db_selection,
};
#[cfg(feature = "app")]
use scamplers_schema::library_preparers;
use time::OffsetDateTime;
use uuid::Uuid;
use valid_string::ValidString;

use crate::{
    db::models::{
        DefaultVec, Links, Pagination,
        nucleic_acid::{
            cdna::CdnaSummary,
            common::{Concentration, ElectrophoreticMeasurementData},
        },
        tenx_assay::chromium::LibraryType,
    },
    define_ordering_enum, uuid_newtype,
};

#[cfg(feature = "app")]
mod create;
#[cfg(feature = "app")]
mod read;

#[cfg_attr(feature = "python", gen_stub_pyclass_complex_enum)]
#[db_json]
#[serde(tag = "type")]
#[cfg_attr(
    feature = "python",
    pyo3(name = "LibraryMeasurementData", module = "scamplepy.common")
)]
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

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::library_measurement))]
pub struct NewLibraryMeasurement {
    #[serde(default)]
    pub library_id: Uuid,
    pub measured_by: Uuid,
    #[serde(flatten)]
    #[garde(dive)]
    pub data: MeasurementData,
}

#[cfg(feature = "python")]
#[gen_stub_pymethods]
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

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::library))]
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
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
    pub volume_µl: f32,
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
    #[pyo3(signature = (*, readable_id, cdna_id, number_of_sample_index_pcr_cycles, volume_µl, target_reads_per_cell, prepared_at, preparer_ids, single_index_set_name=None, dual_index_set_name=None, measurements=Vec::new(), notes=None))]
    fn new(
        readable_id: ValidString,
        cdna_id: Uuid,
        number_of_sample_index_pcr_cycles: i32,
        volume_µl: f32,
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
            volume_µl,
            target_reads_per_cell,
            prepared_at,
            preparer_ids,
            measurements,
            notes,
        }
    }
}

#[db_selection]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::library))]
pub struct LibrarySummary {
    pub id: Uuid,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(readonly))]
    pub links: Links,
    pub readable_id: String,
    pub single_index_set_name: Option<String>,
    pub dual_index_set_name: Option<String>,
    pub number_of_sample_index_pcr_cycles: i32,
    pub target_reads_per_cell: i32,
    pub prepared_at: OffsetDateTime,
    pub notes: Option<String>,
}

#[db_selection]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::library))]
pub struct LibrarySummaryWithParents {
    #[cfg_attr(feature = "app", diesel(column_name = id))]
    pub id_: Uuid,
    #[serde(flatten)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub summary: LibrarySummary,
    #[cfg_attr(feature = "app", diesel(embed))]
    pub cdna: CdnaSummary,
}

#[cfg(feature = "app")]
#[derive(Insertable, Identifiable, Associations, Selectable, Queryable)]
#[diesel(primary_key(library_id, prepared_by), belongs_to(LibrarySummaryWithParents, foreign_key = library_id))]
struct LibraryPreparer {
    library_id: Uuid,
    prepared_by: Uuid,
}

#[db_selection]
#[cfg_attr(feature = "app", derive(Associations))]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::library_measurement, belongs_to(LibrarySummaryWithParents, foreign_key = library_id)))]
pub struct LibraryMeasurement {
    pub id: Uuid,
    pub library_id: Uuid,
    pub measured_by: Uuid,
    #[serde(flatten)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))] // TODO: custom-getter
    pub data: MeasurementData,
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
pub struct Library {
    pub info: LibrarySummaryWithParents,
    pub prepared_by: Vec<Uuid>,
    pub measurements: Vec<LibraryMeasurement>,
}

define_ordering_enum! { LibraryOrderBy{ PreparedAt, ReadableId }, default = PreparedAt }

#[db_query]
pub struct LibraryQuery {
    #[builder(default)]
    pub ids: Vec<Uuid>,
    #[builder(default)]
    pub library_types: Vec<LibraryType>,
    #[builder(default)]
    pub pagination: Pagination,
    #[builder(default)]
    pub order_by: DefaultVec<LibraryOrderBy>,
}

uuid_newtype!(LibraryId);
