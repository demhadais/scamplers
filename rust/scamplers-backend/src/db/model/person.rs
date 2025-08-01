use diesel::{
    define_sql_function,
    dsl::{AssumeNotNull, InnerJoin},
    prelude::*,
    sql_types::{Array, Text},
};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use scamplers_core::model::person::{
    CreatedUser, NewMsLogin, NewPerson, Person, PersonCore, PersonQuery, PersonSummary,
    PersonUpdate, PersonUpdateCore, UserRole,
};
use scamplers_schema::{
    institution,
    person::{
        self,
        dsl::{
            email as email_col, id as id_col, ms_user_id as ms_user_id_col, name as name_col,
            orcid as orcid_col,
        },
    },
};
use uuid::Uuid;
use valid_string::ValidString;

use crate::{
    db::{
        model::{self, AsDieselQueryBase, FetchById, IsUpdate, WriteToDb},
        util::{AsIlike, BoxedDieselExpression, NewBoxedDieselExpression},
    },
    fetch_by_query,
    result::ScamplersResult,
    server::auth::{ApiKey, HashedApiKey},
};

define_sql_function! {fn grant_roles_to_user(user_id: Text, roles: Array<Text>)}
define_sql_function! {fn revoke_roles_from_user(user_id: Text, roles: Array<Text>)}
define_sql_function! {fn create_user_if_not_exists(user_id: Text, roles: Array<Text>)}
define_sql_function! {fn get_user_roles(user_id: Text) -> Array<Text>}

impl<QuerySource> model::AsDieselFilter<QuerySource> for PersonQuery
where
    id_col: SelectableExpression<QuerySource>,
    name_col: SelectableExpression<QuerySource>,
    AssumeNotNull<email_col>: SelectableExpression<QuerySource>,
    AssumeNotNull<orcid_col>: SelectableExpression<QuerySource>,
    AssumeNotNull<ms_user_id_col>: SelectableExpression<QuerySource>,
{
    fn as_diesel_filter<'a>(&'a self) -> Option<BoxedDieselExpression<'a, QuerySource>>
    where
        QuerySource: 'a,
    {
        let Self {
            ids,
            name,
            email,
            orcid,
            ms_user_id,
            ..
        } = self;

        let q1 = (!ids.is_empty()).then(|| id_col.eq_any(ids));
        let q2 = name.as_ref().map(|name| name_col.ilike(name.as_ilike()));
        let q3 = email
            .as_ref()
            .map(|email| email_col.assume_not_null().ilike(email.as_ilike()));
        let q4 = orcid
            .as_ref()
            .map(|orcid| orcid_col.assume_not_null().ilike(orcid.as_ilike()));
        let q5 = ms_user_id
            .as_ref()
            .map(|id| ms_user_id_col.assume_not_null().eq(id));

        BoxedDieselExpression::new_expression()
            .and_condition(q1)
            .and_condition(q2)
            .and_condition(q3)
            .and_condition(q4)
            .and_condition(q5)
            .build()
    }
}

impl AsDieselQueryBase for PersonSummary {
    type QueryBase = person::table;

    fn as_diesel_query_base() -> Self::QueryBase {
        person::table
    }
}

impl model::FetchById for PersonSummary {
    type Id = Uuid;

    async fn fetch_by_id(id: &Self::Id, db_conn: &mut AsyncPgConnection) -> ScamplersResult<Self> {
        Ok(Self::as_diesel_query_base()
            .find(id)
            .select(Self::as_select())
            .first(db_conn)
            .await?)
    }
}

impl model::FetchByQuery for PersonSummary {
    type QueryParams = PersonQuery;

    async fn fetch_by_query(
        query: &Self::QueryParams,
        db_conn: &mut AsyncPgConnection,
    ) -> ScamplersResult<Vec<Self>> {
        use scamplers_core::model::person::PersonOrdinalColumn::{Email, Name};

        fetch_by_query!(query, [(Name, name_col), (Email, email_col)], db_conn)
    }
}

impl AsDieselQueryBase for Person {
    type QueryBase = InnerJoin<person::table, institution::table>;

    fn as_diesel_query_base() -> Self::QueryBase {
        PersonSummary::as_diesel_query_base().inner_join(institution::table)
    }
}

impl model::FetchById for Person {
    type Id = Uuid;

