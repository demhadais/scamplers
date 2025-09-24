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
pub struct NewCryopreservedTissue {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub inner: NewSpecimenCommon,
    #[serde(skip, default = "SpecimenType::tissue")]
    #[builder(skip = SpecimenType::Tissue)]
    pub type_: SpecimenType,
    #[builder(skip = true)]
    #[garde(custom(super::common::is_true))]
    pub cryopreserved: bool,
}

impl_constrained_py_setter! { NewCryopreservedTissue::set_type_(SpecimenType) = SpecimenType::Tissue }

impl_constrained_py_setter! { NewCryopreservedTissue::set_cryopreserved(bool) = true }

#[cfg(feature = "app")]
impl AsGenericNewSpecimen for NewCryopreservedTissue {
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
impl NewCryopreservedTissue {
    #[new]
    #[pyo3(signature = (*, readable_id, name, submitted_by, lab_id, received_at, species, tissue, measurements=Vec::new(), committee_approvals=Vec::new(), returned_at=None, returned_by=None, additional_data=None))]
    fn new(
        readable_id: ValidString,
        name: ValidString,
        submitted_by: Uuid,
        lab_id: Uuid,
        received_at: OffsetDateTime,
        species: Vec<Species>,
        tissue: ValidString,
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
                tissue,
                committee_approvals,
                returned_at,
                returned_by,
                measurements,
                additional_data,
            },
            type_: SpecimenType::Tissue,
            cryopreserved: true,
        }
    }
}

#[db_simple_enum]
#[cfg_attr(feature = "python", pyo3(module = "scamplepy.common"))]
pub enum TissueFixative {
    DithiobisSuccinimidylpropionate,
}

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::specimen))]
pub struct NewFixedTissue {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub inner: NewSpecimenCommon,
    #[serde(skip, default = "SpecimenType::tissue")]
    #[builder(skip = SpecimenType::Tissue)]
    pub type_: SpecimenType,
    pub fixative: TissueFixative,
}

impl_constrained_py_setter! { NewFixedTissue::set_type_(SpecimenType) = SpecimenType::Tissue }

#[cfg(feature = "app")]
impl AsGenericNewSpecimen for NewFixedTissue {
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
            fixative: Some(Fixative::Tissue(*fixative)),
            embedded_in: None,
            cryopreserved: false,
            frozen: false,
        }
    }
}

#[cfg(feature = "python")]
#[pyo3_stub_gen::derive::gen_stub_pymethods]
#[pymethods]
impl NewFixedTissue {
    #[new]
    #[pyo3(signature = (*, readable_id, name, submitted_by, lab_id, received_at, species, fixative, tissue, measurements=Vec::new(), committee_approvals=Vec::new(), returned_at=None, returned_by=None, additional_data=None))]
    fn new(
        readable_id: ValidString,
        name: ValidString,
        submitted_by: Uuid,
        lab_id: Uuid,
        received_at: OffsetDateTime,
        species: Vec<Species>,
        fixative: TissueFixative,
        tissue: ValidString,
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
                tissue,
                committee_approvals,
                returned_at,
                returned_by,
                measurements,
                additional_data,
            },
            fixative,
            type_: SpecimenType::Tissue,
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
    #[builder(skip = SpecimenType::Tissue)]
    pub type_: SpecimenType,
    #[builder(skip = true)]
    #[garde(custom(super::common::is_true))]
    pub frozen: bool,
}

impl_constrained_py_setter! { NewFrozenTissue::set_type_(SpecimenType) = SpecimenType::Tissue }

impl_constrained_py_setter! { NewFrozenTissue::set_frozen(bool) = true }

#[cfg(feature = "app")]
impl AsGenericNewSpecimen for NewFrozenTissue {
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
impl NewFrozenTissue {
    #[new]
    #[pyo3(signature = (*, readable_id, name, submitted_by, lab_id, received_at, species, tissue, measurements=Vec::new(), committee_approvals=Vec::new(), returned_at=None, returned_by=None, additional_data=None))]
    fn new(
        readable_id: ValidString,
        name: ValidString,
        submitted_by: Uuid,
        lab_id: Uuid,
        received_at: OffsetDateTime,
        species: Vec<Species>,
        tissue: ValidString,
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
                tissue,
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
