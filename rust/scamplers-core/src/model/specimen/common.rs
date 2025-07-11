use crate::model::institution::InstitutionHandle;
use scamplers_macros::{db_enum, db_insertion, db_json, db_selection};
#[cfg(feature = "backend")]
use scamplers_schema::{committee_approval, specimen, specimen_measurement};
use serde::Serialize;
use time::OffsetDateTime;
use uuid::Uuid;
use valid_string::ValidString;

#[db_enum]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
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
#[cfg_attr(feature = "backend", diesel(table_name = committee_approval))]
pub struct NewCommitteeApproval {
    #[serde(default)]
    pub specimen_id: Uuid,
    pub institution_id: Uuid,
    pub committee_type: ComplianceCommitteeType,
    #[garde(dive)]
    pub compliance_identifier: ValidString,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = committee_approval))]
pub struct CommitteeApproval {
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub institution: InstitutionHandle,
    pub committee_type: String,
    pub compliance_identifier: String,
}

#[db_json]
pub enum MeasurementData {
    Rin {
        measured_at: OffsetDateTime,
        #[garde(dive)]
        instrument_name: ValidString,
        #[garde(range(min = 1.0, max = 10.0))]
        value: f32,
    },
    Dv200 {
        measured_at: OffsetDateTime,
        #[garde(dive)]
        instrument_name: ValidString,
        #[garde(range(min = 0.0, max = 1.0))]
        value: f32,
    },
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = specimen_measurement))]
pub struct NewSpecimenMeasurement {
    #[serde(default)]
    pub specimen_id: Uuid,
    pub measured_by: Uuid,
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    pub data: MeasurementData,
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = specimen))]
pub struct NewSpecimenCommon {
    #[garde(dive)]
    pub readable_id: ValidString,
    #[garde(dive)]
    pub name: ValidString,
    pub submitted_by: Uuid,
    pub lab_id: Uuid,
    pub received_at: OffsetDateTime,
    #[garde(length(min = 1))]
    pub species: Vec<Species>,
    #[serde(default)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    pub committee_approvals: Vec<NewCommitteeApproval>,
    #[garde(dive)]
    pub notes: Option<ValidString>,
    returned_at: Option<OffsetDateTime>,
    returned_by: Option<Uuid>,
    #[serde(default)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    pub measurements: Vec<NewSpecimenMeasurement>,
}

pub(super) fn is_true(value: &bool, _: &()) -> garde::Result {
    if *value {
        Ok(())
    } else {
        Err(garde::Error::new("value must be true"))
    }
}
