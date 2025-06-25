use diesel_async::RunQueryDsl;
use scamplers_core::model::sequencing_run::NewSequencingRun;
use scamplers_schema::{
    sequencing_run::{self, id as id_col},
    sequencing_submissions,
};

use crate::db::{self, model::WriteToDb};

impl WriteToDb for NewSequencingRun {
    type Returns = ();
    async fn write(
        mut self,
        db_conn: &mut diesel_async::AsyncPgConnection,
    ) -> db::error::Result<Self::Returns> {
        let id = diesel::insert_into(sequencing_run::table)
            .values(&self)
            .returning(id_col)
            .get_result(db_conn)
            .await?;

        let submissions = self.libraries(id);
        diesel::insert_into(sequencing_submissions::table)
            .values(submissions)
            .execute(db_conn)
            .await?;

        Ok(())
    }
}
