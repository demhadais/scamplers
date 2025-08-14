use diesel::{
    prelude::*,
    sql_types::{Array, Text},
};
use scamplers_schema::person;
use uuid::Uuid;
use valid_string::ValidString;

use crate::{
    auth::{ApiKey, HashedApiKey},
    db::{
        DbOperation,
        models::person::{CreatedUser, NewPerson, Person, PersonId, PersonQuery, UserRole},
    },
};

define_sql_function! {fn create_user_if_not_exists(user_id: Text, roles: Array<Text>)}

impl DbOperation<Person> for NewPerson {
    fn execute(self, db_conn: &mut diesel::PgConnection) -> crate::result::ScamplersResult<Person> {
        let id: Uuid = diesel::insert_into(person::table)
            .values(&self)
            .returning(person::id)
            .get_result(db_conn)?;

        diesel::select(create_user_if_not_exists(id.to_string(), &self.roles)).execute(db_conn)?;

        PersonId(id).execute(db_conn)
    }
}

impl DbOperation<CreatedUser> for NewPerson {
    fn execute(
        self,
        db_conn: &mut diesel::PgConnection,
    ) -> crate::result::ScamplersResult<CreatedUser> {
        #[derive(Insertable, AsChangeset, Clone, Copy)]
        #[diesel(table_name = person, primary_key(ms_user_id))]
        struct Upsert<'a> {
            ms_user_id: Option<&'a Uuid>,
            name: &'a ValidString,
            email: &'a str,
            hashed_api_key: &'a HashedApiKey,
            institution_id: &'a Uuid,
        }

        let NewPerson {
            name,
            email,
            institution_id,
            ms_user_id,
            ..
        } = &self;

        // We know that whoever just logged in is the actual owner of this email
        // address. Anyone else that has this email should have it removed from them
        diesel::update(person::table)
            .filter(person::email.eq(email))
            .set(person::email.eq(None::<String>))
            .execute(db_conn)?;

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

        let id: Uuid = diesel::insert_into(person::table)
            .values(upsert)
            .on_conflict(person::ms_user_id)
            .do_update()
            .set(upsert)
            .returning(person::id)
            .get_result(db_conn)?;

        // Create the user, but give them no roles
        let empty_roles: Vec<UserRole> = Vec::with_capacity(0);
        diesel::select(create_user_if_not_exists(id.to_string(), empty_roles)).execute(db_conn)?;

        let person = PersonId(id).execute(db_conn)?;

        Ok(CreatedUser {
            person,
            api_key: api_key.into(),
        })
    }
}
