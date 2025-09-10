use diesel::{PgConnection, prelude::*};
use scamplers_schema::{cdna, library, library_measurement, library_preparers, tenx_assay};
use uuid::Uuid;

use crate::{
    db::{
        DbOperation,
        models::{
            nucleic_acid::{
                common::gems_to_assay,
                library::{Library, LibraryId, LibraryPreparer, NewLibrary, NewLibraryMeasurement},
            },
            tenx_assay::chromium::{LibraryType, LibraryTypeSpecification},
        },
        util::{ChildrenWithSelfId, ManyToMany, ManyToManyChildrenWithSelfId, SetParentId},
    },
    result::{CdnaLibraryTypeError, ScamplersResult},
};

impl SetParentId for NewLibraryMeasurement {
    fn parent_id_mut(&mut self) -> &mut uuid::Uuid {
        &mut self.library_id
    }
}

impl ChildrenWithSelfId<NewLibraryMeasurement> for NewLibrary {
    fn children(&mut self) -> &mut [NewLibraryMeasurement] {
        &mut self.measurements
    }
}

impl ManyToMany for LibraryPreparer {
    fn new(parent_id: uuid::Uuid, child_id: uuid::Uuid) -> Self {
        Self {
            library_id: parent_id,
            prepared_by: child_id,
        }
    }
}

impl ManyToManyChildrenWithSelfId<LibraryPreparer> for NewLibrary {
    fn mtm_children(&self) -> &[uuid::Uuid] {
        &self.preparer_ids
    }
}

impl NewLibrary {
    fn library_type_and_assay_id(
        &self,
        db_conn: &mut PgConnection,
    ) -> ScamplersResult<(LibraryType, Uuid)> {
        let assay_id = cdna::table
            .inner_join(gems_to_assay())
            .filter(cdna::id.eq(self.cdna_id))
            .select((cdna::library_type, tenx_assay::id))
            .first(db_conn)?;

        Ok(assay_id)
    }

    fn validate_type_and_volume(&self, db_conn: &mut PgConnection) -> ScamplersResult<()> {
        let (library_type, assay_id) = self.library_type_and_assay_id(db_conn)?;

        let expected_specifications =
            LibraryTypeSpecification::list_by_assay_id(assay_id, db_conn)?;

        for spec in &expected_specifications {
            if (library_type, self.volume_µl) == (spec.library_type, spec.library_volume_µl) {
                return Ok(());
            }
        }

        Err(CdnaLibraryTypeError {
            expected_specifications,
        }
        .into())
    }
}

impl DbOperation<Library> for NewLibrary {
    fn execute(
        mut self,
        db_conn: &mut diesel::PgConnection,
    ) -> crate::result::ScamplersResult<Library> {
        self.validate_type_and_volume(db_conn)?;

        let id = diesel::insert_into(library::table)
            .values(&self)
            .returning(library::id)
            .get_result(db_conn)?;

        let measurements = self.children_with_self_id(id);
        diesel::insert_into(library_measurement::table)
            .values(&*measurements)
            .execute(db_conn)?;

        let preparers = self.mtm_children_with_self_id(id);
        diesel::insert_into(library_preparers::table)
            .values(&*preparers)
            .execute(db_conn)?;

        LibraryId(id).execute(db_conn)
    }
}
