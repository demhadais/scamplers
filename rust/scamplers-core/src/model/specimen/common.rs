use crate::{model::institution::InstitutionHandle, string::NonEmptyString};
use getset::Setters;
use scamplers_macros::{db_enum, db_insertion, db_json, db_selection};
#[cfg(feature = "backend")]
use scamplers_schema::{committee_approval, specimen, specimen_measurement};
use time::OffsetDateTime;
use uuid::Uuid;

#[db_enum]
pub enum Species {
    AmbystomaMexicanum,
    CanisFamiliaris,
    DrosophilaMelanogaster,
    GasterosteusAculeatus,
    HomoSapiens,
    MusMusculus,
    RattusNorvegicus,
    SminthopsisCrassicaudata,
}

#[db_enum]
pub enum ComplianceCommitteeType {
    Ibc,
    Irb,
    Iacuc,
}

#[db_insertion]
#[derive(Setters)]
#[cfg_attr(feature = "backend", diesel(table_name = committee_approval))]
pub struct NewCommitteeApproval {
    #[serde(default)]
    #[getset(set = "pub(super)")]
    specimen_id: Uuid,
    institution_id: Uuid,
    committee_type: ComplianceCommitteeType,
    #[garde(dive)]
    compliance_identifier: NonEmptyString,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = committee_approval))]
pub struct CommitteeApproval {
    #[cfg_attr(feature = "backend", diesel(embed))]
    institution: InstitutionHandle,
    committee_type: ComplianceCommitteeType,
    compliance_identifier: String,
}

#[db_json]
pub enum MeasurementData {
    Rin {
        measured_at: OffsetDateTime,
        #[garde(dive)]
        instrument_name: NonEmptyString, // This should be an enum
        #[garde(range(min = 1.0, max = 10.0))]
        value: f32,
    },
    Dv200 {
        measured_at: OffsetDateTime,
        #[garde(dive)]
        instrument_name: NonEmptyString, // This should be a different enum
        #[garde(range(min = 0.0, max = 1.0))]
        value: f32,
    },
}

#[db_insertion]
#[derive(Setters)]
#[cfg_attr(feature = "backend", diesel(table_name = specimen_measurement))]
pub struct NewSpecimenMeasurement {
    #[serde(default)]
    #[getset(set = "pub(super)")]
    specimen_id: Uuid,
    measured_by: Uuid,
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    data: MeasurementData,
}

#[db_insertion]
#[derive(getset::MutGetters)]
#[cfg_attr(feature = "backend", diesel(table_name = specimen))]
pub struct NewSpecimenCommon {
    #[garde(dive)]
    readable_id: NonEmptyString,
    #[garde(dive)]
    name: NonEmptyString,
    submitted_by: Uuid,
    lab_id: Uuid,
    received_at: OffsetDateTime,
    #[garde(length(min = 1))]
    species: Vec<Species>,
    #[serde(default)]
    #[garde(dive)]
    #[getset(get_mut = "pub(super)")]
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    committee_approvals: Vec<NewCommitteeApproval>,
    #[garde(dive)]
    notes: Option<NonEmptyString>,
    returned_at: Option<OffsetDateTime>,
    returned_by: Option<Uuid>,
    #[serde(default)]
    #[garde(dive)]
    #[getset(get_mut = "pub(super)")]
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    measurements: Vec<NewSpecimenMeasurement>,
}

pub(super) fn is_true(value: &bool, _: &()) -> garde::Result {
    if *value {
        Ok(())
    } else {
        Err(garde::Error::new("value must be true"))
    }
}
