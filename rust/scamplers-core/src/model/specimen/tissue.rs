#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{db_enum, db_insertion, to_json};
#[cfg(feature = "backend")]
use scamplers_schema::specimen;
#[cfg(feature = "python")]
use time::OffsetDateTime;
#[cfg(feature = "python")]
use uuid::Uuid;
use valid_string::ValidString;

use crate::model::specimen::common::NewSpecimenCommon;
#[cfg(feature = "python")]
use crate::model::specimen::{NewSpecimenMeasurement, Species, common::NewCommitteeApproval};

#[db_enum]
#[derive(Default)]
pub enum TissueType {
    #[default]
    Tissue,
}

#[db_enum]
#[derive(Default)]
pub enum TissueFixative {
    #[default]
    DithiobisSuccinimidylropionate,
}

#[to_json(python)]
#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = specimen))]
pub struct NewFixedTissue {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub inner: NewSpecimenCommon,
    #[serde(skip)]
    pub type_: TissueType,
    #[garde(dive)]
    pub storage_buffer: Option<ValidString>,
    pub fixative: TissueFixative,
}

#[cfg(feature = "python")]
#[pymethods]
impl NewFixedTissue {
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
        fixative: TissueFixative,
        notes: Option<ValidString>,
        returned_at: Option<OffsetDateTime>,
        returned_by: Option<Uuid>,
        storage_buffer: Option<ValidString>,
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
            type_: TissueType::Tissue,
            storage_buffer,
        }
    }
}

#[to_json(python)]
#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = specimen))]
pub struct NewFrozenTissue {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub inner: NewSpecimenCommon,
    #[serde(skip)]
    pub type_: TissueType,
    #[garde(dive)]
    pub storage_buffer: Option<ValidString>,
    #[garde(custom(super::common::is_true))]
    pub frozen: bool,
}

#[cfg(feature = "python")]
#[pymethods]
impl NewFrozenTissue {
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
        storage_buffer: Option<ValidString>,
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
            type_: TissueType::Tissue,
            frozen: true,
            storage_buffer,
        }
    }
}

#[to_json(python)]
#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = specimen))]
pub struct NewCryoPreservedTissue {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub inner: NewSpecimenCommon,
    #[serde(skip)]
    pub type_: TissueType,
    #[garde(dive)]
    pub storage_buffer: Option<ValidString>,
    #[garde(custom(super::common::is_true))]
    pub cryopreserved: bool,
}

#[cfg(feature = "python")]
#[pymethods]
impl NewCryoPreservedTissue {
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
        storage_buffer: Option<ValidString>,
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
            type_: TissueType::Tissue,
            cryopreserved: true,
            storage_buffer,
        }
    }
}
