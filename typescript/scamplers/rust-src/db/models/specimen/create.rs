use any_value::{AnyValue, WithSnakeCaseKeys};
use diesel::prelude::*;
use scamplers_schema::{committee_approval, specimen, specimen_measurement};

use crate::db::{
    DbOperation,
    models::specimen::{
        NewSpecimen, Specimen, SpecimenId,
        common::{
            AsGenericNewSpecimen, GenericNewSpecimen, NewCommitteeApproval, NewSpecimenCommon,
            NewSpecimenMeasurement, VariableFields,
        },
    },
    util::{ChildrenWithSelfId, SetParentId},
};

impl NewSpecimen {
    fn inner_mut(&mut self) -> &mut NewSpecimenCommon {
        match self {
            Self::FixedBlock(b) => &mut b.inner,
            Self::FrozenBlock(b) => &mut b.inner,
            Self::CryopreservedTissue(t) => &mut t.inner,
            Self::FixedTissue(t) => &mut t.inner,
            Self::FrozenTissue(t) => &mut t.inner,
            Self::CryopreservedSuspension(s) => &mut s.inner,
            Self::FixedOrFreshSuspension(s) => &mut s.inner,
            Self::FrozenSuspension(s) => &mut s.inner,
        }
    }

    fn snake_case_additional_data(&mut self) {
        let inner = self.inner_mut();
        inner.additional_data = inner
            .additional_data
            .take()
            .map(AnyValue::with_snake_case_keys);
    }
}

impl AsGenericNewSpecimen for NewSpecimen {
    fn inner(&self) -> &NewSpecimenCommon {
        match self {
            Self::CryopreservedTissue(s) => s.inner(),
            Self::FixedBlock(s) => s.inner(),
            Self::FixedTissue(s) => s.inner(),
            Self::FrozenBlock(s) => s.inner(),
            Self::FrozenTissue(s) => s.inner(),
            Self::CryopreservedSuspension(s) => s.inner(),
            Self::FixedOrFreshSuspension(s) => s.inner(),
            Self::FrozenSuspension(s) => s.inner(),
        }
    }

    fn variable_fields(&self) -> VariableFields {
        match self {
            Self::CryopreservedTissue(s) => s.variable_fields(),
            Self::FixedBlock(s) => s.variable_fields(),
            Self::FixedTissue(s) => s.variable_fields(),
            Self::FrozenBlock(s) => s.variable_fields(),
            Self::FrozenTissue(s) => s.variable_fields(),
            Self::CryopreservedSuspension(s) => s.variable_fields(),
            Self::FixedOrFreshSuspension(s) => s.variable_fields(),
            Self::FrozenSuspension(s) => s.variable_fields(),
        }
    }
}

impl<'a> Insertable<specimen::table> for &'a NewSpecimen {
    type Values = <GenericNewSpecimen<'a> as Insertable<specimen::table>>::Values;

    fn values(self) -> Self::Values {
        let generic = match self {
            NewSpecimen::CryopreservedTissue(s) => s.as_generic(),
            NewSpecimen::FixedBlock(s) => s.as_generic(),
            NewSpecimen::FixedTissue(s) => s.as_generic(),
            NewSpecimen::FrozenBlock(s) => s.as_generic(),
            NewSpecimen::FrozenTissue(s) => s.as_generic(),
            NewSpecimen::CryopreservedSuspension(s) => s.as_generic(),
            NewSpecimen::FixedOrFreshSuspension(s) => s.as_generic(),
            NewSpecimen::FrozenSuspension(s) => s.as_generic(),
        };

        generic.values()
    }
}

impl DbOperation<()> for &[NewCommitteeApproval] {
    fn execute(self, db_conn: &mut PgConnection) -> crate::result::ScamplersResult<()> {
        diesel::insert_into(committee_approval::table)
            .values(self)
            .execute(db_conn)?;

        Ok(())
    }
}

impl DbOperation<()> for &[NewSpecimenMeasurement] {
    fn execute(self, db_conn: &mut PgConnection) -> crate::result::ScamplersResult<()> {
        diesel::insert_into(specimen_measurement::table)
            .values(self)
            .execute(db_conn)?;

        Ok(())
    }
}

impl SetParentId for NewCommitteeApproval {
    fn parent_id_mut(&mut self) -> &mut uuid::Uuid {
        &mut self.specimen_id
    }
}

impl SetParentId for NewSpecimenMeasurement {
    fn parent_id_mut(&mut self) -> &mut uuid::Uuid {
        &mut self.specimen_id
    }
}

impl ChildrenWithSelfId<NewCommitteeApproval> for NewSpecimen {
    fn children(&mut self) -> &mut [NewCommitteeApproval] {
        &mut self.inner_mut().committee_approvals
    }
}

impl ChildrenWithSelfId<NewSpecimenMeasurement> for NewSpecimen {
    fn children(&mut self) -> &mut [NewSpecimenMeasurement] {
        &mut self.inner_mut().measurements
    }
}

impl DbOperation<Specimen> for NewSpecimen {
    fn execute(mut self, db_conn: &mut PgConnection) -> crate::result::ScamplersResult<Specimen> {
        self.snake_case_additional_data();

        let self_id = diesel::insert_into(specimen::table)
            .values(&self)
            .returning(specimen::id)
            .get_result(db_conn)?;

        let new_measurements: &[NewSpecimenMeasurement] = self.children_with_self_id(self_id);
        new_measurements.execute(db_conn)?;

        let new_committee_approvals: &[NewCommitteeApproval] = self.children_with_self_id(self_id);
        new_committee_approvals.execute(db_conn)?;

        SpecimenId(self_id).execute(db_conn)
    }
}
