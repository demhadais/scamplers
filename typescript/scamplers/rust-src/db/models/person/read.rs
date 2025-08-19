use diesel::{prelude::*, sql_types::Text};
use scamplers_schema::{institution, person};

use crate::{
    apply_eq_any_filters, apply_eq_filters, apply_ilike_filters,
    db::{
        DbOperation,
        models::person::{
            Person, PersonId, PersonOrderBy, PersonQuery, PersonSummaryWithRelations,
        },
    },
    impl_id_db_operation, init_stmt,
};

diesel::define_sql_function! {fn get_user_roles(user_id: Text) -> Array<Text>}

impl DbOperation<Vec<Person>> for PersonQuery {
    fn execute(
        self,
        db_conn: &mut diesel::PgConnection,
    ) -> crate::result::ScamplersResult<Vec<Person>> {
        let mut stmt = init_stmt!(stmt = person::table.inner_join(institution::table), query = &self, output_type = PersonSummaryWithRelations, orderby_spec = {PersonOrderBy::Name => person::name, PersonOrderBy::Email => person::email });

        let Self {
            ids,
            name,
            email,
            orcid,
            ms_user_id,
            ..
        } = &self;

        stmt = apply_eq_any_filters!(
            stmt,
            filters = {person::id => ids}
        );

        stmt = apply_ilike_filters!(
            stmt,
            filters = {
                person::name => name,
                person::email => email
            }
        );

        stmt = apply_eq_filters!(
            stmt,
            filters = {
                person::orcid => orcid,
                person::ms_user_id => ms_user_id
            }
        );

        let person_cores: Vec<PersonSummaryWithRelations> = stmt.load(db_conn)?;

        let mut people = Vec::with_capacity(person_cores.len());
        for info in person_cores {
            let roles = diesel::select(get_user_roles(info.id_.to_string())).get_result(db_conn)?;
            people.push(Person { info, roles });
        }

        Ok(people)
    }
}

impl_id_db_operation!(
    id_type = PersonId,
    delegate_to = PersonQuery,
    returns = Person
);

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use deadpool_diesel::postgres::Connection;
    use diesel::{Connection as _, prelude::*};
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use uuid::Uuid;

    use crate::{
        db::{
            DbOperation,
            models::{
                institution::InstitutionQuery,
                person::{
                    CreatedUser, NewPerson, Person, PersonOrderBy, PersonQuery, PersonUpdate,
                    PersonUpdateFields, UserRole,
                },
            },
            test_util::{db_conn, people, test_query},
        },
        result::ScamplersError,
    };

    fn sort_by_name(p1: &Person, p2: &Person) -> Ordering {
        p1.info.summary.name.cmp(&p2.info.summary.name)
    }

    #[rstest]
    #[awt]
    #[tokio::test]
    async fn default_person_query(#[future] db_conn: Connection, #[future] people: Vec<Person>) {
        test_query::<PersonQuery, _>()
            .all_data(people)
            .sort_by(sort_by_name)
            .run(db_conn)
            .await;
    }

    #[rstest]
    #[awt]
    #[tokio::test]
    async fn specific_person_query(#[future] db_conn: Connection, #[future] people: Vec<Person>) {
        let query = PersonQuery::builder()
            .name("person1")
            .order_by(PersonOrderBy::Name { descending: true })
            .build();

        test_query()
            .all_data(people)
            .filter(|p| p.info.summary.name.starts_with("person1"))
            .sort_by(|p1, p2| sort_by_name(p1, p2).reverse())
            .db_query(query)
            .run(db_conn)
            .await;
    }

    #[rstest]
    #[awt]
    #[tokio::test]
    async fn update_user_info(#[future] db_conn: Connection) {
        let db_conn = &mut db_conn.lock().unwrap();

        db_conn.test_transaction::<_, ScamplersError, _>(move |tx| {
            let people = PersonQuery::default().execute(tx).unwrap();

            let id = people.get(0).unwrap().info.id_;

            let new_name = "Thomas Anderson";
            let new_email = "neo@example.com";

            let updated_person = PersonUpdateFields::builder()
                .id(id)
                .name(new_name)
                .email(new_email)
                .build();
            assert!(updated_person.is_update());

            let updated_person = PersonUpdate {
                fields: updated_person,
                ..Default::default()
            }
            .execute(tx)
            .unwrap();

            assert_eq!(new_name, updated_person.info.summary.name);
            assert_eq!(new_email, updated_person.info.summary.email.unwrap());

            Ok(())
        });
    }

    #[rstest]
    #[awt]
    #[tokio::test]
    async fn ms_login_with_roles_update(#[future] db_conn: Connection) {
        let db_conn = &mut db_conn.lock().unwrap();

        db_conn.test_transaction::<_, ScamplersError, _>(move |tx| {
            {
                let institution_id = InstitutionQuery::default().execute(tx).unwrap()[0].id;

                // First, write a new user to the db as a login from the frontend
                let ms_user_id = Uuid::now_v7();

                let mut spiderman = NewPerson::builder()
                    .name("Peter Parker")
                    .email("peter.parker@example.com")
                    .ms_user_id(ms_user_id)
                    .institution_id(institution_id)
                    .build();

                let created_user: CreatedUser = spiderman.clone().execute(tx).unwrap();

                // The user logs out and changes their email address, then logs back in (he's
                // okay with revealing his identity)
                let new_email = "spider.man@example.com".to_string();
                spiderman.email = new_email.clone();
                let recreated_user: CreatedUser = spiderman.execute(tx).unwrap();

                assert_eq!(created_user.person.info.id_, recreated_user.person.info.id_);
                assert_eq!(
                    new_email,
                    *recreated_user.person.info.summary.email.as_ref().unwrap()
                );
                assert_eq!(recreated_user.person.roles, &[]);

                let fields = PersonUpdateFields::builder()
                    .id(created_user.person.info.id_)
                    .build();
                let person_with_granted_roles = PersonUpdate {
                    fields: fields.clone(),
                    grant_roles: vec![UserRole::AppAdmin],
                    ..Default::default()
                }
                .execute(tx)
                .unwrap();

                assert_eq!(person_with_granted_roles.roles, &[UserRole::AppAdmin]);

                let person_with_revoked_roles = PersonUpdate {
                    fields,
                    revoke_roles: vec![UserRole::AppAdmin],
                    ..Default::default()
                }
                .execute(tx)
                .unwrap();

                assert_eq!(person_with_revoked_roles.roles, &[]);

                Ok(())
            }
        });
    }
}
