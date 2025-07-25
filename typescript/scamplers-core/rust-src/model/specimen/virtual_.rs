#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{db_enum, db_insertion, to_from_json};
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
use crate::model::specimen::common::{NewCommitteeApproval, NewSpecimenMeasurement, Species};

#[db_enum]
#[derive(Default)]
pub enum SuspensionType {
    #[default]
    Suspension,
}

#[db_enum]
#[derive(Default)]
pub enum SuspensionFixative {
    #[default]
    FormaldehydeDerivative,
}

#[to_from_json(python)]
#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = specimen))]
pub struct NewVirtualSpecimen {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub inner: NewSpecimenCommon,
    #[serde(skip)]
    pub type_: SuspensionType,
    pub fixative: SuspensionFixative,
}

#[cfg(feature = "python")]
#[pymethods]
impl NewVirtualSpecimen {
    #[new]
    fn new(
        readable_id: ValidString,
        name: ValidString,
        submitted_by: Uuid,
        lab_id: Uuid,
        received_at: OffsetDateTime,
        species: Vec<Species>,
        measurements: Vec<NewSpecimenMeasurement>,
        committee_approvals: Vec<NewCommitteeApproval>,
        notes: Option<ValidString>,
        returned_at: Option<OffsetDateTime>,
        returned_by: Option<Uuid>,
        fixative: SuspensionFixative,
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
            type_: SuspensionType::Suspension,
        }
    }
}
