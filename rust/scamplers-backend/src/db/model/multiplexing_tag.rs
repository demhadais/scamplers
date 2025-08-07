use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use scamplers_core::model::suspension::{MultiplexingTag, NewMultiplexingTag};
use scamplers_schema::multiplexing_tag;

use crate::db::model::{FetchByQuery, WriteToDbInternal};

impl WriteToDbInternal for Vec<NewMultiplexingTag> {
    type Returns = MultiplexingTag;

    async fn write_to_db(
        self,
        db_conn: &mut diesel_async::AsyncPgConnection,
    ) -> crate::result::ScamplersResult<Self::Returns> {
        Ok(diesel::insert_into(multiplexing_tag::table)
            .values(self)
            .on_conflict_do_nothing()
            .returning(MultiplexingTag::as_returning())
            .get_result(db_conn)
            .await?)
    }
}

impl FetchByQuery for MultiplexingTag {
    type QueryParams = ();

    async fn fetch_by_query(
        _query: &Self::QueryParams,
        db_conn: &mut diesel_async::AsyncPgConnection,
    ) -> crate::result::ScamplersResult<Vec<Self>> {
        Ok(multiplexing_tag::table
            .select(Self::as_select())
            .order_by(multiplexing_tag::tag_id)
            .load(db_conn)
            .await?)
    }
}
