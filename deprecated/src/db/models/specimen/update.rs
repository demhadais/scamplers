use diesel::prelude::*;

use crate::db::{
    DbOperation,
    models::specimen::{Specimen, SpecimenId, common::SpecimenUpdateCommon},
    util::SetParentId,
};

impl DbOperation<Specimen> for SpecimenUpdateCommon {
    fn execute(
        self,
        db_conn: &mut diesel::PgConnection,
    ) -> crate::result::ScamplersResult<Specimen> {
        diesel::update(&self)
            .set(&self)
            .execute(db_conn)
            .optional_empty_changeset()?;

        let Self {
            id: specimen_id,
            mut committee_approvals,
            mut measurements,
            ..
        } = self;

        for approval in &mut committee_approvals {
            approval.set_parent_id(specimen_id);
        }
        committee_approvals.execute(db_conn)?;

        for measurement in &mut measurements {
            measurement.set_parent_id(specimen_id);
        }
        measurements.execute(db_conn)?;

        SpecimenId(self.id).execute(db_conn)
    }
}
