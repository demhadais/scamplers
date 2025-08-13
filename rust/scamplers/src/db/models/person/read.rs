use diesel::{prelude::*, sql_types::Text};
use scamplers_schema::{institution, person};

use crate::{
    apply_eq_any_filters, apply_eq_filters, apply_ilike_filters,
    db::{
        DbOperation,
        models::person::{Person, PersonCore, PersonId, PersonOrderBy, PersonQuery},
    },
    impl_id_db_operation, init_stmt,
};

diesel::define_sql_function! {fn get_user_roles(user_id: Text) -> Array<Text>}

impl DbOperation<Vec<Person>> for PersonQuery {
    fn execute(
        self,
        db_conn: &mut diesel::PgConnection,
    ) -> crate::result::ScamplersResult<Vec<Person>> {
        let mut stmt = init_stmt!(stmt = person::table.inner_join(institution::table), query = &self, output = PersonCore; PersonOrderBy::Name => person::name, PersonOrderBy::Email => person::email);

        let Self {
            ids,
            name,
            email,
            orcid,
            ms_user_id,
            ..
        } = &self;

        stmt = apply_eq_any_filters!(
            stmt;
            person::id, ids
        );

        stmt = apply_ilike_filters!(
            stmt;
            person::name, name;
            person::email, email
        );

        stmt = apply_eq_filters!(
            stmt;
            person::orcid, orcid;
            person::ms_user_id, ms_user_id
        );

        let person_cores: Vec<PersonCore> = stmt.load(db_conn)?;

        let mut people = Vec::with_capacity(person_cores.len());
        for core in person_cores {
            let roles =
                diesel::select(get_user_roles(core.summary.id.to_string())).get_result(db_conn)?;
            people.push(Person { core, roles });
        }

        Ok(people)
    }
}

impl_id_db_operation!(PersonId, PersonQuery, Person);
