use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use scamplers_core::model::suspension::{
    NewSuspensionPool, NewSuspensionPoolMeasurement, SuspensionPoolHandle, SuspensionPoolPreparer,
};
use scamplers_schema::{suspension_pool, suspension_pool_measurement, suspension_pool_preparers};
use uuid::Uuid;

use crate::{
    db::model::{WriteToDb, WriteToDbInternal},
    result::ScamplersResult,
};

trait SuspensionPoolExt {
    fn measurements(&mut self, self_id: Uuid) -> &[NewSuspensionPoolMeasurement];
    fn preparers(&self, self_id: Uuid) -> Vec<SuspensionPoolPreparer>;
}

impl SuspensionPoolExt for NewSuspensionPool {
    fn measurements(&mut self, self_id: Uuid) -> &[NewSuspensionPoolMeasurement] {
        for m in &mut self.measurements {
            m.pool_id = self_id;
        }

        self.measurements.as_slice()
    }

    fn preparers(&self, self_id: Uuid) -> Vec<SuspensionPoolPreparer> {
        self.preparer_ids
            .iter()
            .map(|p| SuspensionPoolPreparer {
                pool_id: self_id,
                prepared_by: *p,
            })
            .collect()
    }
}

// TODO: return something more useful than just an ID and link
impl WriteToDb for NewSuspensionPool {
    type Returns = SuspensionPoolHandle;

    async fn write_to_db(
        mut self,
        db_conn: &mut diesel_async::AsyncPgConnection,
    ) -> ScamplersResult<Self::Returns> {
        let handle = diesel::insert_into(suspension_pool::table)
            .values(&self)
            .returning(SuspensionPoolHandle::as_select())
            .get_result(db_conn)
            .await?;

        diesel::insert_into(suspension_pool_measurement::table)
            .values(self.measurements(handle.id))
            .execute(db_conn)
            .await?;

        diesel::insert_into(suspension_pool_preparers::table)
            .values(self.preparers(handle.id))
            .execute(db_conn)
            .await?;

        // This for-loop sucks
        for mut s in self.suspensions {
            s.pooled_into_id = Some(handle.id);
            s.write_to_db(db_conn).await?;
        }

        Ok(handle)
    }
}
