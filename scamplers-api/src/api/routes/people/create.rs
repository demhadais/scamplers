use diesel::{
    RunQueryDsl,
    prelude::*,
    sql_types::{Array, Text},
};
use scamplers_models::person::{self, CreatedUser, Person, PersonId};
use scamplers_schema::{institutions, people};
use uuid::Uuid;

use crate::{
    api::{extract::auth::AuthenticatedUser, routes::CreateApiKey},
    db,
};

define_sql_function! {fn create_user_if_not_exists(user_id: Text, roles: Array<Text>)}

impl db::Operation<Person> for person::Creation {
    fn execute(self, db_conn: &mut diesel::PgConnection) -> Result<Person, db::Error> {
        // Get the ID of the inserted person first, then return the full `Person` struct
        let id: PersonId = diesel::insert_into(people::table)
            .values(&self)
            .returning(people::id)
            .get_result(db_conn)?;

        diesel::select(create_user_if_not_exists(id.to_id_string(), &self.roles))
            .execute(db_conn)?;

        id.execute(db_conn)
    }
}

impl db::Operation<person::CreatedUser> for person::Creation {
    fn execute(self, db_conn: &mut diesel::PgConnection) -> Result<person::CreatedUser, db::Error> {
        use scamplers_schema::people as p;

        #[derive(Insertable, AsChangeset, Clone, Copy)]
        #[diesel(table_name = p, primary_key(ms_user_id))]
        struct Upsert<'a> {
            ms_user_id: Option<&'a Uuid>,
            name: &'a str,
            email: &'a str,
            institution_id: &'a Uuid,
        }

        let person::Creation {
            email,
            inner:
                person::Fields {
                    name,
                    institution_id,
                    ms_user_id,
                    ..
                },
            ..
        } = &self;

        // We know that whoever just logged in is the actual owner of this email
        // address. Anyone else that has this email should have it removed from them
        diesel::update(p::table)
            .filter(p::email.eq(email))
            .set(p::email.eq(None::<String>))
            .execute(db_conn)?;

        let upsert = Upsert {
            ms_user_id: ms_user_id.as_ref(),
            name: name.as_ref(),
            email: email.as_ref(),
            institution_id,
        };

        let id: Uuid = diesel::insert_into(p::table)
            .values(upsert)
            .on_conflict(p::ms_user_id)
            .do_update()
            .set(upsert)
            .returning(p::id)
            .get_result(db_conn)?;

        // Create the user, but give them no roles. Note that we use the inner `Uuid`'s
        // `Display` implementation
        let empty_roles: Vec<String> = Vec::with_capacity(0);
        diesel::select(create_user_if_not_exists(id.to_string(), empty_roles)).execute(db_conn)?;

        let api_key = CreateApiKey { user_id: id }.execute_as_user(id, db_conn)?;

        Ok(CreatedUser::builder()
            .inner(PersonId(id).execute(db_conn)?)
            .api_key(api_key)
            .build())
    }
}
