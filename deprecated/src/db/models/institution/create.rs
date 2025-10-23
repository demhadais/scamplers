use diesel::prelude::*;
use scamplers_schema::institution;

use super::{Institution, NewInstitution};
use crate::{db::DbOperation, result::ScamplersResult};

impl DbOperation<Institution> for NewInstitution {
    fn execute(self, db_conn: &mut diesel::PgConnection) -> ScamplersResult<Institution> {
        Ok(diesel::insert_into(institution::table)
            .values(self)
            .returning(Institution::as_returning())
            .get_result(db_conn)?)
    }
}
