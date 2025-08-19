use diesel::prelude::*;
use scamplers_schema::{lab, person, specimen};

use crate::{
    apply_eq_any_filters, apply_eq_filters, apply_ilike_filters, attach_children_to_parents,
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
            name,
            submitters,
            labs,
            received_before,
            received_after,
            species,
            notes,
            types,
            embedded_in,
            fixatives,
            storage_buffer,
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
                specimen::name => name,
                specimen::notes => notes,
                specimen::storage_buffer => storage_buffer
            }
        );

        stmt = apply_eq_filters!(
            stmt,
            filters = {
                specimen::frozen => frozen,
                specimen::cryopreserved => cryopreserved
            }
        );

        if let Some(received_before) = received_before {
            stmt = stmt.filter(specimen::received_at.lt(received_before));
        }

        if let Some(received_after) = received_after {
            stmt = stmt.filter(specimen::received_at.gt(received_after));
        }

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
