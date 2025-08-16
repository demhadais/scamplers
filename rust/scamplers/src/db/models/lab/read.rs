use diesel::prelude::*;
use scamplers_schema::{lab, lab_membership, person};

use crate::db::{
    DbOperation,
    models::{
        lab::{Lab, LabCore, LabId, LabQuery},
        person::PersonSummary,
    },
};

impl DbOperation<Lab> for LabId {
    fn execute(self, db_conn: &mut diesel::PgConnection) -> crate::result::ScamplersResult<Lab> {
        let core = lab::table
            .inner_join(person::table)
            .filter(lab::id.eq(&self.0))
            .select(LabCore::as_select())
            .first(db_conn)?;

        let members = lab_membership::table
            .filter(lab_membership::lab_id.eq(&self.0))
            .inner_join(person::table)
            .select(PersonSummary::as_select())
            .load(db_conn)?;

        Ok(Lab { core, members })
    }
}

impl DbOperation<Vec<Lab>> for LabQuery {
    fn execute(self, db_conn: &mut PgConnection) -> crate::result::ScamplersResult<Vec<Lab>> {
        todo!()
    }
}
