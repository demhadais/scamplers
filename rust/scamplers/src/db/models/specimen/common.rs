#[cfg(feature = "app")]
use diesel::prelude::*;
#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3_stub_gen::derive::{gen_stub_pyclass_complex_enum, gen_stub_pymethods};
use scamplers_macros::{db_insertion, db_json, db_selection, db_simple_enum};
use time::OffsetDateTime;
use uuid::Uuid;
use valid_string::ValidString;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::db::models::{institution::Institution, specimen::Species};

#[db_simple_enum]
#[cfg_attr(feature = "python", pyo3(module = "scamplepy.create"))]
pub enum ComplianceCommitteeType {
    Ibc,
    Irb,
    Iacuc,
}

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::committee_approval))]
pub struct NewCommitteeApproval {
    #[serde(default)]
    pub specimen_id: Uuid,
    pub institution_id: Uuid,
    pub committee_type: ComplianceCommitteeType,
    #[garde(dive)]
    pub compliance_identifier: ValidString,
}

#[cfg(feature = "python")]
#[gen_stub_pymethods]
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
#[cfg_attr(feature = "app", derive(Associations))]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::committee_approval, primary_key(institution_id, committee_type, specimen_id), belongs_to(super::SpecimenSummaryWithRelations, foreign_key = specimen_id)))]
pub struct CommitteeApproval {
    pub institution_id: Uuid,
    pub specimen_id: Uuid,
    #[cfg_attr(feature = "app", diesel(embed))]
    pub institution: Institution,
    pub committee_type: String,
    pub compliance_identifier: String,
}

#[cfg_attr(feature = "python", gen_stub_pyclass_complex_enum)]
#[db_json]
#[serde(tag = "type")]
#[cfg_attr(
    feature = "python",
    pyo3(name = "SpecimenMeasurementData", module = "scamplepy.common")
)]
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
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::specimen_measurement))]
pub struct NewSpecimenMeasurement {
    #[serde(default)]
    #[builder(default)]
    pub specimen_id: Uuid,
    pub measured_by: Uuid,
    #[serde(flatten)]
    #[garde(dive)]
    pub data: MeasurementData,
}

#[cfg(feature = "python")]
#[gen_stub_pymethods]
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
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::specimen))]
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
    #[builder(default)]
    #[garde(dive)]
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
    pub committee_approvals: Vec<NewCommitteeApproval>,
    #[garde(dive)]
    pub notes: Option<ValidString>,
    pub returned_at: Option<OffsetDateTime>,
    pub returned_by: Option<Uuid>,
    #[serde(default)]
    #[garde(dive)]
    #[builder(default)]
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
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
