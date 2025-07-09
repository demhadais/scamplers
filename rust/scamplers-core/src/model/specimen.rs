use crate::model::{
    Pagination, SortByGroup,
    person::PersonHandle,
    specimen::common::{NewCommitteeApproval, NewSpecimenCommon, NewSpecimenMeasurement},
};

use super::{lab::LabSummary, person::PersonSummary};
use block::NewBlock;
use common::{MeasurementData, Species};
use scamplers_macros::{base_api_model, base_api_model_with_default, db_query, db_selection};
#[cfg(feature = "backend")]
use scamplers_schema::{specimen, specimen_measurement};
use time::OffsetDateTime;
use tissue::NewTissue;
use uuid::Uuid;
use virtual_::NewVirtualSpecimen;

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

// impl NewSpecimen {
//     pub fn measurements(&mut self, self_id: Uuid) -> &[NewSpecimenMeasurement] {
//         let inner = match self {
//             Self::Block(b) => b.inner_mut(),
//             Self::Suspension(s) => s.inner_mut(),
//             Self::Tissue(t) => t.inner_mut(),
//         };

//         let measurements = inner.measurements_mut();
//         for m in &mut *measurements {
//             m.set_specimen_id(self_id);
//         }

//         measurements
//     }

//     pub fn committe_approvals(&mut self, self_id: Uuid) -> &[NewCommitteeApproval] {
//         let approvals = match self {
//             Self::Block(b) => b.committee_approvals_mut(),
//             Self::Suspension(s) => s.committee_approvals_mut(),
//             Self::Tissue(t) => t.committee_approvals_mut(),
//         };

//         for a in &mut *approvals {
//             a.set_specimen_id(self_id);
//         }

//         approvals
//     }
// }

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
    pub species: Vec<Option<Species>>,
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
    #[wasm_bindgen(skip)]
    pub data: MeasurementData,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = specimen))]
pub struct SpecimenCore {
    #[serde(flatten)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    summary: SpecimenSummary,
    #[cfg_attr(feature = "backend", diesel(embed))]
    lab: LabSummary,
    #[cfg_attr(feature = "backend", diesel(embed))]
    submitted_by: PersonSummary,
    #[cfg_attr(feature = "backend", diesel(embed))]
    returned_by: PersonSummary,
}
impl SpecimenCore {
    pub fn id(&self) -> &Uuid {
        self.summary().id()
    }
}

#[base_api_model]
#[derive(::derive_builder::Builder)]
#[builder(pattern = "owned", build_fn(error = crate::model::BuilderError))]
#[cfg_attr(target_arch = "wasm32", ::wasm_bindgen::prelude::wasm_bindgen)]
pub struct Specimen {
    core: SpecimenCore,
    measurements: Vec<SpecimenMeasurement>,
}

#[base_api_model]
#[serde(untagged)]
pub enum SpecimenType {
    Block(block::BlockType),
    Tissue(tissue::TissueType),
    Suspension(virtual_::SuspensionType),
}

#[base_api_model]
#[serde(untagged)]
pub enum BlockEmbeddingMatrix {
    Fixed(block::FixedBlockEmbeddingMatrix),
    Frozen(block::FrozenBlockEmbeddingMatrix),
}

#[base_api_model]
#[serde(untagged)]
pub enum Fixative {
    Block(block::BlockFixative),
    Tissue(tissue::TissueFixative),
    Suspension(virtual_::SuspensionFixative),
}

#[base_api_model_with_default]
pub enum SpecimenOrdinalColumn {
    Name,
    #[default]
    ReceivedAt,
}

#[db_query]
pub struct SpecimenQuery {
    ids: Vec<Uuid>,
    name: Option<String>,
    #[builder(setter(custom))]
    submitters: Vec<Uuid>,
    #[builder(setter(custom))]
    labs: Vec<Uuid>,
    received_before: Option<OffsetDateTime>,
    received_after: Option<OffsetDateTime>,
    #[builder(setter(custom))]
    species: Vec<Species>,
    notes: Option<String>,
    #[serde(alias = "type")]
    type_: Option<SpecimenType>,
    embedded_in: Option<BlockEmbeddingMatrix>,
    fixative: Option<Fixative>,
    storage_buffer: Option<String>,
    frozen: Option<bool>,
    cryopreserved: Option<bool>,
    #[builder(setter(custom))]
    order_by: SortByGroup<SpecimenOrdinalColumn>,
    pagination: Pagination,
}

impl SpecimenQueryBuilder {
    pub fn submitter(mut self, submitter_id: Uuid) {
        if let Some(ref mut submitters) = self.submitters {
            submitters.push(submitter_id);
        } else {
            self.submitters = Some(vec![submitter_id]);
        }
    }

    pub fn species(mut self, species: Species) {
        if let Some(ref mut species_list) = self.species {
            species_list.push(species);
        } else {
            self.species = Some(vec![species]);
        }
    }
}

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;
    use serde_json::{Value, json};
    use uuid::Uuid;

    use crate::model::specimen::{NewSpecimen, block::NewBlock};

    #[test]
    fn deserialize_specimen() {
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
