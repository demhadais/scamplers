use diesel::prelude::*;
use scamplers_models::person::{self, OrdinalColumns, Person, PersonId, PersonSummary, Query};
use scamplers_schema::people;

use crate::{apply_eq_any_filters, apply_ilike_filters, db, impl_id_db_operation, init_stmt};

impl db::Operation<Person> for PersonId {
    fn execute(self, db_conn: &mut PgConnection) -> Result<Person, db::Error> {
        Ok(Person::query()
            .filter(people::id.eq(self.0))
            .first(db_conn)?)
    }
}

impl db::Operation<Vec<PersonSummary>> for person::Query {
    fn execute(self, db_conn: &mut diesel::PgConnection) -> Result<Vec<PersonSummary>, db::Error> {
        let mut stmt = init_stmt!(PersonSummary, query = &self, orderby_spec = {OrdinalColumns::Id => people::id, OrdinalColumns::Name => people::name, OrdinalColumns::Email => people::email });

        let Self {
            ids,
            names,
            emails,
            orcids,
            ms_user_ids,
            ..
        } = &self;

        stmt = apply_eq_any_filters!(
            stmt,
            filters = {
                people::id => ids,
                people::orcid => orcids,
                people::ms_user_id => ms_user_ids
            }
        );

        stmt = apply_ilike_filters!(
            stmt,
            filters = {
                people::name => names,
                people::email => emails
            }
        );

        Ok(stmt.load(db_conn)?)
    }
}
