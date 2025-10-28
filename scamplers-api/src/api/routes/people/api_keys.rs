use diesel::{RunQueryDsl, prelude::*};
use scamplers_schema::api_keys;
use uuid::Uuid;

use crate::{api::routes::DeleteApiKeyEndpoint, db};

impl db::Operation<()> for DeleteApiKeyEndpoint {
    fn execute(self, db_conn: &mut diesel::PgConnection) -> Result<(), db::Error> {
        let Self {
            api_key_prefix,
            user_id,
        } = self;

        let filter = api_keys::prefix
            .eq(api_key_prefix)
            .and(api_keys::user_id.eq(user_id));

        diesel::delete(api_keys::table.filter(filter)).execute(db_conn)?;

        Ok(())
    }
}
