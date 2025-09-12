#[cfg(feature = "app")]
use diesel::{expression::AsExpression, prelude::*};
#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3_stub_gen::{derive::gen_stub_pymethods, impl_stub_type};
use scamplers_macros::{
    Jsonify, PyJsonify, WasmJsonify, base_model, db_query, db_selection, db_simple_enum,
};
#[cfg(feature = "app")]
use scamplers_schema::{lab, person, specimen};
use time::OffsetDateTime;
use uuid::Uuid;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::{
    db::models::{
        DefaultVec, Links, Pagination,
        lab::LabSummary,
        person::PersonSummary,
        specimen::{
            block::{
                BlockFixative, FixedBlockEmbeddingMatrix, FrozenBlockEmbeddingMatrix,
                NewFixedBlock, NewFrozenBlock,
            },
            tissue::{NewCryopreservedTissue, NewFixedTissue, NewFrozenTissue, TissueFixative},
            virtual_::{NewVirtualSpecimen, SuspensionFixative},
        },
    },
    define_ordering_enum, uuid_newtype,
};

pub mod block;
pub mod common;
#[cfg(feature = "app")]
mod create;
#[cfg(feature = "app")]
pub mod read;
pub mod tissue;
pub mod virtual_;

#[base_model]
#[serde(tag = "type", rename_all = "snake_case")]
#[cfg_attr(feature = "python", derive(FromPyObject))]
pub enum NewSpecimen {
    CryopreservedTissue(#[garde(dive)] NewCryopreservedTissue),
    FixedBlock(#[garde(dive)] NewFixedBlock),
    FixedTissue(#[garde(dive)] NewFixedTissue),
    FrozenBlock(#[garde(dive)] NewFrozenBlock),
    FrozenTissue(#[garde(dive)] NewFrozenTissue),
    Suspension(#[garde(dive)] NewVirtualSpecimen),
}

#[cfg(feature = "python")]
impl_stub_type!(
    NewSpecimen = NewCryopreservedTissue
        | NewFixedTissue
        | NewFrozenTissue
        | NewFixedBlock
        | NewFrozenBlock
        | NewVirtualSpecimen
);

#[db_simple_enum]
#[cfg_attr(feature = "python", pyo3(module = "scamplepy.common"))]
#[derive(strum::VariantArray)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub enum Species {
    AmbystomaMexicanum,
    CanisFamiliaris,
    CallithrixJacchus,
    DrosophilaMelanogaster,
    GasterosteusAculeatus,
    HomoSapiens,
    MusMusculus,
    RattusNorvegicus,
    SminthopsisCrassicaudata,
}

#[db_simple_enum]
#[derive(strum::Display)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg_attr(feature = "python", pyo3(module = "scamplepy.common"))]
pub enum SpecimenType {
    Block,
    Suspension,
    Tissue,
}

impl SpecimenType {
    fn block() -> SpecimenType {
        Self::Block
    }

    fn suspension() -> SpecimenType {
        Self::Suspension
    }

    fn tissue() -> SpecimenType {
        Self::Tissue
    }
}

#[db_selection]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::specimen))]
pub struct SpecimenSummary {
    pub id: Uuid,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(readonly))]
    pub links: Links,
    pub readable_id: String,
    pub name: String,
    pub received_at: OffsetDateTime,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub species: Vec<Option<String>>, // Option<String> doesn't implement VectorFromWasmAbi :)
    pub notes: Option<String>,
    pub returned_at: Option<OffsetDateTime>,
    pub type_: SpecimenType,
    pub embedded_in: Option<String>,
    pub fixative: Option<String>,
    pub frozen: bool,
    pub cryopreserved: bool,
    pub storage_buffer: Option<String>,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl SpecimenSummary {
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn species(&self) -> Vec<String> {
        self.species
            .iter()
            .filter_map(std::clone::Clone::clone)
            .collect()
    }
}

#[db_selection]
#[cfg_attr(feature = "app", diesel(table_name = specimen, base_query = specimen::table.inner_join(lab::table).inner_join(person::table.on(specimen::submitted_by.eq(person::id)))))]
pub struct SpecimenSummaryWithParents {
    #[cfg_attr(feature = "app", diesel(column_name = id))]
    pub id_: Uuid,
    #[serde(flatten)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub summary: SpecimenSummary,
    #[cfg_attr(feature = "app", diesel(embed))]
    pub lab: LabSummary,
    #[cfg_attr(feature = "app", diesel(embed))]
    pub submitted_by: PersonSummary,
}

#[db_selection]
#[cfg_attr(feature = "app", derive(Associations))]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::specimen_measurement, belongs_to(SpecimenSummaryWithParents, foreign_key = specimen_id)))]
pub struct SpecimenMeasurement {
    pub id: Uuid,
    pub specimen_id: Uuid,
    pub measured_by: Uuid,
    #[serde(flatten)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))] // TODO: custom-getter
    pub data: common::MeasurementData,
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
pub struct Specimen {
    #[serde(flatten)]
    pub info: SpecimenSummaryWithParents,
    pub measurements: Vec<SpecimenMeasurement>,
}

#[base_model]
#[derive(strum::IntoStaticStr)]
#[cfg_attr(feature = "app", derive(AsExpression))]
#[cfg_attr(feature = "app", diesel(sql_type = diesel::sql_types::Text))]
#[cfg_attr(feature = "python", derive(FromPyObject, IntoPyObject))]
#[serde(untagged)]
pub enum BlockEmbeddingMatrix {
    #[strum(transparent)]
    Fixed(FixedBlockEmbeddingMatrix),
    #[strum(transparent)]
    Frozen(FrozenBlockEmbeddingMatrix),
}

#[cfg(feature = "app")]
impl diesel::serialize::ToSql<::diesel::sql_types::Text, diesel::pg::Pg> for BlockEmbeddingMatrix {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        use diesel::{pg::Pg, serialize::ToSql, sql_types};

        let as_str: &str = self.into();
        ToSql::<sql_types::Text, Pg>::to_sql(as_str, &mut out.reborrow())
    }
}

#[cfg(feature = "python")]
impl_stub_type!(BlockEmbeddingMatrix = FixedBlockEmbeddingMatrix | FrozenBlockEmbeddingMatrix);

#[base_model]
#[derive(strum::IntoStaticStr)]
#[cfg_attr(feature = "app", derive(AsExpression))]
#[cfg_attr(feature = "app", diesel(sql_type = diesel::sql_types::Text))]
#[cfg_attr(feature = "python", derive(FromPyObject, IntoPyObject))]
#[serde(untagged)]
pub enum Fixative {
    #[strum(transparent)]
    Block(BlockFixative),
    #[strum(transparent)]
    Suspension(SuspensionFixative),
    #[strum(transparent)]
    Tissue(TissueFixative),
}

#[cfg(feature = "app")]
impl diesel::serialize::ToSql<::diesel::sql_types::Text, diesel::pg::Pg> for Fixative {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        use diesel::{pg::Pg, serialize::ToSql, sql_types};

        let as_str: &str = self.into();
        ToSql::<sql_types::Text, Pg>::to_sql(as_str, &mut out.reborrow())
    }
}

#[cfg(feature = "python")]
impl_stub_type!(Fixative = BlockFixative | SuspensionFixative | TissueFixative);

define_ordering_enum! { SpecimenOrderBy { Name, ReadableId, ReceivedAt }, default = ReceivedAt }

#[db_query]
pub struct SpecimenQuery {
    #[builder(default)]
    pub ids: Vec<Uuid>,
    #[builder(default)]
    pub names: Vec<String>,
    #[builder(default)]
    pub submitters: Vec<Uuid>,
    #[builder(default)]
    pub labs: Vec<Uuid>,
    pub received_before: Option<OffsetDateTime>,
    pub received_after: Option<OffsetDateTime>,
    #[builder(default)]
    pub species: Vec<Species>,
    #[builder(default)]
    pub notes: Vec<String>,
    #[builder(default)]
    pub types: Vec<SpecimenType>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    #[builder(default)]
    pub embedded_in: Vec<BlockEmbeddingMatrix>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    #[builder(default)]
    pub fixatives: Vec<Fixative>,
    #[builder(default)]
    pub storage_buffers: Vec<String>,
    pub frozen: Option<bool>,
    pub cryopreserved: Option<bool>,
    #[builder(default)]
    pub order_by: DefaultVec<SpecimenOrderBy>,
    #[builder(default)]
    pub pagination: Pagination,
}

#[cfg(feature = "python")]
#[gen_stub_pymethods]
#[pymethods]
impl SpecimenQuery {
    #[new]
    #[pyo3(signature = (*,ids = Vec::new(), names = Vec::new(), submitters = Vec::new(), labs = Vec::new(), received_before = None, received_after = None, species = Vec::new(), notes = Vec::new(), types = Vec::new(), embedded_in = Vec::new(), fixatives = Vec::new(), storage_buffers = Vec::new(), frozen = None, cryopreserved = None, order_by = DefaultVec::default(), pagination = Pagination::default()))]
    #[must_use]
    pub fn new(
        ids: Vec<Uuid>,
        names: Vec<String>,
        submitters: Vec<Uuid>,
        labs: Vec<Uuid>,
        received_before: Option<OffsetDateTime>,
        received_after: Option<OffsetDateTime>,
        species: Vec<Species>,
        notes: Vec<String>,
        types: Vec<SpecimenType>,
        embedded_in: Vec<BlockEmbeddingMatrix>,
        fixatives: Vec<Fixative>,
        storage_buffers: Vec<String>,
        frozen: Option<bool>,
        cryopreserved: Option<bool>,
        order_by: DefaultVec<SpecimenOrderBy>,
        pagination: Pagination,
    ) -> Self {
        Self {
            ids,
            names,
            submitters,
            labs,
            received_before,
            received_after,
            species,
            notes,
            types,
            embedded_in,
            fixatives,
            storage_buffers,
            frozen,
            cryopreserved,
            order_by,
            pagination,
        }
    }
}

uuid_newtype!(SpecimenId);

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use serde_json::{Value, json};
    use uuid::Uuid;

    use super::NewSpecimen;

    #[rstest]
    fn deserialize_new_specimen() {
        let uuid = Uuid::now_v7();
        let received_at = "+001999-01-01T00:00:00.000000000Z";
        let frozen_embedding_matrix = "carboxymethyl_cellulose";

        let mut incorrectly_embedded_block = json!({
          "readable_id": "id",
          "lab_id": uuid,
          "name": "krabby_patty",
          "submitted_by": uuid,
          "received_at": received_at,
          "species": ["homo_sapiens"],
          "type": "fixed_block",
          "embedded_in": frozen_embedding_matrix,
          "fixative": "formaldehyde_derivative"
        });

        let deserialize = |json_val| serde_json::from_value::<NewSpecimen>(json_val);

        let err = deserialize(incorrectly_embedded_block.clone()).unwrap_err();
        assert_eq!(err.classify(), serde_json::error::Category::Data);

        incorrectly_embedded_block["embedded_in"] = Value::String("paraffin".to_string());
        let specimen = deserialize(incorrectly_embedded_block.clone()).unwrap();
        let NewSpecimen::FixedBlock(_) = specimen else {
            panic!("expected frozen block, got {specimen:#?}");
        };

        let mut frozen_block = incorrectly_embedded_block;
        frozen_block["type"] = Value::String("frozen_block".to_string());
        frozen_block["embedded_in"] = Value::String(frozen_embedding_matrix.to_string());
        frozen_block["fixative"].take();
        frozen_block["frozen"] = Value::Bool(true);
        let specimen = deserialize(frozen_block.clone()).unwrap();
        let NewSpecimen::FrozenBlock(_) = specimen else {
            panic!("expected frozen block, got {specimen:#?}");
        };

        let mut tissue = frozen_block;
        tissue["type"] = Value::String("fixed_tissue".to_string());
        let err = deserialize(tissue.clone()).unwrap_err();
        assert_eq!(err.classify(), serde_json::error::Category::Data);
    }
}
