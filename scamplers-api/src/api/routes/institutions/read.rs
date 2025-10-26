use diesel::{PgConnection, SelectableExpression, dsl::AssumeNotNull, prelude::*};
use scamplers_models::institution::{self, Institution, InstitutionId, OrdinalColumns};
use scamplers_schema::institutions;

use crate::{
    apply_eq_any_filters, apply_ilike_filters,
    db::{self, AsBoxedFilter, BoxedFilterExt},
    init_stmt,
};

impl db::Operation<Institution> for InstitutionId {
    fn execute(self, db_conn: &mut PgConnection) -> Result<Institution, db::Error> {
        Ok(Institution::query()
            .filter(institutions::id.eq(&self))
            .first(db_conn)?)
    }
}

impl<'a, QS: 'a> AsBoxedFilter<'a, QS> for institution::Filter
where
    AssumeNotNull<institutions::id>: SelectableExpression<QS>,
    AssumeNotNull<institutions::name>: SelectableExpression<QS>,
{
    fn as_boxed_filter(&'a self) -> db::OptionalBoxedFilter<'a, QS> {
        let Self { ids, names } = &self;

        let mut filter = db::OptionalBoxedFilter::new();

        filter = apply_eq_any_filters!(
            filter,
            filters = {
                institutions::id => ids
            }
        );

        filter = apply_ilike_filters!(
            filter,
            filters = {
                institutions::name => names
            }
        );

        filter
    }
}

impl db::Operation<Vec<Institution>> for institution::Query {
    fn execute(self, db_conn: &mut diesel::PgConnection) -> Result<Vec<Institution>, db::Error> {
        let mut stmt = init_stmt!(Institution, query = &self, orderby_spec = {OrdinalColumns::Id => institutions::id, OrdinalColumns::Name => institutions::name });

        if let Some(filter_conditions) = self.filter.as_boxed_filter() {
            stmt = stmt.filter(filter_conditions);
        }

        Ok(stmt.load(db_conn)?)
    }
}
