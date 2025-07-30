#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{db_enum, db_insertion, db_json, db_selection, to_from_json};
#[cfg(feature = "backend")]
use scamplers_schema::{committee_approval, specimen, specimen_measurement};
use time::OffsetDateTime;
use uuid::Uuid;
use valid_string::ValidString;

use crate::model::institution::InstitutionHandle;

#[db_enum]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub enum Species {
    AmbystomaMexicanum,
    CanisFamiliaris,
    CallithrixJacchus,
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

#[to_from_json(python)]
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

#[cfg(feature = "python")]
#[pymethods]
impl NewCommitteeApproval {
    #[new]
    #[pyo3(signature = (*, institution_id, committee_type, compliance_identifier, specimen_id=Uuid::default()))]
    fn new(
        institution_id: Uuid,
        committee_type: ComplianceCommitteeType,
        compliance_identifier: ValidString,
        specimen_id: Uuid,
    ) -> Self {
        Self {
            specimen_id,
            institution_id,
            committee_type,
            compliance_identifier,
        }
    }
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = committee_approval))]
pub struct CommitteeApproval {
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub institution: InstitutionHandle,
    pub committee_type: String,
    pub compliance_identifier: String,
}

#[to_from_json(python)]
#[db_json]
#[cfg_attr(feature = "python", pyo3(get_all, set_all))]
#[serde(tag = "type")]
#[cfg_attr(feature = "python", pyo3(name = "SpecimenMeasurementData"))]
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

#[to_from_json(python)]
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

#[cfg(feature = "python")]
#[pymethods]
impl NewSpecimenMeasurement {
    #[new]
    #[pyo3(signature = (*, measured_by, data, specimen_id=Uuid::default()))]
    fn new(measured_by: Uuid, data: MeasurementData, specimen_id: Uuid) -> Self {
        Self {
            specimen_id,
            measured_by,
            data,
        }
    }
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = specimen))]
#[cfg_attr(feature = "python", pyo3(name = "_NewSpecimenCommon"))]
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
    pub returned_at: Option<OffsetDateTime>,
    pub returned_by: Option<Uuid>,
    #[serde(default)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    pub measurements: Vec<NewSpecimenMeasurement>,
}

#[allow(clippy::trivially_copy_pass_by_ref)]
pub(super) fn is_true(value: &bool, _: &()) -> garde::Result {
    if *value {
        Ok(())
    } else {
        Err(garde::Error::new("value must be true"))
    }
}
