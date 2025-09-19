#[cfg(feature = "app")]
use diesel::prelude::*;
#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3_stub_gen::derive::{gen_stub_pyclass_complex_enum, gen_stub_pymethods};
use scamplers_macros::{
    Jsonify, PyJsonify, WasmJsonify, base_model, db_insertion, db_query, db_selection,
};
#[cfg(feature = "app")]
use scamplers_schema::cdna_preparers;
use time::OffsetDateTime;
use uuid::Uuid;
use valid_string::ValidString;

use crate::{
    db::models::{
        DefaultVec, Links, Pagination, nucleic_acid::common::ElectrophoreticMeasurementData,
        tenx_assay::chromium::LibraryType,
    },
    define_ordering_enum, uuid_newtype,
};

#[cfg(feature = "app")]
mod create;
#[cfg(feature = "app")]
mod read;

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::cdna_measurement))]
pub struct NewCdnaMeasurement {
    #[serde(default)]
    #[builder(default)]
    pub cdna_id: Uuid,
    pub measured_by: Uuid,
    #[serde(flatten)]
    #[garde(dive)]
    pub data: ElectrophoreticMeasurementData,
}

#[cfg(feature = "python")]
#[gen_stub_pymethods]
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
#[gen_stub_pymethods]
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

// We can get 16 libraries from one OCM GEMs by having 4 samples each with the
// following 4 library types:
// - Gene Expression
// - Antibody Capture
// - CRISPR
// - V(D)J

#[base_model]
#[serde(tag = "group_type", rename_all = "snake_case")]
#[cfg_attr(feature = "python", gen_stub_pyclass_complex_enum)]
#[cfg_attr(
    feature = "python",
    pyclass(eq, get_all, set_all, module = "scamplepy.create")
)]
#[derive(Jsonify, PyJsonify)]
pub enum NewCdnaGroup {
    Single {
        #[garde(dive)]
        cdna: NewCdna,
    },
    Multiple {
        #[garde(custom(validate_same_gems_ids), dive, length(max = 4))]
        cdna: Vec<NewCdna>,
    },
    OnChipMultiplexing {
        #[garde(custom(validate_same_gems_ids), dive, length(max = 16))]
        cdna: Vec<NewCdna>,
    },
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn validate_same_gems_ids(cdna: &[NewCdna], _: &()) -> garde::Result {
    let Some(first_cdna) = cdna.first() else {
        return Ok(());
    };

    if cdna.iter().any(|c| c.gems_id != first_cdna.gems_id) {
        return Err(garde::Error::new(
            "all cDNA in a group must come from the same GEMs",
        ));
    }

    Ok(())
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

#[cfg(feature = "app")]
#[derive(Insertable, Identifiable, Associations, Selectable, Queryable)]
#[diesel(primary_key(cdna_id, prepared_by), belongs_to(CdnaSummary, foreign_key = cdna_id))]
struct CdnaPreparer {
    cdna_id: Uuid,
    prepared_by: Uuid,
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
    pub prepared_by: Vec<Uuid>,
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

#[cfg(feature = "python")]
#[gen_stub_pymethods]
#[pymethods]
impl CdnaQuery {
    #[new]
    #[pyo3(signature = (*, ids = Vec::new(), order_by = DefaultVec::default(), limit = Pagination::default().limit, offset = Pagination::default_offset()))]
    fn new(ids: Vec<Uuid>, order_by: DefaultVec<CdnaOrderBy>, limit: i64, offset: i64) -> Self {
        Self {
            ids,
            order_by,
            pagination: Pagination { limit, offset },
        }
    }
}

uuid_newtype!(CdnaId);
