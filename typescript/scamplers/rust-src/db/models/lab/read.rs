use diesel::prelude::*;
use scamplers_schema::{lab, lab_membership, person};
use uuid::Uuid;

use crate::{
    apply_eq_any_filters, apply_ilike_filters, attach_children_to_parents_mtm,
    db::{
        DbOperation,
        models::{
            lab::{Lab, LabId, LabOrderBy, LabQuery, LabSummaryWithRelations},
            person::PersonSummary,
        },
    },
    impl_id_db_operation, init_stmt,
};

impl DbOperation<Vec<Lab>> for LabQuery {
    fn execute(self, db_conn: &mut PgConnection) -> crate::result::ScamplersResult<Vec<Lab>> {
        #[derive(Identifiable, Associations, Selectable, Queryable)]
        #[diesel(table_name = lab_membership, belongs_to(LabSummaryWithRelations, foreign_key = lab_id), belongs_to(PersonSummary, foreign_key = member_id), primary_key(lab_id, member_id), check_for_backend(diesel::pg::Pg))]
        struct LabMembership {
            lab_id: Uuid,
            member_id: Uuid,
        }

        let mut stmt = init_stmt!(stmt = lab::table.inner_join(person::table), query = &self, output_type = LabSummaryWithRelations, orderby_spec = { LabOrderBy::Name => lab::name });

        let Self { ids, name, .. } = &self;

        stmt = apply_eq_any_filters!(
            stmt,
            filters = {lab::id => ids}
        );

        stmt = apply_ilike_filters!(
            stmt,
            filters = {lab::name => name}
        );

        let labs: Vec<LabSummaryWithRelations> = stmt.load(db_conn)?;
        let members = LabMembership::belonging_to(&labs)
            .inner_join(person::table)
            .select((LabMembership::as_select(), PersonSummary::as_select()))
            .load(db_conn)?;

        Ok(attach_children_to_parents_mtm!(
            parents = labs,
            children = members,
            transform_fn = |(info, members)| Lab { info, members }
        ))
    }
}

impl_id_db_operation!(id_type = LabId, delegate_to = LabQuery, returns = Lab);
