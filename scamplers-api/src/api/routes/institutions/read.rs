use diesel::{PgConnection, SelectableExpression, prelude::*};
use scamplers_models::institution::{self, Institution, InstitutionId, OrdinalColumns};
use scamplers_schema::institutions;

use crate::{
    db::{self, BoxedFilter, BoxedFilterExt, ToBoxedFilter},
    query,
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
        let mut filter = BoxedFilter::new();

        if let Some(ids) = self.ids() {
            filter = filter.and_condition(institutions::id.eq_any(ids));
        }

        if let Some(name) = self.name() {
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
