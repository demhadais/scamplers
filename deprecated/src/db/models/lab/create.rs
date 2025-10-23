use diesel::prelude::*;
use scamplers_schema::lab;
use uuid::Uuid;

use crate::db::{
    DbOperation,
    models::lab::{Lab, LabUpdate, NewLab},
};

impl DbOperation<Lab> for NewLab {
    fn execute(
        mut self,
        db_conn: &mut diesel::PgConnection,
    ) -> crate::result::ScamplersResult<Lab> {
        let id: Uuid = diesel::insert_into(lab::table)
            .values(&self)
            .returning(lab::id)
            .get_result(db_conn)?;

        self.member_ids.push(self.pi_id);

        LabUpdate::builder()
            .id(id)
            .add_members(self.member_ids)
            .build()
            .execute(db_conn)
    }
}
