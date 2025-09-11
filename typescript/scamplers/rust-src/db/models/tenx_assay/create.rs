use diesel::prelude::*;
use scamplers_schema::{library_type_specification, tenx_assay};

use crate::db::{
    DbOperation,
    models::tenx_assay::{
        NewTenxAssay, TenxAssay, TenxAssayId,
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

impl NewChromiumAssay {
    fn set_library_types(&mut self) {
        self.library_types = self
            .library_type_specifications
            .iter()
            .map(|spec| spec.library_type)
            .collect();

        self.library_types.sort();
    }
}

impl DbOperation<TenxAssay> for NewTenxAssay {
    fn execute(
        self,
        db_conn: &mut diesel::PgConnection,
    ) -> crate::result::ScamplersResult<TenxAssay> {
        let mut assay = match self {
            NewTenxAssay::Chromium(mut a) => {
                a.set_library_types();
                a
            }
        };

        let id = diesel::insert_into(tenx_assay::table)
            .values(&assay)
            .returning(tenx_assay::id)
            .get_result(db_conn)?;

        let lib_type_specs = assay.children_with_self_id(id);

        diesel::insert_into(library_type_specification::table)
            .values(&*lib_type_specs)
            .execute(db_conn)?;

        TenxAssayId(id).execute(db_conn)
    }
}
