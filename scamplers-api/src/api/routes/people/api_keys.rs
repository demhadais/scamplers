use diesel::{RunQueryDsl, prelude::*};
use scamplers_schema::api_keys;
use uuid::Uuid;

use crate::{
    api::{
        extract::auth::{ApiKey, AuthenticatedUser},
        routes::{CreateApiKey, DeleteApiKey},
    },
    db,
};

#[derive(Insertable)]
#[diesel(check_for_backend(diesel::pg::Pg), table_name = api_keys)]
struct ApiKeyCreation<'a> {
    prefix: &'a str,
    hash: String,
    user_id: Uuid,
}

impl db::Operation<ApiKey> for CreateApiKey {
    fn execute(self, db_conn: &mut PgConnection) -> Result<ApiKey, db::Error> {
        let api_key = ApiKey::new();
        let hash = api_key.hash();

        diesel::insert_into(api_keys::table)
            .values(ApiKeyCreation {
                prefix: api_key.prefix(),
                hash,
                user_id: self.user_id,
            })
            .execute(db_conn)?;

        Ok(api_key)
    }
}

impl db::Operation<()> for DeleteApiKey {
    fn execute(self, db_conn: &mut diesel::PgConnection) -> Result<(), db::Error> {
        let Self { prefix, user_id } = self;

        let filter = api_keys::prefix
            .eq(prefix)
            .and(api_keys::user_id.eq(user_id));

        diesel::delete(api_keys::table.filter(filter)).execute(db_conn)?;

        Ok(())
    }
}
