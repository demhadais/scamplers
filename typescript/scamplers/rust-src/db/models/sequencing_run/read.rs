use diesel::prelude::*;
use scamplers_schema::sequencing_run;

use crate::{
    apply_eq_any_filters, attach_children_to_parents,
    db::{
        DbOperation,
        models::sequencing_run::{
            NewSequencingSubmission, SequencingRun, SequencingRunId, SequencingRunOrderBy,
            SequencingRunQuery, SequencingRunSummary,
        },
    },
    group_children, impl_id_db_operation, init_stmt,
};

impl DbOperation<Vec<SequencingRun>> for SequencingRunQuery {
    fn execute(
        self,
        db_conn: &mut diesel::PgConnection,
    ) -> crate::result::ScamplersResult<Vec<SequencingRun>> {
        let base_stmt = sequencing_run::table;

        let mut stmt = init_stmt!(stmt = base_stmt, query = &self, output_type = SequencingRunSummary, orderby_spec = { SequencingRunOrderBy::BegunAt => sequencing_run::begun_at, SequencingRunOrderBy::FinishedAt => sequencing_run::finished_at, SequencingRunOrderBy::ReadableId => sequencing_run::readable_id });

        let Self { ids, .. } = &self;

        stmt = apply_eq_any_filters!(stmt, filters = { sequencing_run::id => ids });
        let sequencing_run_summaries = stmt.load(db_conn)?;

        // We use NewSequencingSubmission because it conveniently has the fields we need
        let libraries = NewSequencingSubmission::belonging_to(&sequencing_run_summaries)
            .select(NewSequencingSubmission::as_select())
            .load(db_conn)?;

        // We group them, then exctract just the library IDs
        let grouped_libraries =
            group_children!(parents = sequencing_run_summaries, children = libraries)
                .map(|v| v.into_iter().map(|l| l.library_id).collect());

        let sequencing_runs = attach_children_to_parents!(
            parents = sequencing_run_summaries,
            children = [grouped_libraries],
            transform_fn = |(summary, libraries)| SequencingRun { summary, libraries }
        );

        Ok(sequencing_runs)
    }
}

impl_id_db_operation! { id_type = SequencingRunId, delegate_to = SequencingRunQuery, returns = SequencingRun }
