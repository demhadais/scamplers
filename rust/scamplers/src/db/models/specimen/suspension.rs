#[cfg(feature = "python")]
use any_value::AnyValue;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{db_insertion, db_simple_enum};
#[cfg(feature = "python")]
use time::OffsetDateTime;
#[cfg(feature = "python")]
use uuid::Uuid;
#[cfg(feature = "python")]
use valid_string::ValidString;

#[cfg(feature = "app")]
use crate::db::models::specimen::common::{AsGenericNewSpecimen, VariableFields};
#[cfg(feature = "python")]
use crate::db::models::specimen::{
    Species,
    common::{NewCommitteeApproval, NewSpecimenMeasurement},
};
use crate::{
    db::models::specimen::{SpecimenType, common::NewSpecimenCommon},
    impl_constrained_py_setter,
};

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::specimen))]
pub struct NewCryopreservedSuspension {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub inner: NewSpecimenCommon,
    #[serde(skip, default = "SpecimenType::suspension")]
    #[builder(skip = SpecimenType::Suspension)]
    pub type_: SpecimenType,
    #[builder(skip = true)]
    #[garde(custom(super::common::is_true))]
    pub cryopreserved: bool,
}

impl_constrained_py_setter! { NewCryopreservedSuspension::set_type_(SpecimenType) = SpecimenType::Suspension }

impl_constrained_py_setter! { NewCryopreservedSuspension::set_cryopreserved(bool) = true }

#[cfg(feature = "app")]
impl AsGenericNewSpecimen for NewCryopreservedSuspension {
    fn inner(&self) -> &NewSpecimenCommon {
        &self.inner
    }

    fn variable_fields(&self) -> VariableFields {
        let Self {
            type_,
            cryopreserved,
            ..
        } = self;

        VariableFields {
            type_: *type_,
            cryopreserved: *cryopreserved,
            fixative: None,
            embedded_in: None,
            frozen: false,
        }
    }
}

#[cfg(feature = "python")]
#[pyo3_stub_gen::derive::gen_stub_pymethods]
#[pymethods]
impl NewCryopreservedSuspension {
    #[new]
    #[pyo3(signature = (*, readable_id, name, submitted_by, lab_id, received_at, species, measurements=Vec::new(), committee_approvals=Vec::new(), returned_at=None, returned_by=None, additional_data=None))]
    fn new(
        readable_id: ValidString,
        name: ValidString,
        submitted_by: Uuid,
        lab_id: Uuid,
        received_at: OffsetDateTime,
        species: Vec<Species>,
        measurements: Vec<NewSpecimenMeasurement>,
        committee_approvals: Vec<NewCommitteeApproval>,
        returned_at: Option<OffsetDateTime>,
        returned_by: Option<Uuid>,
        additional_data: Option<AnyValue>,
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
                returned_at,
                returned_by,
                measurements,
                additional_data,
            },
            type_: SpecimenType::Suspension,
            cryopreserved: true,
        }
    }
}

#[db_simple_enum]
#[cfg_attr(feature = "python", pyo3(module = "scamplepy.common"))]
pub enum SuspensionFixative {
    DithiobisSuccinimidylpropionate,
    FormaldehydeDerivative,
}

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::specimen))]
pub struct NewFixedOrFreshSuspension {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub inner: NewSpecimenCommon,
    #[serde(skip, default = "SpecimenType::suspension")]
    #[builder(skip = SpecimenType::Suspension)]
    pub type_: SpecimenType,
    pub fixative: Option<SuspensionFixative>,
}

impl_constrained_py_setter! { NewFixedOrFreshSuspension::set_type_(SpecimenType) = SpecimenType::Suspension }

#[cfg(feature = "app")]
impl AsGenericNewSpecimen for NewFixedOrFreshSuspension {
    fn inner(&self) -> &NewSpecimenCommon {
        &self.inner
    }

    fn variable_fields(&self) -> VariableFields {
        use crate::db::models::specimen::Fixative;

        let Self {
            type_, fixative, ..
        } = self;

        VariableFields {
            type_: *type_,
            fixative: fixative.map(Fixative::Suspension),
            embedded_in: None,
            cryopreserved: false,
            frozen: false,
        }
    }
}

#[cfg(feature = "python")]
#[pyo3_stub_gen::derive::gen_stub_pymethods]
#[pymethods]
impl NewFixedOrFreshSuspension {
    #[new]
    #[pyo3(signature = (*, readable_id, name, submitted_by, lab_id, received_at, species, fixative=None, measurements=Vec::new(), committee_approvals=Vec::new(), returned_at=None, returned_by=None, additional_data=None))]
    fn new(
        readable_id: ValidString,
        name: ValidString,
        submitted_by: Uuid,
        lab_id: Uuid,
        received_at: OffsetDateTime,
        species: Vec<Species>,
        fixative: Option<SuspensionFixative>,
        measurements: Vec<NewSpecimenMeasurement>,
        committee_approvals: Vec<NewCommitteeApproval>,
        returned_at: Option<OffsetDateTime>,
        returned_by: Option<Uuid>,
        additional_data: Option<AnyValue>,
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
                returned_at,
                returned_by,
                measurements,
                additional_data,
            },
            fixative,
            type_: SpecimenType::Suspension,
        }
    }
}

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::specimen))]
pub struct NewFrozenSuspension {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub inner: NewSpecimenCommon,
    #[serde(skip, default = "SpecimenType::suspension")]
    #[builder(skip = SpecimenType::Suspension)]
    pub type_: SpecimenType,
    #[builder(skip = true)]
    #[garde(custom(super::common::is_true))]
    pub frozen: bool,
}

impl_constrained_py_setter! { NewFrozenSuspension::set_type_(SpecimenType) = SpecimenType::Tissue }

impl_constrained_py_setter! { NewFrozenSuspension::set_frozen(bool) = true }

#[cfg(feature = "app")]
impl AsGenericNewSpecimen for NewFrozenSuspension {
    fn inner(&self) -> &NewSpecimenCommon {
        &self.inner
    }

    fn variable_fields(&self) -> VariableFields {
        let Self { type_, frozen, .. } = self;

        VariableFields {
            type_: *type_,
            frozen: *frozen,
            fixative: None,
            embedded_in: None,
            cryopreserved: false,
        }
    }
}

#[cfg(feature = "python")]
#[pyo3_stub_gen::derive::gen_stub_pymethods]
#[pymethods]
impl NewFrozenSuspension {
    #[new]
    #[pyo3(signature = (*, readable_id, name, submitted_by, lab_id, received_at, species, measurements=Vec::new(), committee_approvals=Vec::new(), returned_at=None, returned_by=None, additional_data=None))]
    fn new(
        readable_id: ValidString,
        name: ValidString,
        submitted_by: Uuid,
        lab_id: Uuid,
        received_at: OffsetDateTime,
        species: Vec<Species>,
        measurements: Vec<NewSpecimenMeasurement>,
        committee_approvals: Vec<NewCommitteeApproval>,
        returned_at: Option<OffsetDateTime>,
        returned_by: Option<Uuid>,
        additional_data: Option<AnyValue>,
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
                returned_at,
                returned_by,
                measurements,
                additional_data,
            },
            type_: SpecimenType::Tissue,
            frozen: true,
        }
    }
}
