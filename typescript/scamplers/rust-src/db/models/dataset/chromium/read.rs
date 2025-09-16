use diesel::prelude::*;
use scamplers_schema::{chip_loading, chromium_dataset};

use crate::{
    apply_eq_any_filters, apply_ilike_filters, apply_tenx_assay_query, apply_time_filters,
    attach_children_to_parents,
    db::{
        DbOperation,
        models::{
            Pagination,
            dataset::chromium::{
                ChromiumDataset, ChromiumDatasetId, ChromiumDatasetLibrary, ChromiumDatasetOrderBy,
                ChromiumDatasetQuery, ChromiumDatasetSummary,
            },
            suspension::suspension::SuspensionQuery,
        },
    },
    group_children, impl_id_db_operation, init_stmt,
};

impl DbOperation<Vec<ChromiumDataset>> for ChromiumDatasetQuery {
    fn execute(
        mut self,
        db_conn: &mut diesel::PgConnection,
    ) -> crate::result::ScamplersResult<Vec<ChromiumDataset>> {
        // First, get a list of suspensions whose parent specimens match the current
        // query. Reusing `SuspensionQuery` here may hinder performance because the
        // `SuspensionQuery` itself performs 3 trips to the db, but it's convenient to
        // reuse that logic here
        let suspension_and_suspension_pool_ids: Option<Vec<_>> =
            if let Some(specimen_query) = self.specimen.take() {
                // Extract either the suspension pool ID or the suspension ID for use in a later
                // query
                Some(
                    SuspensionQuery::builder()
                        .parent_specimen(specimen_query)
                        .pagination(Pagination {
                            limit: i64::MAX,
                            offset: 0,
                        })
                        .build()
                        .execute(db_conn)?
                        .iter()
                        .map(|s| s.info.summary.pooled_into.unwrap_or(s.info.id_))
                        .collect(),
                )
            } else {
                None
            };

        let mut stmt = init_stmt!(
            ChromiumDatasetSummary,
            query = &self,
            orderby_spec = {
                ChromiumDatasetOrderBy::DeliveredAt => chromium_dataset::delivered_at,
                ChromiumDatasetOrderBy::Name => chromium_dataset::name
            }
        );

        if let Some(tenx_assay_query) = self.tenx_assay.take() {
            stmt = apply_tenx_assay_query!(stmt, tenx_assay_query);
        }

        if let Some(suspension_and_suspension_pool_ids) =
            suspension_and_suspension_pool_ids.as_ref()
        {
            let condition = chip_loading::suspension_id
                .eq_any(suspension_and_suspension_pool_ids)
                .or(chip_loading::suspension_pool_id.eq_any(suspension_and_suspension_pool_ids));
            stmt = stmt.filter(condition);
        }

        let Self {
            ids,
            names,
            lab_ids,
            delivered_before,
            delivered_after,
            ..
        } = &self;

        stmt = apply_eq_any_filters!(
            stmt,
            filters = {
                chromium_dataset::id => ids,
                chromium_dataset::lab_id => lab_ids
            }
        );

        stmt = apply_ilike_filters!(
            stmt,
            filters = {
                chromium_dataset::name => names
            }
        );

        stmt = apply_time_filters!(
            stmt,
            filters = {
                chromium_dataset::delivered_at => (delivered_before, delivered_after)
            }
        );

        let dataset_summaries = stmt.load(db_conn)?;

        let library_ids = ChromiumDatasetLibrary::belonging_to(&dataset_summaries)
            .select(ChromiumDatasetLibrary::as_select())
            .load(db_conn)?;

        let grouped_library_ids =
            group_children!(parents = dataset_summaries, children = library_ids)
                .map(|v| v.iter().map(|mapping| mapping.library_id).collect());

        let datasets = attach_children_to_parents!(
            parents = dataset_summaries,
            children = [grouped_library_ids],
            transform_fn = |(summary, library_ids)| ChromiumDataset {
                summary,
                library_ids
            }
        );

        Ok(datasets)
    }
}

impl_id_db_operation!(
    id_type = ChromiumDatasetId,
    delegate_to = ChromiumDatasetQuery,
    returns = ChromiumDataset
);

#[cfg(test)]
mod tests {
    use std::i64;

    use diesel::prelude::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use scamplers_schema::{
        cdna, chip_loading, chromium_dataset, chromium_dataset_libraries, library,
    };
    use uuid::Uuid;

    use crate::db::{
        DbOperation,
        models::{
            Pagination,
            dataset::chromium::{ChromiumDatasetOrderBy, ChromiumDatasetQuery},
            specimen::{
                BlockEmbeddingMatrix, SpecimenQuery, SpecimenType,
                block::FrozenBlockEmbeddingMatrix,
            },
            suspension::suspension::SuspensionQuery,
        },
        test_util::db_conn,
    };

    #[rstest]
    #[awt]
    #[tokio::test]
    async fn dataset_query_by_specimens(#[future] db_conn: deadpool_diesel::postgres::Connection) {
        let specimen_query = SpecimenQuery::builder()
            .frozen(true)
            .types([SpecimenType::Block])
            .embedded_in([BlockEmbeddingMatrix::Frozen(
                FrozenBlockEmbeddingMatrix::CarboxymethylCellulose,
            )])
            .build();

        let query = ChromiumDatasetQuery::builder()
            .specimen(specimen_query.clone())
            .order_by(ChromiumDatasetOrderBy::DeliveredAt { descending: false })
            .build();

        let loaded_datasets = db_conn
            .interact(|db_conn| query.execute(db_conn).unwrap())
            .await
            .unwrap();
        let loaded_dataset_ids = loaded_datasets.iter().map(|ds| ds.summary.id);
        let loaded_gems_ids = chromium_dataset::table
            .inner_join(
                chromium_dataset_libraries::table
                    .inner_join(library::table.inner_join(cdna::table)),
            )
            .filter(chromium_dataset::id.eq_any(loaded_dataset_ids))
            .select(cdna::gems_id.assume_not_null());
        let mut loaded_gems_ids: Vec<Uuid> = db_conn
            .interact(move |db_conn| loaded_gems_ids.load(db_conn).unwrap())
            .await
            .unwrap();
        loaded_gems_ids.sort();

        let suspensions_matching_specimen_query = SuspensionQuery::builder()
            .parent_specimen(specimen_query)
            .pagination(Pagination {
                limit: i64::MAX,
                offset: 0,
            })
            .build();
        let suspensions_and_suspension_pools = db_conn
            .interact(|db_conn| {
                suspensions_matching_specimen_query
                    .execute(db_conn)
                    .unwrap()
            })
            .await
            .unwrap();
        let suspensions_and_suspension_pool_ids: Vec<_> = suspensions_and_suspension_pools
            .iter()
            .map(|s| s.info.summary.pooled_into.unwrap_or(s.info.id_))
            .collect();

        let condition = chip_loading::suspension_id
            .eq_any(suspensions_and_suspension_pool_ids.clone())
            .or(chip_loading::suspension_pool_id.eq_any(suspensions_and_suspension_pool_ids));
        let query = chip_loading::table
            .filter(condition)
            .select(chip_loading::gems_id)
            .distinct();
        let mut independently_loaded_gems_ids: Vec<Uuid> = db_conn
            .interact(move |db_conn| query.load(db_conn).unwrap())
            .await
            .unwrap();
        independently_loaded_gems_ids.sort();

        assert_eq!(loaded_gems_ids, independently_loaded_gems_ids);
    }
}
