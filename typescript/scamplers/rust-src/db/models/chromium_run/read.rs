use diesel::prelude::*;
use scamplers_schema::chromium_run;

use crate::{
    apply_eq_any_filters, apply_eq_filters, apply_ilike_filters, apply_time_filters,
    attach_children_to_parents,
    db::{
        DbOperation,
        models::chromium_run::{
            ChromiumRun, ChromiumRunId, ChromiumRunOrderBy, ChromiumRunQuery, ChromiumRunSummary,
            Gems,
        },
    },
    group_otm_children, impl_id_db_operation, init_stmt,
};

impl DbOperation<Vec<ChromiumRun>> for ChromiumRunQuery {
    fn execute(
        self,
        db_conn: &mut diesel::PgConnection,
    ) -> crate::result::ScamplersResult<Vec<ChromiumRun>> {
        let mut stmt = init_stmt!(
            stmt = chromium_run::table,
            query = &self,
            output_type = ChromiumRunSummary,
            orderby_spec = { ChromiumRunOrderBy::RunAt => chromium_run::run_at }
        );

        let Self {
            ids,
            readable_ids,
            chips,
            run_before,
            run_after,
            succeeded,
            notes,
            ..
        } = &self;

        stmt = apply_eq_any_filters!(
            stmt,
            filters = {
                chromium_run::id => ids,
                chromium_run::chip => chips
            }
        );

        stmt = apply_ilike_filters!(
            stmt,
            filters = {
                chromium_run::readable_id => readable_ids,
                chromium_run::notes => notes
            }
        );

        stmt = apply_eq_filters!(
            stmt,
            filters = {
                chromium_run::succeeded => succeeded
            }
        );

        stmt = apply_time_filters!(
            stmt,
            filters = {
                chromium_run::run_at => (run_before, run_after)
            }
        );

        let chromium_run_summaries = stmt.load(db_conn)?;

        let gems = Gems::belonging_to(&chromium_run_summaries)
            .select(Gems::as_select())
            .load(db_conn)?;
        let gems = group_otm_children!(parents = chromium_run_summaries, children = gems);

        Ok(attach_children_to_parents!(
            parents = chromium_run_summaries,
            children = [gems],
            transform_fn = |(summary, gems)| ChromiumRun { summary, gems }
        ))
    }
}

impl_id_db_operation!(
    id_type = ChromiumRunId,
    delegate_to = ChromiumRunQuery,
    returns = ChromiumRun
);
