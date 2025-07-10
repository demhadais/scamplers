use crate::string::ValidString;
use time::OffsetDateTime;
use uuid::Uuid;
#[cfg(feature = "backend")]
use {
    scamplers_macros::{backend_db_enum, backend_db_json, backend_insertion, backend_with_getters},
    scamplers_schema::{committee_approval, specimen, specimen_measurement},
};

#[cfg_attr(feature = "backend", backend_db_enum)]
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

#[cfg_attr(feature = "backend", backend_db_enum)]
pub enum ComplianceCommitteeType {
    Ibc,
    Irb,
    Iacuc,
}

#[cfg_attr(
    feature = "backend",
    backend_insertion(committee_approval),
    derive(bon::Builder, Clone)
)]
#[cfg_attr(feature = "backend", builder(on(ValidString, into)))]
pub struct NewCommitteeApproval {
    #[cfg_attr(feature = "backend", serde(default))]
    specimen_id: Uuid,
    institution_id: Uuid,
    committee_type: ComplianceCommitteeType,
    #[cfg_attr(feature = "backend", garde(dive))]
    compliance_identifier: ValidString,
}

#[cfg_attr(feature = "backend", backend_with_getters)]
mod with_committee_approval_getters {
    use super::ComplianceCommitteeType;
    use crate::model::institution::InstitutionHandle;
    #[cfg(feature = "backend")]
    use {scamplers_macros::backend_selection, scamplers_schema::committee_approval};

    #[cfg_attr(feature = "backend", backend_selection(committee_approval))]
    pub struct CommitteeApproval {
        #[cfg_attr(feature = "backend", diesel(embed))]
        institution: InstitutionHandle,
        committee_type: ComplianceCommitteeType,
        compliance_identifier: String,
    }
}

#[cfg_attr(feature = "backend", backend_db_json, serde(rename_all = "UPPERCASE"))]
pub enum MeasurementData {
    Rin {
        #[cfg_attr(feature = "backend", valuable(skip))]
        measured_at: OffsetDateTime,
        #[cfg_attr(feature = "backend", garde(dive))]
        instrument_name: ValidString, // This should be an enum
        #[cfg_attr(feature = "backend", garde(range(min = 1.0, max = 10.0)))]
        value: f32,
    },
    Dv200 {
        #[cfg_attr(feature = "backend", valuable(skip))]
        measured_at: OffsetDateTime,
        #[cfg_attr(feature = "backend", garde(dive))]
        instrument_name: ValidString, // This should be a different enum
        #[cfg_attr(feature = "backend", garde(range(min = 0.0, max = 1.0)))]
        value: f32,
    },
}

#[cfg_attr(feature = "backend", backend_insertion(specimen_measurement))]
pub struct NewSpecimenMeasurement {
    #[cfg_attr(feature = "backend", serde(default))]
    pub(crate) specimen_id: Uuid,
    measured_by: Uuid,
    #[cfg_attr(
        feature = "backend",
        garde(dive),
        diesel(skip_insertion),
        serde(flatten)
    )]
    data: MeasurementData,
}

#[cfg(feature = "backend")]
pub(super) fn is_true(value: &bool, _: &()) -> garde::Result {
    if *value {
        Ok(())
    } else {
        Err(garde::Error::new("value must be true"))
    }
}

#[cfg_attr(feature = "backend", backend_insertion(specimen))]
pub struct NewSpecimenCommon {
    #[cfg_attr(feature = "backend", garde(dive))]
    pub(crate) readable_id: ValidString,
    #[cfg_attr(feature = "backend", garde(dive))]
    pub(crate) name: ValidString,
    pub(crate) submitted_by: Uuid,
    pub(crate) lab_id: Uuid,
    #[cfg_attr(feature = "backend", valuable(skip))]
    pub(crate) received_at: OffsetDateTime,
    #[cfg_attr(feature = "backend", garde(length(min = 1)))]
    pub(crate) species: Vec<Species>,
    #[cfg_attr(feature = "backend", diesel(skip_insertion), serde(default))]
    pub(crate) committee_approvals: Vec<NewCommitteeApproval>,
    #[cfg_attr(feature = "backend", garde(dive))]
    pub(crate) notes: Option<ValidString>,
    #[cfg_attr(feature = "backend", valuable(skip))]
    pub(crate) returned_at: Option<OffsetDateTime>,
    pub(crate) returned_by: Option<Uuid>,
    #[cfg_attr(
        feature = "backend",
        diesel(skip_insertion),
        garde(dive),
        serde(default)
    )]
    pub(crate) measurements: Vec<NewSpecimenMeasurement>,
}
