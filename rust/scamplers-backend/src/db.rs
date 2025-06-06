pub mod error;
pub mod model;
pub mod seed_data;
mod util;

use diesel_async::AsyncPgConnection;
use util::BoxedDieselExpression;

use util::NewBoxedDieselExpression;
pub use util::set_transaction_user;

trait AsDieselFilter<QuerySource = ()> {
    fn as_diesel_filter<'a>(&'a self) -> Option<BoxedDieselExpression<'a, QuerySource>>
    where
        QuerySource: 'a;
}

impl AsDieselFilter for () {
    fn as_diesel_filter<'a>(&'a self) -> Option<BoxedDieselExpression<'a, ()>>
    where
        (): 'a,
    {
        BoxedDieselExpression::new_expression().build()
    }
}

impl<Query, QuerySource> AsDieselFilter<QuerySource> for Option<Query>
where
    Query: AsDieselFilter<QuerySource>,
{
    fn as_diesel_filter<'a>(&'a self) -> Option<BoxedDieselExpression<'a, QuerySource>>
    where
        QuerySource: 'a,
    {
        match self {
            Some(query) => query.as_diesel_filter(),
            None => BoxedDieselExpression::new_expression().build(),
        }
    }
}

trait AsDieselQueryBase {
    type QueryBase;

    fn as_diesel_query_base() -> Self::QueryBase;
}

pub trait Write {
    type Returns;

    fn write(
        self,
        db_conn: &mut AsyncPgConnection,
    ) -> impl Future<Output = error::Result<Self::Returns>> + Send;
}

pub trait FetchById: Sized {
    type Id;

    fn fetch_by_id(
        id: &Self::Id,
        db_conn: &mut AsyncPgConnection,
    ) -> impl Future<Output = error::Result<Self>> + Send;
}

pub trait FetchByQuery: Sized {
    type QueryParams;

    fn fetch_by_query(
        query: &Self::QueryParams,
        db_conn: &mut AsyncPgConnection,
    ) -> impl Future<Output = error::Result<Vec<Self>>> + Send;
}
