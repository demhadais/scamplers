use diesel::{PgTextExpressionMethods, RunQueryDsl, prelude::*};
use scamplers_schema::institution;

use super::{Institution, InstitutionOrderBy, InstitutionQuery};
use crate::{
    apply_eq_any_filters, apply_ilike_filters,
    db::{DbOperation, models::institution::InstitutionId},
    impl_id_db_operation, init_stmt,
    result::ScamplersResult,
};

impl DbOperation<Vec<Institution>> for InstitutionQuery {
    fn execute(self, db_conn: &mut diesel::PgConnection) -> ScamplersResult<Vec<Institution>> {
        let mut stmt = init_stmt!(Institution, query = &self, orderby_spec = { InstitutionOrderBy::Name => institution::name });

        let Self { ids, names, .. } = &self;

        stmt = apply_eq_any_filters!(stmt, filters = { institution::id => ids });
        stmt = apply_ilike_filters!(stmt, filters = { institution::name => names });

        Ok(stmt.load(db_conn)?)
    }
}

impl_id_db_operation!(
    id_type = InstitutionId,
    delegate_to = InstitutionQuery,
    returns = Institution
);

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use deadpool_diesel::postgres::Connection;
    use rstest::rstest;

    use crate::db::{
        models::institution::{Institution, InstitutionOrderBy, InstitutionQuery},
        test_util::{db_conn, institutions, test_query},
    };

    fn sort_by_name(i1: &Institution, i2: &Institution) -> Ordering {
        i1.name.cmp(&i2.name)
    }

    #[rstest]
    #[awt]
    #[tokio::test]
    async fn default_institution_query(
        #[future] db_conn: Connection,
        #[future] institutions: Vec<Institution>,
    ) {
        test_query::<InstitutionQuery, _>()
            .all_data(institutions)
            .sort_by(sort_by_name)
            .run(db_conn)
            .await;
    }

    #[rstest]
    #[awt]
    #[tokio::test]
    async fn specific_institution_query(
        #[future] db_conn: Connection,
        #[future] institutions: Vec<Institution>,
    ) {
        let query = InstitutionQuery::builder()
            .names(["institution1".to_string()])
            .order_by(InstitutionOrderBy::Name { descending: true })
            .build();

        test_query()
            .all_data(institutions)
            .filter(|i| i.name.starts_with("institution1"))
            .sort_by(|i1, i2| sort_by_name(i1, i2).reverse())
            .db_query(query)
            .run(db_conn)
            .await;
    }
}
