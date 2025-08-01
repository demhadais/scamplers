use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use scamplers_core::model::institution::{Institution, InstitutionQuery, NewInstitution};
use scamplers_schema::institution::dsl::{id as id_col, institution, name as name_col};
use uuid::Uuid;

use crate::{
    db::{
        model::{self, AsDieselQueryBase},
        util::{AsIlike, BoxedDieselExpression, NewBoxedDieselExpression},
    },
    fetch_by_query,
    result::ScamplersResult,
};

impl model::WriteToDb for NewInstitution {
    type Returns = Institution;

    async fn write_to_db(
        self,
        db_conn: &mut diesel_async::AsyncPgConnection,
    ) -> ScamplersResult<Self::Returns> {
        let inserted = diesel::insert_into(institution)
            .values(self)
            .returning(Institution::as_returning())
            .get_result(db_conn)
            .await?;

        Ok(inserted)
    }
}

impl model::AsDieselQueryBase for Institution {
    type QueryBase = institution;

    fn as_diesel_query_base() -> Self::QueryBase {
        institution
    }
}

impl model::FetchById for Institution {
    type Id = Uuid;

    async fn fetch_by_id(
        id: &Self::Id,
        db_conn: &mut diesel_async::AsyncPgConnection,
    ) -> ScamplersResult<Self> {
        let query_base = Self::as_diesel_query_base();
        Ok(query_base
            .find(id)
            .select(Institution::as_select())
            .first(db_conn)
            .await?)
    }
}

impl<QuerySource> model::AsDieselFilter<QuerySource> for InstitutionQuery
where
    id_col: SelectableExpression<QuerySource>,
    name_col: SelectableExpression<QuerySource>,
{
    fn as_diesel_filter<'a>(&'a self) -> Option<BoxedDieselExpression<'a, QuerySource>>
    where
        QuerySource: 'a,
    {
        let Self { ids, name, .. } = self;
        let q1 = (!ids.is_empty()).then(|| id_col.eq_any(ids));
        let q2 = name.as_ref().map(|name| name_col.ilike(name.as_ilike()));

        BoxedDieselExpression::new_expression()
            .and_condition(q1)
            .and_condition(q2)
            .build()
    }
}

impl model::FetchByQuery for Institution {
    type QueryParams = InstitutionQuery;

    async fn fetch_by_query(
        query: &Self::QueryParams,
        db_conn: &mut diesel_async::AsyncPgConnection,
    ) -> ScamplersResult<Vec<Self>> {
        use scamplers_core::model::institution::InstitutionOrdinalColumn::Name;

        fetch_by_query!(query, [(Name, name_col)], db_conn)
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use rstest::rstest;
    use scamplers_core::model::institution::*;

    use crate::db::test_util::{DbConnection, db_conn, institutions, test_query};

    fn sort_by_name(i1: &Institution, i2: &Institution) -> Ordering {
        i1.name.cmp(&i2.name)
    }

    #[rstest]
    #[awt]
    #[tokio::test]
    async fn default_institution_query(
        #[future] db_conn: DbConnection,
        #[future] institutions: Vec<Institution>,
    ) {
        test_query()
            .all_data(institutions)
            .extract(|i| i)
            .sort_by(sort_by_name)
            .run(db_conn)
            .await;
    }

    #[rstest]
    #[awt]
    #[tokio::test]
    async fn specific_institution_query(
        #[future] db_conn: DbConnection,
        #[future] institutions: Vec<Institution>,
    ) {
        let query = InstitutionQuery::builder()
            .name("institution1")
            .order_by((InstitutionOrdinalColumn::Name, true))
            .build();

        test_query()
            .all_data(institutions)
            .extract(|i| i)
            .filter(|i| i.name.starts_with("institution1"))
            .sort_by(|i1, i2| sort_by_name(i1, i2).reverse())
            .db_query(query)
            .run(db_conn)
            .await;
    }
}
