use diesel::prelude::*;
use scamplers_schema::{lab, person, specimen};

use crate::{
    apply_eq_any_filters, apply_eq_filters, apply_ilike_filters, apply_time_filters,
    attach_children_to_parents,
    db::{
        DbOperation,
        models::specimen::{
            Specimen, SpecimenId, SpecimenMeasurement, SpecimenOrderBy, SpecimenQuery,
            SpecimenSummaryWithParents,
        },
    },
    group_otm_children, impl_id_db_operation, init_stmt,
};

impl DbOperation<Vec<Specimen>> for SpecimenQuery {
    fn execute(self, db_conn: &mut PgConnection) -> crate::result::ScamplersResult<Vec<Specimen>> {
        let submitter_join_condition = specimen::submitted_by.eq(person::id);

        let base_stmt = specimen::table
            .inner_join(lab::table)
            .inner_join(person::table.on(submitter_join_condition));

        let mut stmt = init_stmt!(stmt = base_stmt, query = &self, output_type = SpecimenSummaryWithParents, orderby_spec = {SpecimenOrderBy::Name => specimen::name, SpecimenOrderBy::ReceivedAt => specimen::received_at});

        let Self {
            ids,
            names,
            submitters,
            labs,
            received_before,
            received_after,
            species,
            notes,
            types,
            embedded_in,
            fixatives,
            storage_buffers,
            frozen,
            cryopreserved,
            ..
        } = &self;

        stmt = apply_eq_any_filters!(
            stmt,
            filters = {
                specimen::id => ids,
                specimen::type_ => types,
                specimen::submitted_by => submitters,
                specimen::lab_id => labs,
                specimen::embedded_in => embedded_in,
                specimen::fixative => fixatives
            }
        );

        stmt = apply_ilike_filters!(stmt,
            filters = {
                specimen::name => names,
                specimen::notes => notes,
                specimen::storage_buffer => storage_buffers
            }
        );

        stmt = apply_eq_filters!(
            stmt,
            filters = {
                specimen::frozen => frozen,
                specimen::cryopreserved => cryopreserved
            }
        );

        stmt = apply_time_filters!(
            stmt,
            filters = {
                specimen::received_at => (received_before, received_after)
            }
        );

        if !species.is_empty() {
            stmt = stmt.filter(specimen::species.overlaps_with(species));
        }

        let specimen_records = stmt.load(db_conn)?;

        let measurements = SpecimenMeasurement::belonging_to(&specimen_records)
            .select(SpecimenMeasurement::as_select())
            .load(db_conn)?;

        let grouped_measurements =
            group_otm_children!(parents = specimen_records, children = measurements);

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
