use diesel::prelude::*;
use scamplers_schema::cdna;

use crate::{
    apply_eq_any_filters, attach_children_to_parents,
    db::{
        DbOperation,
        models::nucleic_acid::cdna::{
            Cdna, CdnaId, CdnaMeasurement, CdnaOrderBy, CdnaPreparer, CdnaQuery, CdnaSummary,
        },
    },
    group_children, group_preparers, impl_id_db_operation, init_stmt,
};

impl DbOperation<Vec<Cdna>> for CdnaQuery {
    fn execute(
        self,
        db_conn: &mut diesel::PgConnection,
    ) -> crate::result::ScamplersResult<Vec<Cdna>> {
        let mut stmt = init_stmt!(
            CdnaSummary,
            query = &self,
            orderby_spec = {
                CdnaOrderBy::PreparedAt => cdna::prepared_at,
                CdnaOrderBy::ReadableId => cdna::readable_id
            }
        );

        let Self { ids, .. } = &self;

        stmt = apply_eq_any_filters!(stmt, filters = { cdna::id => ids });

        let summaries = stmt.load(db_conn)?;

        let preparers = CdnaPreparer::belonging_to(&summaries)
            .select(CdnaPreparer::as_select())
            .load(db_conn)?;
        let grouped_preparers = group_preparers!(parents = summaries, children = preparers);

        let measurements = CdnaMeasurement::belonging_to(&summaries)
            .select(CdnaMeasurement::as_select())
            .load(db_conn)?;
        let grouped_measurements = group_children!(parents = summaries, children = measurements);

        let cdna = attach_children_to_parents!(
            parents = summaries,
            children = [grouped_preparers, grouped_measurements],
            transform_fn = |((summary, prepared_by), measurements)| Cdna {
                summary,
                prepared_by,
                measurements
            }
        );

        Ok(cdna)
    }
}

impl_id_db_operation!(id_type = CdnaId, delegate_to = CdnaQuery, returns = Cdna);
