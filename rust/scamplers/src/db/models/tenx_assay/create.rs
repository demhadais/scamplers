use diesel::prelude::*;
use scamplers_schema::{library_type_specification, tenx_assay};

use crate::db::{
    DbOperation,
    models::tenx_assay::{
        NewTenxAssay,
        chromium::{LibraryTypeSpecification, NewChromiumAssay},
    },
    util::{ChildrenWithSelfId, SetParentId},
};

impl SetParentId for LibraryTypeSpecification {
    fn parent_id_mut(&mut self) -> &mut uuid::Uuid {
        &mut self.assay_id
    }
}

impl ChildrenWithSelfId<LibraryTypeSpecification> for NewChromiumAssay {
    fn children(&mut self) -> &mut [LibraryTypeSpecification] {
        &mut self.library_type_specifications
    }
}

impl DbOperation<()> for Vec<NewTenxAssay> {
    fn execute(self, db_conn: &mut diesel::PgConnection) -> crate::result::ScamplersResult<()> {
        let mut chromium_assays = Vec::with_capacity(self.len());

        for assay in self {
            match assay {
                NewTenxAssay::Chromium(a) => chromium_assays.push(a),
            }
        }

        let ids = diesel::insert_into(tenx_assay::table)
            .values(&chromium_assays)
            .on_conflict_do_nothing()
            .returning(tenx_assay::id)
            .get_results(db_conn)?;

        let mut lib_type_specs = Vec::with_capacity(chromium_assays.len());
        for (assay, id) in chromium_assays.iter_mut().zip(&ids) {
            lib_type_specs.extend(&*assay.children_with_self_id(*id));
        }

        diesel::insert_into(library_type_specification::table)
            .values(lib_type_specs)
            .execute(db_conn)?;

        Ok(())
    }
}
