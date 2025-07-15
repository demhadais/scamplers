use crate::db::{
    self,
    model::{FetchById, WriteToDb, WriteToDbInternal},
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use scamplers_core::model::{
    person::PersonHandle,
    suspension::{
        NewSuspension, NewSuspensionMeasurement, Suspension, SuspensionCore, SuspensionMeasurement,
        SuspensionPreparer,
    },
};
use scamplers_schema::{
    multiplexing_tag,
    person::{self},
    specimen,
    suspension::{self, id as id_col},
    suspension_measurement::{self, suspension_id},
    suspension_preparers,
};
use uuid::Uuid;

#[diesel::dsl::auto_type]
fn suspension_measurement_from_clause() -> _ {
    suspension_measurement::table.inner_join(person::table)
}

impl FetchById for Suspension {
    type Id = Uuid;

    async fn fetch_by_id(
        id: &Self::Id,
        db_conn: &mut diesel_async::AsyncPgConnection,
    ) -> db::error::Result<Self> {
        let core = core_from_clause()
            .filter(id_col.eq(id))
            .select(SuspensionCore::as_select())
            .first(db_conn)
            .await?;

        let measurements = suspension_measurement_from_clause()
            .filter(suspension_id.eq(id))
            .select(SuspensionMeasurement::as_select())
            .load(db_conn)
            .await?;

        let preparers = suspension_preparers::table
            .filter(suspension_preparers::suspension_id.eq(id))
            .inner_join(person::table)
            .select(PersonHandle::as_select())
            .load(db_conn)
            .await?;

        Ok(Suspension {
            core,
            measurements,
            preparers,
        })
    }
}

impl WriteToDbInternal for &[NewSuspensionMeasurement] {
    type Returns = ();

    async fn write(
        self,
        db_conn: &mut diesel_async::AsyncPgConnection,
    ) -> db::error::Result<Self::Returns> {
        diesel::insert_into(suspension_measurement::table)
            .values(self)
            .execute(db_conn)
            .await?;

        Ok(())
    }
}

#[diesel::dsl::auto_type]
fn core_from_clause() -> _ {
    suspension::table
        .inner_join(specimen::table)
        .inner_join(multiplexing_tag::table)
}

trait NewSuspensionExt {
    fn measurements(&mut self, self_id: Uuid) -> &[NewSuspensionMeasurement];
    fn preparers(&mut self, self_id: Uuid) -> Vec<SuspensionPreparer>;
}
impl NewSuspensionExt for NewSuspension {
    fn measurements(&mut self, self_id: Uuid) -> &[NewSuspensionMeasurement] {
        for m in &mut self.measurements {
            m.suspension_id = self_id;
        }

        self.measurements.as_slice()
    }

    fn preparers(&mut self, self_id: Uuid) -> Vec<SuspensionPreparer> {
        self.preparer_ids
            .iter()
            .map(|p| SuspensionPreparer {
                suspension_id: self_id,
                prepared_by: *p,
            })
            .collect()
    }
}

impl WriteToDb for NewSuspension {
    type Returns = Suspension;

    async fn write_to_db(
        mut self,
        db_conn: &mut diesel_async::AsyncPgConnection,
    ) -> crate::db::error::Result<Self::Returns> {
        let id = diesel::insert_into(suspension::table)
            .values(&self)
            .returning(id_col)
            .get_result(db_conn)
            .await?;

        diesel::insert_into(suspension_preparers::table)
            .values(self.preparers(id))
            .execute(db_conn)
            .await?;

        self.measurements(id).write(db_conn).await?;

        Suspension::fetch_by_id(&id, db_conn).await
    }
}