    async fn fetch_by_id(id: &Self::Id, db_conn: &mut AsyncPgConnection) -> ScamplersResult<Self> {
        let core = Self::as_diesel_query_base()
            .select(PersonCore::as_select())
            .filter(id_col.eq(id))
            .get_result(db_conn)
            .await?;

        let roles: Vec<UserRole> =
            diesel::select(get_user_roles(core.summary.handle.id.to_string()))
                .get_result(db_conn)
                .await?;

        Ok(Person { core, roles })
    }
}

impl model::WriteToDb for NewPerson {
    type Returns = Person;

    async fn write_to_db(self, db_conn: &mut AsyncPgConnection) -> ScamplersResult<Self::Returns> {
        let id = diesel::insert_into(person::table)
            .values(self)
            .returning(id_col)
            .get_result(db_conn)
            .await?;

        Person::fetch_by_id(&id, db_conn).await
    }
}

impl IsUpdate<5> for PersonUpdateCore {
    fn fields_are_some(&self) -> [bool; 5] {
        let Self {
            name,
            email,
            ms_user_id,
            orcid,
            institution_id,
            ..
        } = self;
        [
            name.is_some(),
            email.is_some(),
            ms_user_id.is_some(),
            orcid.is_some(),
            institution_id.is_some(),
        ]
    }
}

impl model::WriteToDb for PersonUpdate {
    type Returns = Person;

    async fn write_to_db(self, db_conn: &mut AsyncPgConnection) -> ScamplersResult<Self::Returns> {
        let Self {
            core,
            grant_roles,
            revoke_roles,
        } = self;

        if core.is_update() {
            diesel::update(&core).set(&core).execute(db_conn).await?;
        }

        let user_id = &core.id;
        let user_id_str = user_id.to_string();

        diesel::select(grant_roles_to_user(&user_id_str, grant_roles))
            .execute(db_conn)
            .await?;

        diesel::select(revoke_roles_from_user(&user_id_str, revoke_roles))
            .execute(db_conn)
            .await?;

        Person::fetch_by_id(user_id, db_conn).await
    }
}

impl WriteToDb for NewMsLogin {
    type Returns = CreatedUser;

    async fn write_to_db(self, db_conn: &mut AsyncPgConnection) -> ScamplersResult<Self::Returns> {
        #[derive(Insertable, AsChangeset, Clone, Copy)]
        #[diesel(table_name = person, primary_key(ms_user_id))]
        struct Upsert<'a> {
            ms_user_id: Option<&'a Uuid>,
            name: &'a ValidString,
            email: &'a str,
            hashed_api_key: &'a HashedApiKey,
            institution_id: &'a Uuid,
        }

        let Self(NewPerson {
            name,
            email,
            institution_id,
            ms_user_id,
            ..
        }) = &self;

        // TODO: We shouldn't overwrite the user's API key on every single login
        let api_key = ApiKey::new();
        let hashed_api_key = api_key.hash();

        let upsert = Upsert {
            ms_user_id: ms_user_id.as_ref(),
            name,
            email,
            hashed_api_key: &hashed_api_key,
            institution_id,
        };

        // We know that whoever just logged in is the actual owner of this email
        // address. Anyone else that has this email should have it removed from them
        diesel::update(person::table)
            .filter(email_col.eq(email))
            .set(email_col.eq(None::<String>))
            .execute(db_conn)
            .await?;

        let id: Uuid = diesel::insert_into(person::table)
            .values(upsert)
            .on_conflict(ms_user_id_col)
            .do_update()
            .set(upsert)
            .returning(id_col)
            .get_result(db_conn)
            .await?;

        // Create the user, but give them no roles
        let empty_roles: Vec<UserRole> = Vec::with_capacity(0);
        diesel::select(create_user_if_not_exists(id.to_string(), empty_roles))
            .execute(db_conn)
            .await?;

        let person = Person::fetch_by_id(&id, db_conn).await?;

