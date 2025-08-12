use std::collections::HashMap;

use crate::db::models::index_set::common::{
    INDEX_SET_NAME_ERROR_MESSAGE, INDEX_SET_NAME_REGEX, IndexSetName, insert_kit_name, map_err,
};
use crate::db::{DbOperation, models::index_set::dual::NewDualIndexSet};
use crate::result::ServerError;
use diesel::RunQueryDsl;
use diesel::prelude::*;
use scamplers_schema::dual_index_set;

impl DbOperation<()> for HashMap<String, NewDualIndexSet> {
    fn execute(self, db_conn: &mut diesel::PgConnection) -> crate::result::ScamplersResult<()> {
        #[derive(Insertable)]
        #[diesel(table_name = dual_index_set, check_for_backend(diesel::pg::Pg))]
        struct NewDualIndexSetInner<'a> {
            name: &'a str,
            kit: &'a str,
            well: &'a str,
            index_i7: &'a str,
            index2_workflow_a_i5: &'a str,
            index2_workflow_b_i5: &'a str,
        }

        let Some(index_set_name) = self.keys().next().cloned() else {
            return Ok(());
        };

        if !INDEX_SET_NAME_REGEX.is_match(&index_set_name) {
            return Err(ServerError::builder()
                .message(INDEX_SET_NAME_ERROR_MESSAGE)
                .build()
                .into());
        }

        let kit_name = index_set_name.kit_name().map_err(map_err)?;
        insert_kit_name(kit_name, db_conn).map_err(map_err)?;

        let mut insertables = Vec::with_capacity(self.len());
        for (
            index_set_name,
            NewDualIndexSet {
                index_i7,
                index2_workflow_a_i5,
                index2_workflow_b_i5,
            },
        ) in &self
        {
            let well_name = index_set_name.well_name().map_err(map_err)?;

            insertables.push(NewDualIndexSetInner {
                name: index_set_name,
                kit: kit_name,
                well: well_name,
                index_i7,
                index2_workflow_a_i5,
                index2_workflow_b_i5,
            });
        }

        diesel::insert_into(dual_index_set::table)
            .values(insertables)
            .on_conflict_do_nothing()
            .execute(db_conn)?;

        Ok(())
    }
}
