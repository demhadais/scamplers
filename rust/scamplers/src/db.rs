use diesel::{Connection, PgConnection, RunQueryDsl};
#[cfg(feature = "app")]
use diesel::{query_builder::SqlQuery, query_dsl::methods::ExecuteDsl};
use std::fmt::Display;

use crate::result::ScamplersResult;

pub mod models;
#[cfg(feature = "app")]
pub mod seed_data;
pub mod util;

#[cfg(feature = "app")]
pub trait DbOperation<Output>: Sized {
    fn execute(self, db_conn: &mut PgConnection) -> ScamplersResult<Output>;

    fn execute_as_user<UserId, Conn>(
        self,
        user_id: UserId,
        db_conn: &mut PgConnection,
    ) -> ScamplersResult<Output>
    where
        UserId: Display,
    {
        db_conn.transaction(|tx| {
            diesel::sql_query(format!(r#"set local role "{user_id}""#)).execute(tx)?;

            self.execute(tx)
        })
    }
}
