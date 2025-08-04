use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use scamplers_core::model::suspension::{MultiplexingTag, NewMultiplexingTag};
use scamplers_schema::multiplexing_tag;

use crate::db::model::WriteToDb;

impl WriteToDb for NewMultiplexingTag {
    type Returns = MultiplexingTag;

    async fn write_to_db(
        self,
        db_conn: &mut diesel_async::AsyncPgConnection,
    ) -> crate::result::ScamplersResult<Self::Returns> {
        Ok(diesel::insert_into(multiplexing_tag::table)
            .values(self)
            .returning(MultiplexingTag::as_returning())
            .get_result(db_conn)
            .await?)
    }
}
