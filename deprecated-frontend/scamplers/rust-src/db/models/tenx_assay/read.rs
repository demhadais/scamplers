use diesel::prelude::*;
use scamplers_schema::{library_type_specification, tenx_assay};
use uuid::Uuid;

use crate::{
    db::{
        DbOperation,
        models::tenx_assay::{
            TenxAssay, TenxAssayId, TenxAssayOrderBy, TenxAssayQuery,
            chromium::LibraryTypeSpecification,
        },
    },
    impl_id_db_operation, init_stmt,
};

#[macro_export]
macro_rules! apply_tenx_assay_query {
    ($stmt:expr, $query:expr) => {{
        $stmt = $crate::apply_eq_any_filters!(
            $stmt,
            filters = {
                scamplers_schema::tenx_assay::id => $query.ids,
                scamplers_schema::tenx_assay::sample_multiplexing => $query.sample_multiplexing,
                scamplers_schema::tenx_assay::chromium_chip => $query.chromium_chips
            }
        );

        $stmt = $crate::apply_ilike_filters!(
            $stmt,
            filters = {
                scamplers_schema::tenx_assay::name => &$query.names,
                scamplers_schema::tenx_assay::chemistry_version => &$query.chemistry_versions
            }
        );

        let mut lib_types_condition: Option<Box<dyn diesel::BoxableExpression<_, diesel::pg::Pg, SqlType = diesel::sql_types::Nullable<diesel::sql_types::Bool>>>> = None;

        for mut lib_type_group in $query.library_types {
            lib_type_group.sort();

            if let Some(condition) = lib_types_condition {
                lib_types_condition = Some(Box::new(condition.or(scamplers_schema::tenx_assay::library_types.eq(lib_type_group))));
            } else {
                lib_types_condition = Some(Box::new(scamplers_schema::tenx_assay::library_types.eq(lib_type_group)));
            }
        }

        if let Some(condition) = lib_types_condition {
            $stmt = $stmt.filter(condition);
        }

        $stmt
    }};
}

impl DbOperation<Vec<TenxAssay>> for TenxAssayQuery {
    fn execute(
        self,
        db_conn: &mut diesel::PgConnection,
    ) -> crate::result::ScamplersResult<Vec<TenxAssay>> {
        let mut stmt = init_stmt!(
            TenxAssay,
            query = self,
            orderby_spec = {
                TenxAssayOrderBy::Name => tenx_assay::name,
                TenxAssayOrderBy::LibraryTypes => tenx_assay::library_types
            }
        );

        stmt = apply_tenx_assay_query!(stmt, self);

        Ok(stmt.load(db_conn)?)
    }
}

impl_id_db_operation!(
    id_type = TenxAssayId,
    delegate_to = TenxAssayQuery,
    returns = TenxAssay
);

impl LibraryTypeSpecification {
    // This is a temporary function which will eventually be moved into the
    // `TenxAssayQuery`
    pub fn list_by_assay_id(
        assay_id: Uuid,
        db_conn: &mut PgConnection,
    ) -> crate::result::ScamplersResult<Vec<Self>> {
        Ok(library_type_specification::table
            .filter(library_type_specification::assay_id.eq(assay_id))
            .order_by(library_type_specification::library_type)
            .select(Self::as_select())
            .load(db_conn)?)
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use rstest::rstest;

    use crate::db::{
        models::tenx_assay::{
            TenxAssay, TenxAssayOrderBy, TenxAssayQuery,
            chromium::{LibraryType, SampleMultiplexing},
        },
        test_util::{db_conn, tenx_assays, test_query},
    };

    fn sort(a1: &TenxAssay, a2: &TenxAssay) -> Ordering {
        a1.name
            .cmp(&a2.name)
            .then(a1.library_types.cmp(&a2.library_types))
    }

    #[rstest]
    #[awt]
    #[tokio::test]
    async fn default_tenx_assay_query(
        #[future] db_conn: deadpool_diesel::postgres::Connection,
        #[future] tenx_assays: Vec<TenxAssay>,
    ) {
        test_query()
            .all_data(tenx_assays)
            .sort_by(sort)
            .db_query(
                TenxAssayQuery::builder()
                    .order_by([
                        TenxAssayOrderBy::Name { descending: false },
                        TenxAssayOrderBy::LibraryTypes { descending: false },
                    ])
                    .build(),
            )
            .run(db_conn)
            .await;
    }

    #[rstest]
    #[awt]
    #[tokio::test]
    async fn specific_tenx_assay_query(
        #[future] db_conn: deadpool_diesel::postgres::Connection,
        #[future] tenx_assays: Vec<TenxAssay>,
    ) {
        macro_rules! desired_lib_types {
            () => {
                [
                    vec![LibraryType::GeneExpression],
                    vec![LibraryType::AntibodyCapture, LibraryType::GeneExpression],
                ]
            };
        }

        let query = TenxAssayQuery::builder()
            .library_types(desired_lib_types!())
            .sample_multiplexing([SampleMultiplexing::OnChipMultiplexing])
            .order_by([
                TenxAssayOrderBy::Name { descending: true },
                TenxAssayOrderBy::LibraryTypes { descending: true },
            ])
            .build();

        test_query()
            .all_data(tenx_assays)
            .filter(|a| {
                a.library_types.as_ref().is_some_and(|types| {
                    let types = types.iter().cloned().filter_map(|t| t).collect();
                    desired_lib_types!().contains(&types)
                }) && a.sample_multiplexing == Some(SampleMultiplexing::OnChipMultiplexing)
            })
            .sort_by(|a1, a2| sort(a1, a2).reverse())
            .db_query(query)
            .run(db_conn)
            .await;
    }
}
