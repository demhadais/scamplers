#[cfg(feature = "app")]
use std::fmt::Display;

#[cfg(feature = "app")]
use diesel::{PgConnection, prelude::*};

#[cfg(feature = "app")]
use crate::result::ScamplersResult;

pub mod models;
#[cfg(feature = "app")]
pub mod seed_data;
#[cfg(all(feature = "app", test))]
pub mod test_util;
pub mod util;
mod validators;

#[cfg(feature = "app")]
pub trait DbOperation<Output>: Sized {
    /// # Errors
    fn execute(self, db_conn: &mut PgConnection) -> ScamplersResult<Output>;

    /// # Errors
    fn execute_as_user<UserId>(
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
