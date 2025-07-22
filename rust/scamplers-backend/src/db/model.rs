use diesel_async::AsyncPgConnection;
use uuid::Uuid;

use super::error;
use crate::db::util::{BoxedDieselExpression, NewBoxedDieselExpression};

pub mod cdna;
pub mod chromium_run;
pub mod dataset;
pub mod institution;
pub mod lab;
pub mod library;
pub mod person;
pub mod sequencing_run;
pub mod specimen;
pub mod suspension;
pub mod suspension_pool;

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

pub trait WriteToDb {
    type Returns;

    fn write_to_db(
        self,
        db_conn: &mut AsyncPgConnection,
    ) -> impl Future<Output = error::Result<Self::Returns>> + Send;
}

trait WriteToDbInternal {
    type Returns;

    async fn write(self, db_conn: &mut AsyncPgConnection) -> error::Result<Self::Returns>;
}

trait IsUpdate<const N: usize> {
    fn fields_are_some(&self) -> [bool; N];
    fn is_update(&self) -> bool {
        self.fields_are_some().iter().any(|b| *b)
    }
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

pub trait FetchRelatives<R>: diesel::Table {
    type Id;

    fn fetch_relatives(
        id: &Self::Id,
        db_conn: &mut AsyncPgConnection,
    ) -> impl Future<Output = error::Result<Vec<R>>> + Send;
}

#[macro_export]
macro_rules! fetch_by_query {
    ($query:ident, [$(($ordinal_col_enum_variant:ident, $corresponding_db_col:ident)),*], $db_conn:ident) => {{
        use super::AsDieselFilter;

        let Self::QueryParams{order_by, pagination: scamplers_core::model::Pagination{limit, offset}, ..} = $query;

        let query = $query.as_diesel_filter();

        let mut statement = Self::as_diesel_query_base()
            .select(Self::as_select())
            .limit(*limit)
            .offset(*offset)
            .into_boxed();

        if let Some(query) = query {
            statement = statement.filter(query);
        }

        for ordering in order_by.as_slice() {
            statement = match (&ordering.by, ordering.descending) {
                $(
                    ($ordinal_col_enum_variant, false) => statement.then_order_by($corresponding_db_col.asc()),
                    ($ordinal_col_enum_variant, true) => statement.then_order_by($corresponding_db_col.desc()),
                )*
            };
        }

        Ok(statement.load($db_conn).await?)
    }};
}

trait SetParentId {
    fn parent_id_mut(&mut self) -> &mut Uuid;

    fn set_parent_id(&mut self, id: Uuid) {
        let entity_id = self.parent_id_mut();
        *entity_id = id;
    }
}

trait Mappping {
    fn new(parent_id: Uuid, child_id: Uuid) -> Self;
}

trait HasPreparers {
    type Preparers: Mappping;

    fn children(&self) -> &[Uuid];

    fn preparers(&self, self_id: Uuid) -> Vec<Self::Preparers> {
        self.children()
            .iter()
            .map(|child_id| Self::Preparers::new(self_id, *child_id))
            .collect()
    }
}

trait HasMeasurements {
    type Measurement: SetParentId;

    fn measurements(&mut self) -> &mut [Self::Measurement];

    fn measurements_with_self_id(&mut self, self_id: Uuid) -> &[Self::Measurement] {
        let measurements = self.measurements();

        for m in &mut *measurements {
            m.set_parent_id(self_id);
        }

        measurements
    }
}
