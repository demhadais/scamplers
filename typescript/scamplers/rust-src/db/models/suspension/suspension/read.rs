use diesel::prelude::*;
use scamplers_schema::{multiplexing_tag, specimen, suspension};

use crate::{
    apply_eq_any_filters, attach_children_to_parents,
    db::{
        DbOperation,
        models::suspension::suspension::{
            Suspension, SuspensionId, SuspensionMeasurement, SuspensionOrderBy, SuspensionPreparer,
            SuspensionQuery, SuspensionSummaryWithParents,
        },
    },
    group_children, group_preparers, impl_id_db_operation, init_stmt,
};

impl DbOperation<Vec<Suspension>> for SuspensionQuery {
    fn execute(
        mut self,
        db_conn: &mut PgConnection,
    ) -> crate::result::ScamplersResult<Vec<Suspension>> {
        let base_stmt = suspension::table
            .inner_join(specimen::table)
            .left_join(multiplexing_tag::table);

        let mut stmt = init_stmt!(stmt = base_stmt, query = &self, output_type = SuspensionSummaryWithParents, orderby_spec = { SuspensionOrderBy::CreatedAt => suspension::created_at, SuspensionOrderBy::ReadableId => suspension::readable_id });

        if let Some(specimen_query) = self.specimen.take() {
            stmt = crate::apply_specimen_query!(stmt, specimen_query);
        }

        let Self { ids, .. } = &self;

        stmt = apply_eq_any_filters!(stmt, filters = { suspension::id => ids});

        let suspension_records = stmt.load(db_conn)?;

        let measurements: Vec<SuspensionMeasurement> =
            SuspensionMeasurement::belonging_to(&suspension_records)
                .select(SuspensionMeasurement::as_select())
                .load(db_conn)?;

        let grouped_measurements =
            group_children!(parents = suspension_records, children = measurements);

        let preparers: Vec<SuspensionPreparer> =
            SuspensionPreparer::belonging_to(&suspension_records)
                .select(SuspensionPreparer::as_select())
                .load(db_conn)?;

        let grouped_preparers =
            group_preparers!(parents = suspension_records, children = preparers);

        let suspensions = attach_children_to_parents!(
            parents = suspension_records,
            children = [grouped_measurements, grouped_preparers],
            transform_fn = |((info, measurements), prepared_by)| Suspension {
                info,
                prepared_by,
                measurements
            }
        );

        Ok(suspensions)
    }
}

impl_id_db_operation! {
    id_type = SuspensionId,
    delegate_to = SuspensionQuery,
    returns = Suspension
}
