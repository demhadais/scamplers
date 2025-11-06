use diesel::{dsl::AssumeNotNull, prelude::*, sql_types::Text};
use scamplers_models::person::{
    self, OrdinalColumns, Person, PersonId, PersonSummary, PersonSummaryWithParents,
};
use scamplers_schema::people;

use crate::{
    db::{self, BoxedFilter, BoxedFilterExt, ToBoxedFilter},
    query,
};

diesel::define_sql_function! {fn get_user_roles(user_id: Text) -> Array<Text>}

impl db::Operation<Person> for PersonId {
    fn execute(self, db_conn: &mut PgConnection) -> Result<Person, db::Error> {
        let info = PersonSummaryWithParents::query()
            .filter(people::id.eq(&self))
            .first(db_conn)?;

        let roles = diesel::select(get_user_roles(self.to_id_string())).get_result(db_conn)?;

        Ok(Person::builder().info(info).roles(roles).build())
    }
}

impl<'a, QS: 'a> ToBoxedFilter<'a, QS> for person::Filter
where
    people::id: SelectableExpression<QS>,
    people::name: SelectableExpression<QS>,
    AssumeNotNull<people::email>: SelectableExpression<QS>,
    AssumeNotNull<people::orcid>: SelectableExpression<QS>,
    AssumeNotNull<people::microsoft_entra_oid>: SelectableExpression<QS>,
{
    fn to_boxed_filter(&'a self) -> BoxedFilter<'a, QS> {
        let Self {
            ids,
            name,
            email,
            orcid,
            microsoft_entra_oids,
        } = &self;

        let mut filter = BoxedFilter::new();

        if !ids.is_empty() {
            filter = filter.and_condition(people::id.eq_any(ids));
        }

        if let Some(name) = name {
            filter = filter.and_condition(people::name.like(name));
        }

        if let Some(email) = email {
            filter = filter.and_condition(people::email.assume_not_null().like(email));
        }

        if let Some(orcid) = orcid {
            filter = filter.and_condition(people::orcid.assume_not_null().like(orcid));
        }

        if !microsoft_entra_oids.is_empty() {
            filter = filter.and_condition(
                people::microsoft_entra_oid
                    .assume_not_null()
                    .eq_any(microsoft_entra_oids),
            );
        }

        filter
    }
}

impl db::Operation<Vec<PersonSummary>> for person::Query {
    fn execute(self, db_conn: &mut diesel::PgConnection) -> Result<Vec<PersonSummary>, db::Error> {
        let stmt = query!(PersonSummary::query(self).order_by(
            OrdinalColumns::Id = people::id,
            OrdinalColumns::Name = people::name,
            OrdinalColumns::Email = people::email
        ));

        Ok(stmt.load(db_conn)?)
    }
}
