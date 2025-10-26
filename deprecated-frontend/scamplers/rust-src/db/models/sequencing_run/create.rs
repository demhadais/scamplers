use any_value::{AnyValue, WithSnakeCaseKeys};
use diesel::prelude::*;
use scamplers_schema::{sequencing_run, sequencing_submissions};

use crate::db::{
    DbOperation,
    models::sequencing_run::{NewSequencingRun, SequencingRun, SequencingRunId},
};

impl NewSequencingRun {
    fn snake_case_additional_data(&mut self) {
        self.additional_data = self
            .additional_data
            .take()
            .map(AnyValue::with_snake_case_keys);
    }
}

impl DbOperation<SequencingRun> for NewSequencingRun {
    fn execute(
        mut self,
        db_conn: &mut diesel::PgConnection,
    ) -> crate::result::ScamplersResult<SequencingRun> {
        self.snake_case_additional_data();

        let id = diesel::insert_into(sequencing_run::table)
            .values(&self)
            .returning(sequencing_run::id)
            .get_result(db_conn)?;

        for submission in &mut self.libraries {
            submission.sequencing_run_id = id;
        }

        // TODO: Factor this out because it'll be useful to add libraries to a
        // sequencing run after it's been created
        diesel::insert_into(sequencing_submissions::table)
            .values(self.libraries)
            .execute(db_conn)?;

        SequencingRunId(id).execute(db_conn)
    }
}
