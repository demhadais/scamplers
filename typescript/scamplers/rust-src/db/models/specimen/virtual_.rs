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

#[db_simple_enum]
#[cfg_attr(feature = "python", pyo3(module = "scamplepy.common"))]
#[derive(Default)]
pub enum SuspensionFixative {
    #[default]
    FormaldehydeDerivative,
}

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::specimen))]
pub struct NewVirtualSpecimen {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub inner: NewSpecimenCommon,
    #[serde(skip, default = "SpecimenType::suspension")]
    #[builder(skip = SpecimenType::Suspension)]
    pub type_: SpecimenType,
    pub fixative: Option<SuspensionFixative>,
}

impl_constrained_py_setter! { NewVirtualSpecimen::set_type_(SpecimenType) = SpecimenType::Suspension }

#[cfg(feature = "app")]
impl AsGenericNewSpecimen for NewVirtualSpecimen {
    fn inner(&self) -> &NewSpecimenCommon {
        &self.inner
    }

    fn variable_fields(&self) -> VariableFields<'_> {
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
            storage_buffer: None,
        }
    }
}

#[cfg(feature = "python")]
#[pyo3_stub_gen::derive::gen_stub_pymethods]
#[pymethods]
impl NewVirtualSpecimen {
    #[new]
    #[pyo3(signature = (*, readable_id, name, submitted_by, lab_id, received_at, species, fixative=None, measurements=Vec::new(), committee_approvals=Vec::new(), notes=None, returned_at=None, returned_by=None))]
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
            type_: SpecimenType::Suspension,
        }
    }
}
