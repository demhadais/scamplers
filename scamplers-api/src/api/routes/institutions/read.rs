use diesel::{PgConnection, SelectableExpression, dsl::AssumeNotNull, prelude::*};
use scamplers_models::institution::{self, Institution, InstitutionId, OrdinalColumns};
use scamplers_schema::institutions;

use crate::{
    db::{self, BoxedFilter, BoxedFilterExt, ToBoxedFilter},
    query, with_eq_any, with_like,
};

impl db::Operation<Institution> for InstitutionId {
    fn execute(self, db_conn: &mut PgConnection) -> Result<Institution, db::Error> {
        Ok(Institution::query()
            .filter(institutions::id.eq(&self))
            .first(db_conn)?)
    }
}

impl<'a, QS: 'a> ToBoxedFilter<'a, QS> for institution::Filter
where
    institutions::id: SelectableExpression<QS>,
    institutions::name: SelectableExpression<QS>,
{
    fn to_boxed_filter(&'a self) -> BoxedFilter<'a, QS> {
        let Self { ids, name } = &self;

        let mut filter = BoxedFilter::new();

        if !ids.is_empty() {
            filter = filter.and_condition(institutions::id.eq_any(ids));
        }

        if let Some(name) = name {
            filter = filter.and_condition(institutions::name.like(name));
        }

        filter
    }
}

impl db::Operation<Vec<Institution>> for institution::Query {
    fn execute(self, db_conn: &mut diesel::PgConnection) -> Result<Vec<Institution>, db::Error> {
        let stmt = query!(Institution::query(self).order_by(
            OrdinalColumns::Id = institutions::id,
            OrdinalColumns::Name = institutions::name
        ));

        Ok(stmt.load(db_conn)?)
    }
}
