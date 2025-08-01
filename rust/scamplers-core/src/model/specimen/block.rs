#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{db_enum, db_insertion};
#[cfg(feature = "backend")]
use scamplers_schema::specimen;
#[cfg(feature = "python")]
use time::OffsetDateTime;
#[cfg(feature = "python")]
use uuid::Uuid;
#[cfg(feature = "python")]
use valid_string::ValidString;

use super::common::NewSpecimenCommon;
#[cfg(feature = "python")]
use crate::model::specimen::{
    common::NewSpecimenMeasurement,
    common::{NewCommitteeApproval, Species},
};

#[db_enum]
#[derive(Default)]
pub enum BlockType {
    #[default]
    Block,
}

#[db_enum]
#[derive(Default)]
pub enum FixedBlockEmbeddingMatrix {
    #[default]
    Paraffin,
}

#[db_enum]
#[derive(Default)]
pub enum BlockFixative {
    #[default]
    FormaldehydeDerivative,
}

#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(scamplers_macros::FromJson, scamplers_macros::ToJson)
)]
#[cfg_attr(not(target_arch = "wasm32"), json(wrapper = super::NewSpecimen, python))]
#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = specimen))]
pub struct NewFixedBlock {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub inner: NewSpecimenCommon,
    #[serde(skip)]
    #[builder(default)]
    pub type_: BlockType,
    pub embedded_in: FixedBlockEmbeddingMatrix,
    pub fixative: BlockFixative,
}

#[cfg(feature = "python")]
#[pymethods]
impl NewFixedBlock {
    #[new]
    #[pyo3(signature = (*, readable_id, name, submitted_by, lab_id, received_at, species, embedded_in, fixative, measurements=Vec::new(), committee_approvals=Vec::new(), notes=None, returned_at=None, returned_by=None))]
    fn new(
        readable_id: ValidString,
        name: ValidString,
        submitted_by: Uuid,
        lab_id: Uuid,
        received_at: OffsetDateTime,
        species: Vec<Species>,
        embedded_in: FixedBlockEmbeddingMatrix,
        fixative: BlockFixative,
        measurements: Vec<NewSpecimenMeasurement>,
        committee_approvals: Vec<NewCommitteeApproval>,
        notes: Option<ValidString>,
        returned_at: Option<OffsetDateTime>,
        returned_by: Option<Uuid>,
    ) -> Self {
        Self {
            inner: NewSpecimenCommon {
                readable_id,
                name,
                submitted_by,
                lab_id,
                received_at,
                species,
                committee_approvals,
                notes,
                returned_at,
                returned_by,
                measurements,
            },
            embedded_in,
            fixative,
            type_: BlockType::Block,
        }
    }
}

#[db_enum]
#[derive(Default, strum::VariantArray)]
pub enum FrozenBlockEmbeddingMatrix {
    #[default] // Honestly not sure why I need this :(
    CarboxymethylCellulose,
    OptimalCuttingTemperatureCompound,
}

#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(scamplers_macros::FromJson, scamplers_macros::ToJson)
)]
#[cfg_attr(not(target_arch = "wasm32"), json(wrapper = super::NewSpecimen, python))]
#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = specimen))]
pub struct NewFrozenBlock {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub inner: NewSpecimenCommon,
    #[serde(skip)]
    #[builder(default)]
    pub type_: BlockType,
    pub embedded_in: FrozenBlockEmbeddingMatrix,
    pub fixative: Option<BlockFixative>,
    #[garde(custom(super::common::is_true))]
    pub frozen: bool,
}

#[cfg(feature = "python")]
#[pymethods]
impl NewFrozenBlock {
    #[new]
    #[pyo3(signature = (*, readable_id, name, submitted_by, lab_id, received_at, species, embedded_in, fixative=None, measurements=Vec::new(), committee_approvals=Vec::new(), notes=None, returned_at=None, returned_by=None))]
    fn new(
        readable_id: ValidString,
        name: ValidString,
        submitted_by: Uuid,
        lab_id: Uuid,
        received_at: OffsetDateTime,
        species: Vec<Species>,
        embedded_in: FrozenBlockEmbeddingMatrix,
        fixative: Option<BlockFixative>,
        measurements: Vec<NewSpecimenMeasurement>,
        committee_approvals: Vec<NewCommitteeApproval>,
        notes: Option<ValidString>,
        returned_at: Option<OffsetDateTime>,
        returned_by: Option<Uuid>,
    ) -> Self {
        Self {
            inner: NewSpecimenCommon {
                readable_id,
                name,
                submitted_by,
                lab_id,
                received_at,
                species,
                committee_approvals,
                notes,
                returned_at,
                returned_by,
                measurements,
            },
            embedded_in,
            fixative,
            type_: BlockType::Block,
            frozen: true,
        }
    }
}
