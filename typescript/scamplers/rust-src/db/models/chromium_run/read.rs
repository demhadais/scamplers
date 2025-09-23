use diesel::prelude::*;
use scamplers_schema::chromium_run;

use crate::{
    apply_eq_any_filters, apply_eq_filters, apply_ilike_filters, apply_jsonb_contains_filters,
    apply_tenx_assay_query, apply_time_filters, attach_children_to_parents,
    db::{
        DbOperation,
        models::chromium_run::{
            ChromiumRun, ChromiumRunId, ChromiumRunOrderBy, ChromiumRunQuery,
            ChromiumRunSummaryWithParents, Gems,
        },
    },
    group_children, impl_id_db_operation, init_stmt,
};

impl DbOperation<Vec<ChromiumRun>> for ChromiumRunQuery {
    fn execute(
        mut self,
        db_conn: &mut diesel::PgConnection,
    ) -> crate::result::ScamplersResult<Vec<ChromiumRun>> {
        let mut stmt = init_stmt!(
            ChromiumRunSummaryWithParents,
            query = &self,
            orderby_spec = { ChromiumRunOrderBy::RunAt => chromium_run::run_at }
        );

        if let Some(tenx_assay_query) = self.assay.take() {
            stmt = apply_tenx_assay_query!(stmt, tenx_assay_query);
        }

        let Self {
            ids,
            readable_ids,
            run_before,
            run_after,
            succeeded,
            additional_data,
            ..
        } = &self;

        stmt = apply_eq_any_filters!(
            stmt,
            filters = {
                chromium_run::id => ids
            }
        );

        stmt = apply_ilike_filters!(
            stmt,
            filters = {
                chromium_run::readable_id => readable_ids
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

        stmt = apply_jsonb_contains_filters!(stmt,
            filters = {
                chromium_run::additional_data => additional_data
            }
        );

        let chromium_run_summaries = stmt.load(db_conn)?;

        let gems = Gems::belonging_to(&chromium_run_summaries)
            .select(Gems::as_select())
            .load(db_conn)?;
        let gems = group_children!(parents = chromium_run_summaries, children = gems);

        Ok(attach_children_to_parents!(
            parents = chromium_run_summaries,
            children = [gems],
            transform_fn = |(info, gems)| ChromiumRun { info, gems }
        ))
    }
}

impl_id_db_operation!(
    id_type = ChromiumRunId,
    delegate_to = ChromiumRunQuery,
    returns = ChromiumRun
);
