use diesel::prelude::*;
use scamplers_schema::{suspension_pool, suspension_pool_measurement, suspension_pool_preparers};
use uuid::Uuid;

use crate::db::{
    DbOperation,
    models::suspension::{
        pool::{
            NewSuspensionPool, NewSuspensionPoolMeasurement, SuspensionPool, SuspensionPoolId,
            SuspensionPoolMeasurement, SuspensionPoolPreparer,
        },
        suspension::NewSuspension,
    },
    util::{ChildrenWithSelfId, ManyToMany, ManyToManyChildrenWithSelfId, SetParentId},
};

impl SetParentId for NewSuspension {
    fn parent_id_mut(&mut self) -> &mut Uuid {
        self.pooled_into.insert(Uuid::nil())
    }
}

impl ChildrenWithSelfId<NewSuspension> for NewSuspensionPool {
    fn children(&mut self) -> &mut [NewSuspension] {
        &mut self.suspensions
    }
}

impl SetParentId for NewSuspensionPoolMeasurement {
    fn parent_id_mut(&mut self) -> &mut Uuid {
        &mut self.pool_id
    }
}

impl ChildrenWithSelfId<NewSuspensionPoolMeasurement> for NewSuspensionPool {
    fn children(&mut self) -> &mut [NewSuspensionPoolMeasurement] {
        &mut self.measurements
    }
}

impl DbOperation<Vec<SuspensionPoolMeasurement>> for &[NewSuspensionPoolMeasurement] {
    fn execute(
        self,
        db_conn: &mut PgConnection,
    ) -> crate::result::ScamplersResult<Vec<SuspensionPoolMeasurement>> {
        Ok(diesel::insert_into(suspension_pool_measurement::table)
            .values(self)
            .returning(SuspensionPoolMeasurement::as_returning())
            .get_results(db_conn)?)
    }
}

impl ManyToMany for SuspensionPoolPreparer {
    fn new(pool_id: Uuid, prepared_by: Uuid) -> Self {
        Self {
            pool_id,
            prepared_by,
        }
    }
}
impl ManyToManyChildrenWithSelfId<SuspensionPoolPreparer> for NewSuspensionPool {
    fn mtm_children(&self) -> &[Uuid] {
        &self.preparer_ids
    }
}

impl DbOperation<SuspensionPool> for NewSuspensionPool {
    fn execute(
        mut self,
        db_conn: &mut diesel::PgConnection,
    ) -> crate::result::ScamplersResult<SuspensionPool> {
        let id = diesel::insert_into(suspension_pool::table)
            .values(&self)
            .returning(suspension_pool::id)
            .get_result(db_conn)?;

        let new_suspensions: &mut [NewSuspension] = self.children_with_self_id(id);
        for s in new_suspensions {
            s.execute(db_conn)?;
        }

        let new_measurements: &mut [NewSuspensionPoolMeasurement] = self.children_with_self_id(id);
        (&*new_measurements).execute(db_conn)?;

        let suspension_pool_preparers = self.mtm_children_with_self_id(id);
        diesel::insert_into(suspension_pool_preparers::table)
            .values(suspension_pool_preparers)
            .execute(db_conn)?;

        SuspensionPoolId(id).execute(db_conn)
    }
}
