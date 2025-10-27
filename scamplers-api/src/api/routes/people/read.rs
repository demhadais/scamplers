use diesel::{dsl::AssumeNotNull, prelude::*, sql_types::Text};
use scamplers_models::person::{
    self, OrdinalColumns, Person, PersonId, PersonSummary, PersonSummaryWithParents,
};
use scamplers_schema::people;

use crate::{
    apply_eq_any_filters, apply_ilike_filters,
    db::{self, AsBoxedFilter, BoxedFilterExt},
    init_stmt,
};

diesel::define_sql_function! {fn get_user_roles(user_id: Text) -> Array<Text>}

impl db::Operation<Person> for PersonId {
    fn execute(self, db_conn: &mut PgConnection) -> Result<Person, db::Error> {
        let info = PersonSummaryWithParents::query()
            .filter(people::id.eq(&self))
            .first(db_conn)?;

        // Use the inner `Uuid`'s `Display` implementation because the outer type's
        // implementation is provided by axum
        let roles = diesel::select(get_user_roles(self.to_id_string())).get_result(db_conn)?;

        Ok(Person::builder().info(info).roles(roles).build())
    }
}

impl<'a, QS: 'a> AsBoxedFilter<'a, QS> for person::Filter
where
    AssumeNotNull<people::id>: SelectableExpression<QS>,
    AssumeNotNull<people::name>: SelectableExpression<QS>,
    AssumeNotNull<people::email>: SelectableExpression<QS>,
    AssumeNotNull<people::orcid>: SelectableExpression<QS>,
    AssumeNotNull<people::ms_user_id>: SelectableExpression<QS>,
{
    fn as_boxed_filter(&'a self) -> db::OptionalBoxedFilter<'a, QS> {
        let Self {
            ids,
            names,
            emails,
            orcids,
            ms_user_ids,
        } = &self;

        let mut filter = db::OptionalBoxedFilter::new();

        filter = apply_eq_any_filters!(
            filter,
            filters = {
                people::id => ids,
                people::orcid => orcids,
                people::ms_user_id => ms_user_ids
            }
        );

        filter = apply_ilike_filters!(
            filter,
            filters = {
                people::name => names,
                people::email => emails
            }
        );

        filter
    }
}

impl db::Operation<Vec<PersonSummary>> for person::Query {
    fn execute(self, db_conn: &mut diesel::PgConnection) -> Result<Vec<PersonSummary>, db::Error> {
        let mut stmt = init_stmt!(PersonSummary, query = &self, orderby_spec = {OrdinalColumns::Id => people::id, OrdinalColumns::Name => people::name, OrdinalColumns::Email => people::email });

        if let Some(filter_conditions) = self.filter.as_boxed_filter() {
            stmt = stmt.filter(filter_conditions);
        }

        Ok(stmt.load(db_conn)?)
    }
}
