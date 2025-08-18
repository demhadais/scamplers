use diesel::prelude::*;
use scamplers_schema::multiplexing_tag;

use crate::db::{DbOperation, models::multiplexing_tag::MultiplexingTag};

impl DbOperation<Vec<MultiplexingTag>> for () {
    fn execute(
        self,
        db_conn: &mut diesel::PgConnection,
    ) -> crate::result::ScamplersResult<Vec<MultiplexingTag>> {
        Ok(multiplexing_tag::table
            .select(MultiplexingTag::as_select())
            .order_by(multiplexing_tag::tag_id)
            .load(db_conn)?)
    }
}
