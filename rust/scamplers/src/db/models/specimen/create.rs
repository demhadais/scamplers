use diesel::prelude::*;
use scamplers_schema::{committee_approval, specimen, specimen_measurement};

use crate::db::{
    DbOperation,
    models::specimen::{
        NewSpecimen, Specimen, SpecimenId,
        block::{NewFixedBlock, NewFrozenBlock},
        common::{NewCommitteeApproval, NewSpecimenCommon, NewSpecimenMeasurement},
        tissue::{NewCryopreservedTissue, NewFixedTissue, NewFrozenTissue},
        virtual_::NewVirtualSpecimen,
    },
    util::{ChildrenWithSelfId, SetParentId},
};

impl<'a> Insertable<specimen::table> for &'a NewSpecimen {
    type Values = <(
        Option<&'a NewFixedBlock>,
        Option<&'a NewFrozenBlock>,
        Option<&'a NewCryopreservedTissue>,
        Option<&'a NewFixedTissue>,
        Option<&'a NewFrozenTissue>,
        Option<&'a NewVirtualSpecimen>,
    ) as Insertable<specimen::table>>::Values;

    fn values(self) -> Self::Values {
        match self {
            NewSpecimen::FixedBlock(f) => (
                Some(f),
                None::<&NewFrozenBlock>,
                None::<&NewCryopreservedTissue>,
                None::<&NewFixedTissue>,
                None::<&NewFrozenTissue>,
                None::<&NewVirtualSpecimen>,
            )
                .values(),
            NewSpecimen::FrozenBlock(f) => (
                None::<&NewFixedBlock>,
                Some(f),
                None::<&NewCryopreservedTissue>,
                None::<&NewFixedTissue>,
                None::<&NewFrozenTissue>,
                None::<&NewVirtualSpecimen>,
            )
                .values(),
            NewSpecimen::CryopreservedTissue(f) => (
                None::<&NewFixedBlock>,
                None::<&NewFrozenBlock>,
                Some(f),
                None::<&NewFixedTissue>,
                None::<&NewFrozenTissue>,
                None::<&NewVirtualSpecimen>,
            )
                .values(),
            NewSpecimen::FixedTissue(f) => (
                None::<&NewFixedBlock>,
                None::<&NewFrozenBlock>,
                None::<&NewCryopreservedTissue>,
                Some(f),
                None::<&NewFrozenTissue>,
                None::<&NewVirtualSpecimen>,
            )
                .values(),
            NewSpecimen::FrozenTissue(f) => (
                None::<&NewFixedBlock>,
                None::<&NewFrozenBlock>,
                None::<&NewCryopreservedTissue>,
                None::<&NewFixedTissue>,
                Some(f),
                None::<&NewVirtualSpecimen>,
            )
                .values(),
            NewSpecimen::Suspension(f) => (
                None::<&NewFixedBlock>,
                None::<&NewFrozenBlock>,
                None::<&NewCryopreservedTissue>,
                None::<&NewFixedTissue>,
                None::<&NewFrozenTissue>,
                Some(f),
            )
                .values(),
        }
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

impl NewSpecimen {
    fn inner_mut(&mut self) -> &mut NewSpecimenCommon {
        match self {
            Self::FixedBlock(b) => &mut b.inner,
            Self::FrozenBlock(b) => &mut b.inner,
            Self::CryopreservedTissue(t) => &mut t.inner,
            Self::FixedTissue(t) => &mut t.inner,
            Self::FrozenTissue(t) => &mut t.inner,
            Self::Suspension(s) => &mut s.inner,
        }
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