        Ok(CreatedUser {
            person,
            api_key: api_key.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use diesel_async::{AsyncConnection, scoped_futures::ScopedFutureExt};
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use scamplers_core::model::{
        institution::{Institution, InstitutionQuery},
        person::{
            NewMsLogin, NewPerson, Person, PersonOrdinalColumn, PersonQuery, PersonSummary,
            PersonUpdate, PersonUpdateCore, UserRole,
        },
    };
    use uuid::Uuid;

    use crate::{
        config::LOGIN_USER,
        db::{
            DbTransaction,
            model::{FetchByQuery, IsUpdate, WriteToDb},
            test_util::{DbConnection, db_conn, people, test_query},
        },
        result::ScamplersError,
    };

    fn extract(p: Person) -> PersonSummary {
        p.core.summary
    }

    fn sort_by_name(p1: &PersonSummary, p2: &PersonSummary) -> Ordering {
        p1.name.cmp(&p2.name)
    }

    #[rstest]
    #[awt]
    #[tokio::test]
    async fn default_person_query(#[future] db_conn: DbConnection, #[future] people: Vec<Person>) {
        test_query()
            .all_data(people)
            .extract(extract)
            .sort_by(sort_by_name)
            .db_query(PersonQuery::default())
            .run(db_conn)
            .await;
    }

    #[rstest]
    #[awt]
    #[tokio::test]
    async fn specific_person_query(#[future] db_conn: DbConnection, #[future] people: Vec<Person>) {
        let query = PersonQuery::builder()
            .name("person1")
            .order_by((PersonOrdinalColumn::Name, true))
            .build();

        test_query()
            .all_data(people)
            .extract(extract)
            .filter(|p| p.name.starts_with("person1"))
            .sort_by(|p1, p2| sort_by_name(p1, p2).reverse())
            .db_query(query)
            .run(db_conn)
            .await;
    }

    #[rstest]
    #[awt]
    #[tokio::test]
    async fn update_user_info(#[future] mut db_conn: DbConnection) {
        db_conn
            .test_transaction::<_, ScamplersError, _>(|tx| {
                async move {
                    let people = PersonSummary::fetch_by_query(&PersonQuery::default(), tx)
                        .await
                        .unwrap();

                    let id = people.get(0).unwrap().handle.id;

                    let new_name = "Thomas Anderson";
                    let new_email = "neo@example.com";

                    let updated_person = PersonUpdateCore::builder()
                        .id(id)
                        .name(new_name)
                        .email(new_email)
                        .build();
                    assert!(updated_person.is_update());

                    let updated_person = PersonUpdate {
                        core: updated_person,
                        ..Default::default()
                    }
                    .write_to_db(tx)
                    .await
                    .unwrap();

                    assert_eq!(new_name, updated_person.core.summary.name);
                    assert_eq!(new_email, updated_person.core.summary.email.unwrap());

                    Ok(())
                }
                .scope_boxed()
            })
            .await;
    }

    #[rstest]
    #[awt]
    #[tokio::test]
    async fn ms_login_with_roles_update(#[future] mut db_conn: DbConnection) {
        db_conn
            .test_transaction::<_, ScamplersError, _>(|tx| {
                async move {
                    tx.set_transaction_user(LOGIN_USER).await.unwrap();

                    let institution_id =
                        Institution::fetch_by_query(&InstitutionQuery::default(), tx)
                            .await
                            .unwrap()
                            .get(0)
                            .unwrap()
                            .handle
                            .id;

                    // First, write a new user to the db as a login from the frontend
                    let ms_user_id = Uuid::now_v7();

                    let spiderman = NewPerson::builder()
                        .name("Peter Parker")
                        .email("peter.parker@example.com")
                        .ms_user_id(ms_user_id)
                        .institution_id(institution_id)
                        .build();
                    let mut new_ms_login = NewMsLogin(spiderman);

                    let created_user = new_ms_login.clone().write_to_db(tx).await.unwrap();

                    // The user logs out and changes their email address, then logs back in
                    let new_email = "spider.man@example.com".to_string();
                    new_ms_login.0.email = new_email.clone();
                    let recreated_user = new_ms_login.write_to_db(tx).await.unwrap();

                    assert_eq!(
                        created_user.person.core.summary.handle.id,
                        recreated_user.person.core.summary.handle.id
                    );
                    assert_eq!(
                        new_email,
                        *recreated_user.person.core.summary.email.as_ref().unwrap()
                    );
                    assert_eq!(recreated_user.person.roles, &[]);

                    tx.set_transaction_user("postgres").await.unwrap();

                    let core = PersonUpdateCore::builder()
                        .id(created_user.person.core.summary.handle.id)
                        .build();
                    let person_with_granted_roles = PersonUpdate {
                        core: core.clone(),
                        grant_roles: vec![UserRole::AppAdmin],
                        ..Default::default()
                    }
                    .write_to_db(tx)
                    .await
                    .unwrap();

                    assert_eq!(person_with_granted_roles.roles, &[UserRole::AppAdmin]);

                    let person_with_revoked_roles = PersonUpdate {
                        core,
                        revoke_roles: vec![UserRole::AppAdmin],
                        ..Default::default()
                    }
                    .write_to_db(tx)
                    .await
                    .unwrap();

                    assert_eq!(person_with_revoked_roles.roles, &[]);

                    Ok(())
                }
                .scope_boxed()
            })
            .await;
    }
}
