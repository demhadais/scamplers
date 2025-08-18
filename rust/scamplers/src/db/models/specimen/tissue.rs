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
use valid_string::ValidString;

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::specimen))]
pub struct NewCryopreservedTissue {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub inner: NewSpecimenCommon,
    #[serde(skip, default = "SpecimenType::tissue")]
    #[builder(default = SpecimenType::Tissue)]
    pub type_: SpecimenType,
    #[garde(dive)]
    pub storage_buffer: Option<ValidString>,
    #[garde(custom(super::common::is_true))]
    pub cryopreserved: bool,
}

impl_constrained_py_setter! { NewCryopreservedTissue::set_type_(SpecimenType) = SpecimenType::Tissue }

impl_constrained_py_setter! { NewCryopreservedTissue::set_cryopreserved(bool) = true }

#[cfg(feature = "python")]
#[pyo3_stub_gen::derive::gen_stub_pymethods]
#[pymethods]
impl NewCryopreservedTissue {
    #[new]
    #[pyo3(signature = (*, readable_id, name, submitted_by, lab_id, received_at, species, storage_buffer=None, measurements=Vec::new(), committee_approvals=Vec::new(), notes=None, returned_at=None, returned_by=None))]
    fn new(
        readable_id: ValidString,
        name: ValidString,
        submitted_by: Uuid,
        lab_id: Uuid,
        received_at: OffsetDateTime,
        species: Vec<Species>,
        storage_buffer: Option<ValidString>,
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
            type_: SpecimenType::Tissue,
            cryopreserved: true,
            storage_buffer,
        }
    }
}

#[db_simple_enum]
#[cfg_attr(feature = "python", pyo3(module = "scamplepy.common"))]
#[derive(Default)]
pub enum TissueFixative {
    #[default]
    DithiobisSuccinimidylropionate,
}

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::specimen))]
pub struct NewFixedTissue {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub inner: NewSpecimenCommon,
    #[serde(skip, default = "SpecimenType::tissue")]
    #[builder(default = SpecimenType::Tissue)]
    pub type_: SpecimenType,
    pub fixative: TissueFixative,
    #[garde(dive)]
    pub storage_buffer: Option<ValidString>,
}

impl_constrained_py_setter! { NewFixedTissue::set_type_(SpecimenType) = SpecimenType::Tissue }

#[cfg(feature = "python")]
#[pyo3_stub_gen::derive::gen_stub_pymethods]
#[pymethods]
impl NewFixedTissue {
    #[new]
    #[pyo3(signature = (*, readable_id, name, submitted_by, lab_id, received_at, species, fixative, storage_buffer=None, measurements=Vec::new(), committee_approvals=Vec::new(), notes=None, returned_at=None, returned_by=None))]
    fn new(
        readable_id: ValidString,
        name: ValidString,
        submitted_by: Uuid,
        lab_id: Uuid,
        received_at: OffsetDateTime,
        species: Vec<Species>,
        fixative: TissueFixative,
        storage_buffer: Option<ValidString>,
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
            fixative,
            type_: SpecimenType::Tissue,
            storage_buffer,
        }
    }
}

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::specimen))]
pub struct NewFrozenTissue {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub inner: NewSpecimenCommon,
    #[serde(skip, default = "SpecimenType::tissue")]
    #[builder(default = SpecimenType::Tissue)]
    pub type_: SpecimenType,
    #[garde(dive)]
    pub storage_buffer: Option<ValidString>,
    #[garde(custom(super::common::is_true))]
    pub frozen: bool,
}

impl_constrained_py_setter! { NewFrozenTissue::set_type_(SpecimenType) = SpecimenType::Tissue }

impl_constrained_py_setter! { NewFrozenTissue::set_frozen(bool) = true }

#[cfg(feature = "python")]
#[pyo3_stub_gen::derive::gen_stub_pymethods]
#[pymethods]
impl NewFrozenTissue {
    #[new]
    #[pyo3(signature = (*, readable_id, name, submitted_by, lab_id, received_at, species, storage_buffer=None, measurements=Vec::new(), committee_approvals=Vec::new(), notes=None, returned_at=None, returned_by=None))]
    fn new(
        readable_id: ValidString,
        name: ValidString,
        submitted_by: Uuid,
        lab_id: Uuid,
        received_at: OffsetDateTime,
        species: Vec<Species>,
        storage_buffer: Option<ValidString>,
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
            type_: SpecimenType::Tissue,
            frozen: true,
            storage_buffer,
        }
    }
}
