use any_value::AnyValue;
#[cfg(feature = "app")]
use diesel::prelude::*;
#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3_stub_gen::derive::{gen_stub_pyclass_complex_enum, gen_stub_pymethods};
use scamplers_macros::{db_insertion, db_json, db_selection, db_simple_enum, db_update};
#[cfg(feature = "app")]
use scamplers_schema::{committee_approval, institution};
use time::OffsetDateTime;
use uuid::Uuid;
use valid_string::ValidString;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "app")]
use crate::db::models::specimen::{BlockEmbeddingMatrix, Fixative, SpecimenType};
use crate::db::{
    models::{institution::Institution, specimen::Species},
    validators::children_parent_ids_matches_parent_id,
};

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
#[cfg_attr(feature = "app", diesel(table_name = committee_approval, primary_key(institution_id, committee_type, specimen_id), base_query = committee_approval::table.inner_join(institution::table), belongs_to(super::SpecimenSummaryWithParents, foreign_key = specimen_id)))]
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
    pyo3(name = "SpecimenMeasurementData", module = "scamplepy.common", set_all)
)]
pub enum MeasurementData {
    #[serde(rename = "RIN")]
    Rin {
        measured_at: OffsetDateTime,
        #[garde(dive)]
        instrument_name: Option<ValidString>,
        #[garde(range(min = 1.0, max = 10.0))]
        value: f32,
    },
    #[serde(rename = "DV200")]
    Dv200 {
        measured_at: OffsetDateTime,
        #[garde(dive)]
        instrument_name: Option<ValidString>,
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
    #[garde(dive)]
    pub tissue: ValidString,
    #[serde(default)]
    #[builder(default)]
    #[garde(dive)]
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
    pub committee_approvals: Vec<NewCommitteeApproval>,
    pub returned_at: Option<OffsetDateTime>,
    pub returned_by: Option<Uuid>,
    #[serde(default)]
    #[garde(dive)]
    #[builder(default)]
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
    pub measurements: Vec<NewSpecimenMeasurement>,
    pub additional_data: Option<AnyValue>,
}

#[allow(clippy::trivially_copy_pass_by_ref)]
pub(super) fn is_true(value: &bool, _: &()) -> garde::Result {
    if *value {
        Ok(())
    } else {
        Err(garde::Error::new("value must be true"))
    }
}

#[cfg(feature = "app")]
#[derive(Insertable)]
#[diesel(table_name = scamplers_schema::specimen)]
pub struct VariableFields {
    pub type_: SpecimenType,
    pub embedded_in: Option<BlockEmbeddingMatrix>,
    pub fixative: Option<Fixative>,
    pub cryopreserved: bool,
    pub frozen: bool,
}

#[cfg(feature = "app")]
pub type GenericNewSpecimen<'a> = (&'a NewSpecimenCommon, VariableFields);

#[cfg(feature = "app")]
pub trait AsGenericNewSpecimen {
    fn inner(&self) -> &NewSpecimenCommon;

    fn variable_fields(&self) -> VariableFields;

    fn as_generic(&self) -> GenericNewSpecimen<'_> {
        (self.inner(), self.variable_fields())
    }
}

// The weird type-hints are necessitated by rust being weird
#[db_update]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::specimen))]
pub struct SpecimenUpdateCommon {
    pub id: Uuid,
    #[garde(dive)]
    pub readable_id: Option<ValidString>,
    #[garde(dive)]
    pub name: Option<ValidString>,
    pub submitted_by: Option<Uuid>,
    pub lab_id: Option<Uuid>,
    pub received_at: Option<OffsetDateTime>,
    #[garde(length(min = 1))]
    pub species: Option<Vec<Species>>,
    #[garde(dive)]
    pub tissue: Option<ValidString>,
    #[garde(dive, custom(|approvals: &[NewCommitteeApproval], (): &()| children_parent_ids_matches_parent_id(self.id, approvals, |a| &a.specimen_id)))]
    #[cfg_attr(feature = "app", diesel(skip_update))]
    pub committee_approvals: Vec<NewCommitteeApproval>,
    pub returned_at: Option<OffsetDateTime>,
    pub returned_by: Option<Uuid>,
    #[garde(dive, custom(|measurements: &[NewSpecimenMeasurement], (): &()| children_parent_ids_matches_parent_id(self.id, measurements, |m| &m.specimen_id)))]
    #[cfg_attr(feature = "app", diesel(skip_update))]
    pub measurements: Vec<NewSpecimenMeasurement>,
    pub additional_data: Option<AnyValue>,
}

#[cfg(feature = "python")]
#[gen_stub_pymethods]
#[pymethods]
impl SpecimenUpdateCommon {
    #[new]
    #[pyo3(signature = (*, id, readable_id=None, name=None, submitted_by=None, lab_id=None, received_at = None, species = None, tissue=None, committee_approvals = Vec::new(), returned_at=None, returned_by=None, measurements=Vec::new(), additional_data = None))]
    #[must_use]
    pub fn new(
        id: Uuid,
        readable_id: Option<ValidString>,
        name: Option<ValidString>,
        submitted_by: Option<Uuid>,
        lab_id: Option<Uuid>,
        received_at: Option<OffsetDateTime>,
        species: Option<Vec<Species>>,
        tissue: Option<ValidString>,
        committee_approvals: Vec<NewCommitteeApproval>,
        returned_at: Option<OffsetDateTime>,
        returned_by: Option<Uuid>,
        measurements: Vec<NewSpecimenMeasurement>,
        additional_data: Option<AnyValue>,
    ) -> Self {
        Self {
            id,
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
        }
    }
}
