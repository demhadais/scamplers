use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use scamplers_core::model::sequencing_run::{
    NewSequencingRun, NewSequencingSubmission, SequencingRunHandle,
};
use scamplers_schema::{sequencing_run, sequencing_submissions};
use uuid::Uuid;

use crate::db::{self, model::WriteToDb};

trait SequencingRunExt {
    fn libraries(&mut self, self_id: Uuid) -> &[NewSequencingSubmission];
}

impl SequencingRunExt for NewSequencingRun {
    fn libraries(&mut self, self_id: Uuid) -> &[NewSequencingSubmission] {
        for s in &mut self.libraries {
            s.sequencing_run_id = self_id;
        }

        &self.libraries
    }
}

impl WriteToDb for NewSequencingRun {
    type Returns = SequencingRunHandle;

    async fn write_to_db(
        mut self,
        db_conn: &mut diesel_async::AsyncPgConnection,
    ) -> db::error::Result<Self::Returns> {
        let handle = diesel::insert_into(sequencing_run::table)
            .values(&self)
            .returning(SequencingRunHandle::as_select())
            .get_result(db_conn)
            .await?;

        diesel::insert_into(sequencing_submissions::table)
            .values(self.libraries(handle.id))
            .execute(db_conn)
            .await?;

        Ok(handle)
    }
}
