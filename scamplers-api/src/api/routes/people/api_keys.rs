use diesel::RunQueryDsl;
use diesel::prelude::*;
use scamplers_schema::api_keys;
use uuid::Uuid;

use crate::api::extract::auth::User;
use crate::api::routes::ApiKeyPrefix;
use crate::{api::extract::auth::ApiKey, db};

#[derive(Insertable)]
#[diesel(table_name = api_keys)]
struct ApiKeyCreation<'a> {
    prefix: &'a str,
    hash: String,
    user_id: Uuid,
}

impl db::Operation<ApiKey> for User {
    fn execute(self, db_conn: &mut PgConnection) -> Result<ApiKey, db::Error> {
        let api_key = ApiKey::new();
        let hash = api_key.hash();

        diesel::insert_into(api_keys::table)
            .values(ApiKeyCreation {
                prefix: api_key.prefix(),
                hash,
                user_id: self.0,
            })
            .execute(db_conn)?;

        Ok(api_key)
    }
}

impl db::Operation<()> for (String, User) {
    fn execute(self, db_conn: &mut diesel::PgConnection) -> Result<(), db::Error> {
        let (prefix, User(user_id)) = self;

        let filter = api_keys::prefix
            .eq(prefix)
            .and(api_keys::user_id.eq(user_id));

        diesel::delete(api_keys::table.filter(filter)).execute(db_conn)?;

        Ok(())
    }
}
