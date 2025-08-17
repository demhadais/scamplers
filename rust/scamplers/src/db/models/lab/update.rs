use diesel::PgConnection;
use diesel::prelude::*;
use scamplers_schema::lab;
use scamplers_schema::lab_membership;

use crate::db::models::lab::LabId;
use crate::{
    db::{
        DbOperation,
        models::lab::{Lab, LabUpdate},
    },
    result::ScamplersResult,
};

impl DbOperation<Lab> for LabUpdate {
    fn execute(self, db_conn: &mut PgConnection) -> ScamplersResult<Lab> {
        let Self {
            core,
            add_members,
            remove_members,
        } = self;

        if core.is_update() {
            diesel::update(&core)
                .set(&core)
                .returning((lab::pi_id))
                .execute(db_conn)?;
        }

        let member_additions: Vec<_> = add_members
            .iter()
            .map(|m_id| {
                (
                    lab_membership::lab_id.eq(core.id),
                    lab_membership::member_id.eq(m_id),
                )
            })
            .collect();

        diesel::insert_into(lab_membership::table)
            .values(member_additions)
            .on_conflict_do_nothing()
            .execute(db_conn)?;

        if remove_members.is_empty() {
            return LabId(core.id).execute(db_conn);
        }

        let mut deletion = diesel::delete(lab_membership::table).into_boxed();
        for member_id in &remove_members {
            deletion = deletion.or_filter(
                lab_membership::lab_id
                    .eq(&core.id)
                    .and(lab_membership::member_id.eq(member_id)),
            );
        }

        deletion.execute(db_conn)?;

        LabId(core.id).execute(db_conn)
    }
}
