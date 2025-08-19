use diesel::prelude::*;
use scamplers_schema::single_index_set;

use crate::{
    db::{
        DbOperation,
        models::index_set::{
            common::{IndexSetName, insert_kit_name, map_err},
            single::NewSingleIndexSet,
        },
    },
    result::ScamplersResult,
};

impl DbOperation<()> for Vec<NewSingleIndexSet> {
    fn execute(self, db_conn: &mut PgConnection) -> ScamplersResult<()> {
        #[derive(Insertable)]
        #[diesel(table_name = single_index_set, check_for_backend(diesel::pg::Pg))]
        struct SingleIndexSetInsertion<'a> {
            name: &'a str,
            kit: &'a str,
            well: &'a str,
            sequences: &'a [String],
        }

        #[allow(clippy::get_first)]
        let Some(NewSingleIndexSet(index_set_name, ..)) = self.get(0).cloned() else {
            return Ok(());
        };

        let kit_name = index_set_name.kit_name().map_err(map_err)?;
        insert_kit_name(kit_name, db_conn).map_err(map_err)?;

        let mut insertables = Vec::with_capacity(self.len());
        for NewSingleIndexSet(index_set_name, sequences) in &self {
            let well_name = index_set_name.well_name().map_err(map_err)?;

            insertables.push(SingleIndexSetInsertion {
                name: index_set_name,
                kit: kit_name,
                well: well_name,
                sequences,
            });
        }

        diesel::insert_into(single_index_set::table)
            .values(insertables)
            .on_conflict_do_nothing()
            .execute(db_conn)?;

        Ok(())
    }
}
