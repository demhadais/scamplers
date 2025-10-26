use diesel::prelude::*;
use scamplers_models::institution::{self, Institution};
use scamplers_schema::institutions;

use crate::db;

impl db::Operation<Institution> for institution::Creation {
    fn execute(self, db_conn: &mut diesel::PgConnection) -> Result<Institution, db::Error> {
        Ok(diesel::insert_into(institutions::table)
            .values(self)
            .returning(Institution::as_returning())
            .get_result(db_conn)?)
    }
}
