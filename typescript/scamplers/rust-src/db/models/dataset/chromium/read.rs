use diesel::prelude::*;
use scamplers_schema::{
    cdna, chip_loading, chromium_dataset, chromium_dataset_libraries, lab, library,
};

use crate::{
    apply_eq_any_filters, apply_ilike_filters, apply_tenx_assay_query, apply_time_filters,
    db::{
        DbOperation,
        models::{
            Pagination,
            dataset::chromium::{
                ChromiumDataset, ChromiumDatasetId, ChromiumDatasetOrderBy, ChromiumDatasetQuery,
            },
            nucleic_acid::common::gems_to_assay,
            suspension::suspension::SuspensionQuery,
        },
    },
    impl_id_db_operation, init_stmt,
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
                        .specimen(specimen_query)
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

        let base_stmt = chip_loading::table.inner_join(
            gems_to_assay().inner_join(
                cdna::table.inner_join(
                    library::table.inner_join(
                        chromium_dataset_libraries::table
                            .inner_join(chromium_dataset::table.inner_join(lab::table)),
                    ),
                ),
            ),
        );

        let mut stmt = init_stmt!(
            stmt = base_stmt,
            query = &self,
            output_type = ChromiumDataset,
            orderby_spec = {
                ChromiumDatasetOrderBy::DeliveredAt => chromium_dataset::delivered_at,
                ChromiumDatasetOrderBy::Name => chromium_dataset::name
            }
        );

        if let Some(tenx_assay_query) = self.tenx_assay.take() {
            stmt = apply_tenx_assay_query!(stmt, tenx_assay_query);
        }

        if let Some(suspension_and_suspension_pool_ids) = &suspension_and_suspension_pool_ids {
            stmt = stmt
                .filter(chip_loading::suspension_id.eq_any(suspension_and_suspension_pool_ids))
                .or_filter(
                    chip_loading::suspension_pool_id.eq_any(suspension_and_suspension_pool_ids),
                );
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

        Ok(stmt.load(db_conn)?)
    }
}

impl_id_db_operation!(
    id_type = ChromiumDatasetId,
    delegate_to = ChromiumDatasetQuery,
    returns = ChromiumDataset
);

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::db::{
        DbOperation,
        models::{
            dataset::chromium::{ChromiumDatasetOrderBy, ChromiumDatasetQuery},
            specimen::{
                BlockEmbeddingMatrix, SpecimenQuery, SpecimenType,
                block::FrozenBlockEmbeddingMatrix,
            },
        },
        test_util::db_conn,
    };

    #[rstest]
    #[awt]
    #[tokio::test]
    async fn specific_dataset_query(#[future] db_conn: deadpool_diesel::postgres::Connection) {
        let specimen_query = SpecimenQuery::builder()
            .frozen(true)
            .types([SpecimenType::Block])
            .embedded_in([BlockEmbeddingMatrix::Frozen(
                FrozenBlockEmbeddingMatrix::CarboxymethylCellulose,
            )])
            .build();

        let query = ChromiumDatasetQuery::builder()
            .specimen(specimen_query)
            .order_by(ChromiumDatasetOrderBy::DeliveredAt { descending: false })
            .build();

        let _ = db_conn
            .interact(|db_conn| query.execute(db_conn).unwrap())
            .await
            .unwrap();
    }
}
