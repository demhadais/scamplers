use crate::model::Order;
use crate::model::{
    Pagination,
    specimen::{
        block::{
            BlockFixative, BlockType, FixedBlockEmbeddingMatrix, FrozenBlockEmbeddingMatrix,
            NewBlock,
        },
        common::Species,
        suspension::{NewVirtualSuspensionSpecimen, SuspensionFixative, SuspensionType},
        tissue::{NewTissue, TissueFixative, TissueType},
    },
};
pub use common::NewSpecimenMeasurement;
#[cfg(feature = "backend")]
use scamplers_macros::{backend_db_enum, backend_query_request, backend_with_getters};
use uuid::Uuid;

pub mod block;
mod common;
pub mod suspension;
pub mod tissue;

#[cfg_attr(
    feature = "backend",
    derive(serde::Deserialize, Debug, valuable::Valuable, garde::Validate)
)]
#[cfg_attr(feature = "backend", serde(rename_all = "lowercase", tag = "type"))]
pub enum NewSpecimen {
    Block(#[cfg_attr(feature = "backend", garde(dive))] NewBlock),
    Tissue(#[cfg_attr(feature = "backend", garde(dive))] NewTissue),
    Suspension(#[cfg_attr(feature = "backend", garde(dive))] NewVirtualSuspensionSpecimen),
}

#[cfg(feature = "backend")]
impl NewSpecimen {
    #[must_use]
    pub fn measurements(self, specimen_id: Uuid) -> Vec<NewSpecimenMeasurement> {
        let mut inner = match self {
            Self::Block(b) => match b {
                NewBlock::Fixed(b) => b.common,
                NewBlock::Frozen(b) => b.common,
            },

            Self::Suspension(s) => s.common,

            Self::Tissue(t) => match t {
                NewTissue::Cryopreserved(t) => t.common,
                NewTissue::Fixed(t) => t.common,
                NewTissue::Frozen(t) => t.common,
            },
        };
        let mut measurements = inner.measurements.drain(..);

        for mut m in &mut measurements {
            m.specimen_id = specimen_id;
        }

        measurements.collect()
    }
}

#[cfg_attr(feature = "backend", backend_with_getters)]
mod with_getters {
    use crate::model::{
        lab::LabSummary,
        person::{PersonHandle, PersonSummary},
        specimen::common::{MeasurementData, Species},
    };
    use time::OffsetDateTime;
    use uuid::Uuid;
    #[cfg(feature = "backend")]
    use {
        scamplers_macros::backend_selection,
        scamplers_schema::{specimen, specimen_measurement},
    };

    #[cfg_attr(feature = "backend", backend_selection(specimen))]
    pub struct SpecimenHandle {
        id: Uuid,
        link: String,
    }

    #[cfg_attr(feature = "backend", backend_selection(specimen))]
    pub struct SpecimenSummary {
        #[cfg_attr(feature = "backend", diesel(embed), serde(flatten))]
        handle: SpecimenHandle,
        readable_id: String,
        name: String,
        #[cfg_attr(feature = "backend", valuable(skip))]
        received_at: OffsetDateTime,
        species: Vec<Option<Species>>,
        notes: Option<String>,
        #[cfg_attr(feature = "backend", valuable(skip))]
        returned_at: Option<OffsetDateTime>,
        type_: String,
        embedded_in: Option<String>,
        fixative: Option<String>,
        frozen: bool,
        cryopreserved: bool,
        storage_buffer: Option<String>,
    }

    #[cfg_attr(feature = "backend", backend_selection(specimen_measurement))]
    pub struct SpecimenMeasurement {
        #[cfg_attr(feature = "backend", diesel(embed))]
        measured_by: PersonHandle,
        data: MeasurementData,
    }

    #[cfg_attr(feature = "backend", backend_selection(specimen), derive(bon::Builder))]
    pub struct SpecimenCore {
        #[cfg_attr(feature = "backend", diesel(embed), serde(flatten))]
        summary: SpecimenSummary,
        #[cfg_attr(feature = "backend", diesel(embed))]
        lab: LabSummary,
        #[cfg_attr(feature = "backend", diesel(embed))]
        submitted_by: PersonSummary,
        #[cfg_attr(feature = "backend", diesel(embed))]
        returned_by: PersonSummary,
    }

    #[cfg_attr(feature = "backend", derive(serde::Serialize, bon::Builder))]
    pub struct Specimen {
        core: SpecimenCore,
        measurements: Vec<SpecimenMeasurement>,
    }
}
use time::OffsetDateTime;
pub use with_getters::*;

#[cfg_attr(feature = "backend", backend_db_enum)]
#[cfg_attr(feature = "backend", serde(untagged))]
pub enum SpecimenType {
    Block(BlockType),
    Tissue(TissueType),
    Suspension(SuspensionType),
}

#[cfg_attr(
    feature = "backend",
    derive(serde::Deserialize, valuable::Valuable, Debug)
)]
#[cfg_attr(feature = "backend", serde(untagged))]
pub enum BlockEmbeddingMatrix {
    Fixed(FixedBlockEmbeddingMatrix),
    Frozen(FrozenBlockEmbeddingMatrix),
}

#[cfg_attr(
    feature = "backend",
    derive(serde::Deserialize, valuable::Valuable, Debug)
)]
#[cfg_attr(feature = "backend", serde(untagged))]
pub enum Fixative {
    Block(BlockFixative),
    Tissue(TissueFixative),
    Suspension(SuspensionFixative),
}

#[derive(Default, valuable::Valuable, Debug, serde::Deserialize, serde::Serialize)]
pub enum SpecimenOrdinalColumn {
    #[default]
    Name,
    ReceivedAt,
}

#[cfg_attr(feature = "backend", backend_query_request)]
pub struct SpecimenQuery {
    pub ids: Vec<Uuid>,
    pub name: Option<String>,
    #[cfg_attr(feature = "backend", valuable(skip))]
    pub received_before: Option<OffsetDateTime>,
    #[cfg_attr(feature = "backend", valuable(skip))]
    pub received_after: Option<OffsetDateTime>,
    pub species: Vec<Species>,
    pub notes: Option<String>,
    #[cfg_attr(feature = "backend", serde(alias = "type"))]
    pub type_: Option<SpecimenType>,
    pub embedded_in: Option<BlockEmbeddingMatrix>,
    pub fixative: Option<Fixative>,
    pub storage_buffer: Option<String>,
    pub frozen: Option<bool>,
    pub cryopreserved: Option<bool>,
    pub order_by: Vec<Order<SpecimenOrdinalColumn>>,
    pub pagination: Pagination,
}

#[cfg(all(feature = "backend", test))]
mod tests {

    use pretty_assertions::assert_eq;
    use serde_json::{Value, json};
    use time::OffsetDateTime;
    use uuid::Uuid;

    use crate::model::specimen::{NewSpecimen, block::NewBlock};

    #[test]
    fn deserialize_specimen() {
        let uuid = Uuid::now_v7();
        let received_at = OffsetDateTime::now_utc();
        let frozen_embedding_matrix = "carboxymethyl_cellulose";

        let mut fixed_block = json!({
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

        let err = deserialize(fixed_block.clone()).unwrap_err();
        assert_eq!(err.classify(), serde_json::error::Category::Data);

        fixed_block["embedded_in"] = Value::String("paraffin".to_string());
        let specimen = deserialize(fixed_block.clone()).unwrap();
        let NewSpecimen::Block(NewBlock::Fixed(_)) = specimen else {
            panic!("expected frozen block, got {specimen:?}");
        };

        let mut frozen_block = fixed_block;
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
