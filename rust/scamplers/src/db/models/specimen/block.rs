#[cfg(feature = "python")]
use crate::db::models::specimen::{
    Species,
    common::{NewCommitteeApproval, NewSpecimenMeasurement},
};
use crate::{
    db::models::specimen::{SpecimenType, common::NewSpecimenCommon},
    impl_constrained_py_setter,
};
#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{db_insertion, db_simple_enum};
#[cfg(feature = "python")]
use time::OffsetDateTime;
#[cfg(feature = "python")]
use uuid::Uuid;
#[cfg(feature = "python")]
use valid_string::ValidString;

#[db_simple_enum]
#[cfg_attr(feature = "python", pyo3(module = "scamplepy.common"))]
#[derive(Default)]
pub enum FixedBlockEmbeddingMatrix {
    #[default]
    Paraffin,
}

#[db_simple_enum]
#[cfg_attr(feature = "python", pyo3(module = "scamplepy.common"))]
#[derive(Default)]
pub enum BlockFixative {
    #[default]
    FormaldehydeDerivative,
}

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::specimen))]
pub struct NewFixedBlock {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub inner: NewSpecimenCommon,
    #[serde(skip, default = "SpecimenType::block")]
    #[builder(default = SpecimenType::Block)]
    pub type_: SpecimenType,
    pub embedded_in: FixedBlockEmbeddingMatrix,
    pub fixative: BlockFixative,
}

impl_constrained_py_setter! { NewFixedBlock::set_type_(SpecimenType) = SpecimenType::Block }

#[cfg(feature = "python")]
#[pyo3_stub_gen::derive::gen_stub_pymethods]
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
            type_: SpecimenType::Block,
        }
    }
}

#[db_simple_enum]
#[derive(strum::VariantArray)]
#[cfg_attr(feature = "python", pyo3(module = "scamplepy.common"))]
pub enum FrozenBlockEmbeddingMatrix {
    CarboxymethylCellulose,
    OptimalCuttingTemperatureCompound,
}

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::specimen))]
pub struct NewFrozenBlock {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub inner: NewSpecimenCommon,
    #[serde(skip, default = "SpecimenType::block")]
    #[builder(default = SpecimenType::Block)]
    pub type_: SpecimenType,
    pub embedded_in: FrozenBlockEmbeddingMatrix,
    pub fixative: Option<BlockFixative>,
    #[garde(custom(super::common::is_true))]
    pub frozen: bool,
}

impl_constrained_py_setter! { NewFrozenBlock::set_type_(SpecimenType) = SpecimenType::Block }

impl_constrained_py_setter! { NewFrozenBlock::set_frozen(bool) = true }

#[cfg(feature = "python")]
#[pyo3_stub_gen::derive::gen_stub_pymethods]
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
            type_: SpecimenType::Block,
            frozen: true,
        }
    }
}
