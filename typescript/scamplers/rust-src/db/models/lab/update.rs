use diesel::{PgConnection, prelude::*};
use scamplers_schema::lab_membership;

use crate::{
    db::{
        DbOperation,
        models::lab::{Lab, LabId, LabUpdate},
    },
    result::ScamplersResult,
};

impl DbOperation<Lab> for LabUpdate {
    fn execute(self, db_conn: &mut PgConnection) -> ScamplersResult<Lab> {
        diesel::update(&self)
            .set(&self)
            .execute(db_conn)
            .optional_empty_changeset()?;

        let Self {
            add_members,
            remove_members,
            ..
        } = self;

        let member_additions: Vec<_> = add_members
            .iter()
            .map(|m_id| {
                (
                    lab_membership::lab_id.eq(self.id),
                    lab_membership::member_id.eq(m_id),
                )
            })
            .collect();

        diesel::insert_into(lab_membership::table)
            .values(member_additions)
            .on_conflict_do_nothing()
            .execute(db_conn)?;

        if remove_members.is_empty() {
            return LabId(self.id).execute(db_conn);
        }

        let mut deletion = diesel::delete(lab_membership::table).into_boxed();
        for member_id in &remove_members {
            deletion = deletion.or_filter(
                lab_membership::lab_id
                    .eq(&self.id)
                    .and(lab_membership::member_id.eq(member_id)),
            );
        }

        deletion.execute(db_conn)?;

        LabId(self.id).execute(db_conn)
    }
}
