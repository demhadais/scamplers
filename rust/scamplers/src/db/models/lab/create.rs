use crate::db::{
    DbOperation,
    models::lab::{Lab, LabUpdate, LabUpdateFields, NewLab},
};
use diesel::prelude::*;
use scamplers_schema::lab;
use uuid::Uuid;

impl DbOperation<Lab> for NewLab {
    fn execute(
        mut self,
        db_conn: &mut diesel::PgConnection,
    ) -> crate::result::ScamplersResult<Lab> {
        let id: Uuid = diesel::insert_into(lab::table)
            .values(&self)
            .returning(lab::id)
            .get_result(db_conn)?;

        let update_core = LabUpdateFields {
            id,
            ..Default::default()
        };

        self.member_ids.push(self.pi_id);
        let update = LabUpdate {
            fields: update_core,
            add_members: self.member_ids,
            ..Default::default()
        };

        update.execute(db_conn)
    }
}
