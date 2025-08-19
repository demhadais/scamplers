use diesel::prelude::*;
use scamplers_schema::{suspension, suspension_measurement, suspension_preparers};
use uuid::Uuid;

use crate::db::{
    DbOperation,
    models::suspension::suspension::{
        NewSuspension, NewSuspensionMeasurement, Suspension, SuspensionId, SuspensionMeasurement,
        SuspensionPreparer,
    },
    util::{ChildrenWithSelfId, ManyToMany, ManyToManyChildrenWithSelfId, SetParentId},
};

impl SetParentId for NewSuspensionMeasurement {
    fn parent_id_mut(&mut self) -> &mut Uuid {
        &mut self.suspension_id
    }
}

impl ChildrenWithSelfId<NewSuspensionMeasurement> for NewSuspension {
    fn children(&mut self) -> &mut [NewSuspensionMeasurement] {
        &mut self.measurements
    }
}

impl DbOperation<Vec<SuspensionMeasurement>> for &[NewSuspensionMeasurement] {
    fn execute(
        self,
        db_conn: &mut diesel::PgConnection,
    ) -> crate::result::ScamplersResult<Vec<SuspensionMeasurement>> {
        Ok(diesel::insert_into(suspension_measurement::table)
            .values(self)
            .returning(SuspensionMeasurement::as_returning())
            .get_results(db_conn)?)
    }
}

impl ManyToMany for SuspensionPreparer {
    fn new(suspension_id: Uuid, prepared_by: Uuid) -> Self {
        Self {
            suspension_id,
            prepared_by,
        }
    }
}

impl ManyToManyChildrenWithSelfId<SuspensionPreparer> for NewSuspension {
    fn mtm_children(&self) -> &[Uuid] {
        &self.preparer_ids
    }
}

impl DbOperation<Suspension> for NewSuspension {
    fn execute(mut self, db_conn: &mut PgConnection) -> crate::result::ScamplersResult<Suspension> {
        let id = diesel::insert_into(suspension::table)
            .values(&self)
            .returning(suspension::id)
            .get_result(db_conn)?;

        let new_measurements = self.children_with_self_id(id);
        new_measurements.execute(db_conn)?;

        let preparers = self.mtm_children_with_self_id(id);
        diesel::insert_into(suspension_preparers::table)
            .values(preparers)
            .execute(db_conn)?;

        SuspensionId(id).execute(db_conn)
    }
}
