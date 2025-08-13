use diesel::{
    RunQueryDsl, define_sql_function,
    sql_types::{Array, Text},
};

use crate::db::{
    DbOperation,
    models::person::{Person, PersonId, PersonUpdate},
};

define_sql_function! {fn grant_roles_to_user(user_id: Text, roles: Array<Text>)}
define_sql_function! {fn revoke_roles_from_user(user_id: Text, roles: Array<Text>)}

impl DbOperation<Person> for PersonUpdate {
    fn execute(self, db_conn: &mut diesel::PgConnection) -> crate::result::ScamplersResult<Person> {
        if self.core.is_update() {
            diesel::update(&self.core)
                .set(&self.core)
                .execute(db_conn)?;
        }

        let user_id = &self.core.id;
        let user_id_str = user_id.to_string();

        diesel::select(grant_roles_to_user(&user_id_str, self.grant_roles)).execute(db_conn)?;

        diesel::select(revoke_roles_from_user(&user_id_str, self.revoke_roles)).execute(db_conn)?;

        PersonId(*user_id).execute(db_conn)
    }
}
