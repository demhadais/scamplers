use diesel::{BelongingToDsl, prelude::*};
use scamplers_schema::suspension_pool;

use crate::{
    apply_eq_any_filters, attach_children_to_parents,
    db::{
        DbOperation,
        models::suspension::{
            pool::{
                SuspensionPool, SuspensionPoolId, SuspensionPoolMeasurement, SuspensionPoolOrderBy,
                SuspensionPoolPreparer, SuspensionPoolQuery, SuspensionPoolSummary,
            },
            suspension::SuspensionSummary,
        },
    },
    group_otm_children, impl_id_db_operation, init_stmt,
};

impl DbOperation<Vec<SuspensionPool>> for SuspensionPoolQuery {
    fn execute(
        self,
        db_conn: &mut diesel::PgConnection,
    ) -> crate::result::ScamplersResult<Vec<SuspensionPool>> {
        let base_stmt = suspension_pool::table;

        let mut stmt = init_stmt!(stmt = base_stmt, query = &self, output_type = SuspensionPoolSummary, orderby_spec = {SuspensionPoolOrderBy::Name => suspension_pool::name, SuspensionPoolOrderBy::PooledAt => suspension_pool::pooled_at});

        let Self { ids, .. } = &self;

        stmt = apply_eq_any_filters!(stmt, filters = {suspension_pool::id => ids});
        let suspension_pool_summaries = stmt.load(db_conn)?;

        let measurements =
            SuspensionPoolMeasurement::belonging_to(&suspension_pool_summaries).load(db_conn)?;

        let grouped_measurements =
            group_otm_children!(parents = suspension_pool_summaries, children = measurements);

        let preparers = SuspensionPoolPreparer::belonging_to(&suspension_pool_summaries)
            .select(SuspensionPoolPreparer::as_select())
            .load(db_conn)?;

        let grouped_preparers =
            group_otm_children!(parents = suspension_pool_summaries, children = preparers)
                .map(|v| v.into_iter().map(|p| p.prepared_by).collect());

        let suspensions = SuspensionSummary::belonging_to(&suspension_pool_summaries)
            .select(SuspensionSummary::as_select())
            .load(db_conn)?;

        let grouped_suspensions =
            group_otm_children!(parents = suspension_pool_summaries, children = suspensions);

        let suspension_pools = attach_children_to_parents!(
            parents = suspension_pool_summaries,
            children = [grouped_suspensions, grouped_measurements, grouped_preparers],
            transform_fn = |(((summary, suspensions), measurements), preparers)| SuspensionPool {
                summary,
                suspensions,
                preparers,
                measurements,
            }
        );

        Ok(suspension_pools)
    }
}

impl_id_db_operation! {id_type = SuspensionPoolId, delegate_to = SuspensionPoolQuery, returns = SuspensionPool}

#[cfg(test)]
mod tests {
    #![allow(clippy::cast_possible_wrap)]
    use rstest::rstest;

    use crate::db::{
        models::{
            Pagination,
            suspension::pool::{SuspensionPool, SuspensionPoolQuery},
        },
        test_util::{N_SUSPENSION_POOLS, db_conn, suspension_pools, test_query},
    };

    #[rstest]
    #[awt]
    #[tokio::test]
    async fn suspension_pool_measurements(#[future] suspension_pools: Vec<SuspensionPool>) {
        for s in suspension_pools {
            assert_eq!(s.measurements.len(), 1);
        }
    }

    #[rstest]
    #[awt]
    #[tokio::test]
    async fn default_suspension_pool_query(
        #[future] db_conn: deadpool_diesel::postgres::Connection,
        #[future] suspension_pools: Vec<SuspensionPool>,
    ) {
        let query = SuspensionPoolQuery::builder()
            .pagination(Pagination {
                limit: N_SUSPENSION_POOLS as i64,
                offset: 0,
            })
            .build();

        test_query()
            .all_data(suspension_pools)
            .sort_by(|s1, s2| s1.summary.pooled_at.cmp(&s2.summary.pooled_at))
            .db_query(query)
            .run(db_conn)
            .await;
    }
}
