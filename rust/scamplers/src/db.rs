use diesel::{Connection, PgConnection, RunQueryDsl};
use std::fmt::Display;

use crate::result::ScamplersResult;

pub mod models;
pub mod util;

#[cfg(feature = "app")]
pub trait DbOperation<Output>: Sized {
    fn execute(self, db_conn: &mut PgConnection) -> ScamplersResult<Output>;

    fn execute_as_user<User>(
        self,
        user: User,
        db_conn: &mut PgConnection,
    ) -> ScamplersResult<Output>
    where
        User: Display,
    {
        db_conn.transaction(|tx| {
            diesel::sql_query(format!(r#"set local role "{user}""#)).execute(tx)?;

            self.execute(tx)
        })
    }
}
