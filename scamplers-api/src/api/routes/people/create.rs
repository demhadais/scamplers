use diesel::{
    RunQueryDsl,
    prelude::*,
    sql_types::{Array, Text},
};
use scamplers_models::person::{self, Person, PersonId};
use scamplers_schema::people;
use uuid::Uuid;

use crate::db;

define_sql_function! {fn create_user_if_not_exists(user_id: Text, password: Text, roles: Array<Text>)}

impl db::Operation<Person> for person::Creation {
    fn execute(self, db_conn: &mut diesel::PgConnection) -> Result<Person, db::Error> {
        // Get the ID of the inserted person first, then return the full `Person` struct
        let id: PersonId = diesel::insert_into(people::table)
            .values(&self)
            .returning(people::id)
            .get_result(db_conn)?;

        // Create a db user corresponding to this person so we can assign them a role. Note that we set a random password so that nobody can log into the database as that user.
        diesel::select(create_user_if_not_exists(
            id.to_id_string(),
            Uuid::now_v7().to_string(),
            self.roles,
        ))
        .execute(db_conn)?;

        id.execute(db_conn)
    }
}
