use diesel::prelude::*;
use scamplers_schema::specimen;

use crate::{
    apply_eq_any_filters, attach_children_to_parents,
    db::{
        DbOperation,
        models::specimen::{
            Specimen, SpecimenId, SpecimenMeasurement, SpecimenOrderBy, SpecimenQuery,
            SpecimenSummaryWithParents,
        },
    },
    group_children, impl_id_db_operation, init_stmt,
};

#[macro_export]
macro_rules! apply_specimen_query {
    ($stmt:expr, $query:expr) => {{
        use scamplers_schema::specimen;

        $stmt = apply_eq_any_filters!(
            $stmt,
            filters = {
                specimen::id => $query.ids,
                specimen::type_ => $query.types,
                specimen::submitted_by => $query.submitters,
                specimen::lab_id => $query.labs,
                specimen::embedded_in => $query.embedded_in,
                specimen::fixative => $query.fixatives
            }
        );

        $stmt = $crate::apply_ilike_filters!(
            $stmt,
            filters = {
                specimen::name => &$query.names,
                specimen::notes => &$query.notes,
                specimen::storage_buffer => &$query.storage_buffers
            }
        );

        $stmt = $crate::apply_eq_filters!(
            $stmt,
            filters = {
                specimen::frozen => $query.frozen,
                specimen::cryopreserved => $query.cryopreserved
            }
        );

        $stmt = $crate::apply_time_filters!(
            $stmt,
            filters = {
                specimen::received_at => ($query.received_before, $query.received_after)
            }
        );

        if !$query.species.is_empty() {
            $stmt = $stmt.filter(specimen::species.overlaps_with($query.species));
        }

        $stmt
    }};
}

impl DbOperation<Vec<Specimen>> for SpecimenQuery {
    fn execute(self, db_conn: &mut PgConnection) -> crate::result::ScamplersResult<Vec<Specimen>> {
        let mut stmt = init_stmt!(SpecimenSummaryWithParents, query = &self, orderby_spec = { SpecimenOrderBy::Name => specimen::name, SpecimenOrderBy::ReadableId => specimen::readable_id, SpecimenOrderBy::ReceivedAt => specimen::received_at });

        stmt = apply_specimen_query!(stmt, self);

        let specimen_records = stmt.load(db_conn)?;

        let measurements = SpecimenMeasurement::belonging_to(&specimen_records)
            .select(SpecimenMeasurement::as_select())
            .load(db_conn)?;

        let grouped_measurements =
            group_children!(parents = specimen_records, children = measurements);

        let specimens = attach_children_to_parents!(
            parents = specimen_records,
            children = [grouped_measurements],
            transform_fn = |(info, measurements)| Specimen { info, measurements }
        );

        Ok(specimens)
    }
}

impl_id_db_operation!(
    id_type = SpecimenId,
    delegate_to = SpecimenQuery,
    returns = Specimen
);

#[cfg(test)]
mod tests {
    #![allow(clippy::cast_possible_wrap)]
    use rstest::rstest;

    use crate::db::{
        models::{
            Pagination,
            specimen::{
                BlockEmbeddingMatrix, Specimen, SpecimenOrderBy, SpecimenQuery, SpecimenType,
                block::FrozenBlockEmbeddingMatrix,
            },
        },
        test_util::{N_SPECIMENS, db_conn, specimens, test_query},
    };

    #[rstest]
    #[awt]
    #[tokio::test]
    async fn specimen_measurements(#[future] specimens: Vec<Specimen>) {
        for s in specimens {
            assert_eq!(s.measurements.len(), 1);
        }
    }

    #[rstest]
    #[awt]
    #[tokio::test]
    async fn default_specimen_query(
        #[future] db_conn: deadpool_diesel::postgres::Connection,
        #[future] specimens: Vec<Specimen>,
    ) {
        let query = SpecimenQuery::builder()
            .pagination(Pagination {
                limit: N_SPECIMENS as i64,
                offset: 0,
            })
            .build();

        test_query()
            .all_data(specimens)
            .sort_by(|s1, s2| {
                s1.info
                    .summary
                    .received_at
                    .cmp(&s2.info.summary.received_at)
            })
            .db_query(query)
            .run(db_conn)
            .await;
    }

    #[rstest]
    #[awt]
    #[tokio::test]
    async fn specific_specimen_query(
        #[future] db_conn: deadpool_diesel::postgres::Connection,
        #[future] specimens: Vec<Specimen>,
    ) {
        fn filter(s: &Specimen) -> bool {
            s.info.summary.frozen
                && s.info.summary.type_ == SpecimenType::Block
                && s.info
                    .summary
                    .embedded_in
                    .as_ref()
                    .is_some_and(|e| e == "carboxymethyl_cellulose")
        }

        let query = SpecimenQuery::builder()
            .frozen(true)
            .types([SpecimenType::Block])
            .embedded_in([BlockEmbeddingMatrix::Frozen(
                FrozenBlockEmbeddingMatrix::CarboxymethylCellulose,
            )])
            .order_by(SpecimenOrderBy::Name { descending: true })
            .pagination(Pagination {
                limit: N_SPECIMENS as i64,
                offset: 0,
            })
            .build();

        test_query()
            .all_data(specimens)
            .filter(filter)
            .sort_by(|s1, s2| s1.info.summary.name.cmp(&s2.info.summary.name).reverse())
            .db_query(query)
            .run(db_conn)
            .await;
    }
}
