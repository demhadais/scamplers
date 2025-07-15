use crate::model::{
    Pagination, SortByGroup, person::PersonHandle, specimen::common::NewSpecimenCommon,
};

use super::{lab::LabSummary, person::PersonSummary};
use common::{MeasurementData, Species};
use scamplers_macros::{base_api_model, db_enum, db_query, db_selection};
#[cfg(feature = "backend")]
use scamplers_schema::{specimen, specimen_measurement};
use time::OffsetDateTime;
use uuid::Uuid;
pub use {
    block::NewBlock, common::NewSpecimenMeasurement, tissue::NewTissue,
    virtual_::NewVirtualSpecimen,
};

mod block;
mod common;
mod tissue;
mod virtual_;

#[base_api_model]
#[serde(tag = "type")]
pub enum NewSpecimen {
    Block(#[garde(dive)] NewBlock),
    Suspension(#[garde(dive)] NewVirtualSpecimen),
    Tissue(#[garde(dive)] NewTissue),
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = specimen))]
pub struct SpecimenHandle {
    pub id: Uuid,
    pub link: String,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = specimen))]
pub struct SpecimenSummary {
    #[serde(flatten)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub handle: SpecimenHandle,
    pub readable_id: String,
    pub name: String,
    pub received_at: OffsetDateTime,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub species: Vec<Option<String>>,
    pub notes: Option<String>,
    pub returned_at: Option<OffsetDateTime>,
    pub type_: String,
    pub embedded_in: Option<String>,
    pub fixative: Option<String>,
    pub frozen: bool,
    pub cryopreserved: bool,
    pub storage_buffer: Option<String>,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = specimen_measurement))]
pub struct SpecimenMeasurement {
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub measured_by: PersonHandle,
    #[serde(flatten)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub data: MeasurementData,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = specimen))]
pub struct SpecimenCore {
    #[serde(flatten)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub summary: SpecimenSummary,
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub lab: LabSummary,
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub submitted_by: PersonSummary,
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub returned_by: PersonSummary,
}

#[base_api_model]
#[cfg_attr(
    target_arch = "wasm32",
    ::wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)
)]
pub struct Specimen {
    pub core: SpecimenCore,
    pub measurements: Vec<SpecimenMeasurement>,
}

#[db_enum]
#[serde(untagged)]
pub enum SpecimenType {
    Block(block::BlockType),
    Tissue(tissue::TissueType),
    Suspension(virtual_::SuspensionType),
}

#[db_enum]
#[serde(untagged)]
pub enum BlockEmbeddingMatrix {
    Fixed(block::FixedBlockEmbeddingMatrix),
    Frozen(block::FrozenBlockEmbeddingMatrix),
}

#[db_enum]
#[serde(untagged)]
pub enum Fixative {
    Block(block::BlockFixative),
    Tissue(tissue::TissueFixative),
    Suspension(virtual_::SuspensionFixative),
}

#[db_enum]
#[derive(Default)]
#[serde(untagged)]
pub enum SpecimenOrdinalColumn {
    Name,
    #[default]
    ReceivedAt,
}

#[db_query]
pub struct SpecimenQuery {
    pub ids: Vec<Uuid>,
    pub name: Option<String>,
    pub submitters: Vec<Uuid>,
    pub labs: Vec<Uuid>,
    pub received_before: Option<OffsetDateTime>,
    pub received_after: Option<OffsetDateTime>,
    pub species: Vec<Species>,
    pub notes: Option<String>,
    #[serde(alias = "type")]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub type_: Option<SpecimenType>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub embedded_in: Option<BlockEmbeddingMatrix>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub fixative: Option<Fixative>,
    pub storage_buffer: Option<String>,
    pub frozen: Option<bool>,
    pub cryopreserved: Option<bool>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub order_by: SortByGroup<SpecimenOrdinalColumn>,
    pub pagination: Pagination,
}

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;
    use serde_json::{Value, json};
    use uuid::Uuid;

    use crate::model::specimen::{NewSpecimen, block::NewBlock};

    #[test]
    fn deserialize_new_specimen() {
        let uuid = Uuid::now_v7();
        let received_at = "1999-01-01 00:00:00.0 +00:00:00";
        let frozen_embedding_matrix = "carboxymethyl_cellulose";

        let mut incorrectly_embedded_block = json!({
          "readable_id": "id",
          "lab_id": uuid,
          "name": "krabby_patty",
          "submitted_by": uuid,
          "received_at": received_at,
          "species": ["homo_sapiens"],
          "type": "block",
          "preservation": "fixed",
          "embedded_in": frozen_embedding_matrix,
          "fixative": "formaldehyde_derivative"
        });

        let deserialize = |json_val| serde_json::from_value::<NewSpecimen>(json_val);

        let err = deserialize(incorrectly_embedded_block.clone()).unwrap_err();
        assert_eq!(err.classify(), serde_json::error::Category::Data);

        incorrectly_embedded_block["embedded_in"] = Value::String("paraffin".to_string());
        let specimen = deserialize(incorrectly_embedded_block.clone()).unwrap();
        let NewSpecimen::Block(NewBlock::Fixed(_)) = specimen else {
            panic!("expected frozen block, got {specimen:?}");
        };

        let mut frozen_block = incorrectly_embedded_block;
        frozen_block["preservation"] = Value::String("frozen".to_string());
        frozen_block["embedded_in"] = Value::String(frozen_embedding_matrix.to_string());
        frozen_block["fixative"] = Value::Null;
        frozen_block["frozen"] = Value::Bool(true);
        let specimen = deserialize(frozen_block.clone()).unwrap();
        let NewSpecimen::Block(NewBlock::Frozen(_)) = specimen else {
            panic!("expected frozen block, got {specimen:?}");
        };

        let mut tissue = frozen_block;
        tissue["preservation"] = Value::String("fixed".to_string());
        tissue["type"] = Value::String("tissue".to_string());
        let err = deserialize(tissue.clone()).unwrap_err();
        assert_eq!(err.classify(), serde_json::error::Category::Data);
    }
}
