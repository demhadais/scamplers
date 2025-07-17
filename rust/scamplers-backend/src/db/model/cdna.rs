use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use scamplers_core::model::nucleic_acid::{
    CdnaHandle, NewCdna, NewCdnaMeasurement, NewCdnaPreparer,
};
use scamplers_schema::{cdna, cdna_measurement, cdna_preparers};
use uuid::Uuid;

use crate::db::model::WriteToDb;

trait NewCdnaExt {
    fn measurements(&mut self, self_id: Uuid) -> &[NewCdnaMeasurement];
    fn preparers(&mut self, self_id: Uuid) -> Vec<NewCdnaPreparer>;
}

impl NewCdnaExt for NewCdna {
    fn measurements(&mut self, self_id: Uuid) -> &[NewCdnaMeasurement] {
        for m in &mut self.measurements {
            m.cdna_id = self_id;
        }

        &self.measurements
    }

    fn preparers(&mut self, self_id: Uuid) -> Vec<NewCdnaPreparer> {
        self.preparer_ids
            .iter()
            .map(|p| NewCdnaPreparer {
                cdna_id: self_id,
                prepared_by: *p,
            })
            .collect()
    }
}

// TODO: VALIDATE LIBRARY TYPE
impl WriteToDb for NewCdna {
    type Returns = CdnaHandle;

    async fn write_to_db(
        mut self,
        db_conn: &mut diesel_async::AsyncPgConnection,
    ) -> crate::db::error::Result<Self::Returns> {
        let handle = diesel::insert_into(cdna::table)
            .values(&self)
            .returning(CdnaHandle::as_returning())
            .get_result(db_conn)
            .await?;

        let measurements = self.measurements(handle.id);
        diesel::insert_into(cdna_measurement::table)
            .values(measurements)
            .execute(db_conn)
            .await?;

        let preparers = self.preparers(handle.id);
        diesel::insert_into(cdna_preparers::table)
            .values(preparers)
            .execute(db_conn)
            .await?;

        Ok(handle)
    }
}
