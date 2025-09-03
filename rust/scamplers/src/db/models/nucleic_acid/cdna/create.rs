use diesel::prelude::*;
use scamplers_schema::{
    cdna, cdna_measurement, cdna_preparers, chemistry,
    gems::{self},
    library_type_specification,
};
use uuid::Uuid;

use crate::{
    db::{
        DbOperation,
        models::{
            library_type_specification::{
                LibraryType, LibraryTypeSpecification,
                chromatin_accessibility_library_specification,
            },
            nucleic_acid::cdna::{Cdna, CdnaQuery, NewCdna, NewCdnaGroup, NewCdnaMeasurement},
        },
        util::{ChildrenWithSelfId, ManyToMany, ManyToManyChildrenWithSelfId, SetParentId},
    },
    result::{CdnaLibraryTypeError, ScamplersResult},
};

impl SetParentId for NewCdnaMeasurement {
    fn parent_id_mut(&mut self) -> &mut uuid::Uuid {
        &mut self.cdna_id
    }
}

impl ChildrenWithSelfId<NewCdnaMeasurement> for NewCdna {
    fn children(&mut self) -> &mut [NewCdnaMeasurement] {
        &mut self.measurements
    }
}

#[derive(Insertable)]
struct CdnaPreparer {
    cdna_id: Uuid,
    prepared_by: Uuid,
}

impl ManyToMany for CdnaPreparer {
    fn new(parent_id: Uuid, child_id: Uuid) -> Self {
        Self {
            cdna_id: parent_id,
            prepared_by: child_id,
        }
    }
}

impl ManyToManyChildrenWithSelfId<CdnaPreparer> for NewCdna {
    fn mtm_children(&self) -> &[Uuid] {
        &self.preparer_ids
    }
}

impl NewCdnaGroup {
    fn to_vec(self) -> Vec<NewCdna> {
        match self {
            Self::Single(c) => vec![c],
            Self::Multiple(g) | Self::Ocm(g) => g,
        }
    }
}

trait VecExt {
    fn validate_library_types(&self, db_conn: &mut PgConnection) -> ScamplersResult<()>;
}

fn validate_chromatin_accessibility_cdna(
    library_types_and_volumes: &[(LibraryType, f32)],
) -> Result<(), CdnaLibraryTypeError> {
    let expected_specification = chromatin_accessibility_library_specification();

    let expected_library_types_and_volumes = [(
        expected_specification.library_type,
        expected_specification.cdna_volume_µl,
    )];

    if library_types_and_volumes != expected_library_types_and_volumes {
        return Err(CdnaLibraryTypeError {
            expected_specifications: vec![expected_specification],
        });
    }

    Ok(())
}

impl VecExt for Vec<NewCdna> {
    fn validate_library_types(&self, db_conn: &mut PgConnection) -> ScamplersResult<()> {
        let chemistry: Option<String> = gems::table
            .inner_join(chemistry::table)
            .filter(gems::id.eq(&self[0].gems_id))
            .select(gems::chemistry)
            .first(db_conn)?;

        let mut found_library_types_and_volumes: Vec<_> =
            self.iter().map(|c| (c.library_type, c.volume_µl)).collect();
        found_library_types_and_volumes.sort_by_key(|(lib_type, _)| *lib_type);

        let Some(chemistry) = chemistry else {
            return Ok(validate_chromatin_accessibility_cdna(
                &found_library_types_and_volumes,
            )?);
        };

        let mut expected_specifications: Vec<LibraryTypeSpecification> =
            library_type_specification::table
                .filter(library_type_specification::chemistry.eq(chemistry))
                .order_by(library_type_specification::library_type)
                .select(LibraryTypeSpecification::as_select())
                .load(db_conn)?;

        // If the chemistry in the db only has one library type, then we know all the
        // new cDNA must conform to this specification
        if expected_specifications.len() == 1 {
            // Fill in the rest of the `Vec`
            for _ in 0..(self.len() - 1) {
                expected_specifications.push(expected_specifications[0].clone());
            }
        }

        let expected_library_types_and_volumes: Vec<_> = expected_specifications
            .iter()
            .map(|s| (s.library_type, s.cdna_volume_µl))
            .collect();

        if expected_library_types_and_volumes != found_library_types_and_volumes {
            return Err(CdnaLibraryTypeError {
                expected_specifications,
            }
            .into());
        }

        Ok(())
    }
}

impl DbOperation<Vec<Cdna>> for NewCdnaGroup {
    fn execute(
        self,
        db_conn: &mut diesel::PgConnection,
    ) -> crate::result::ScamplersResult<Vec<Cdna>> {
        let mut cdnas = self.to_vec();

        cdnas.validate_library_types(db_conn)?;

        let ids = diesel::insert_into(cdna::table)
            .values(&cdnas)
            .returning(cdna::id)
            .get_results(db_conn)?;

        let mut measurements = Vec::with_capacity(cdnas.len());
        let mut preparers = Vec::with_capacity(cdnas.len());

        // We have to do two iterations for borrow checker (one for measurements and one
        // for preparers)
        for (cdna, id) in cdnas.iter_mut().zip(&ids) {
            measurements.extend(cdna.children_with_self_id(*id).iter_mut().map(|m| &*m));
        }

        diesel::insert_into(cdna_measurement::table)
            .values(measurements)
            .execute(db_conn)?;

        for (cdna, id) in cdnas.iter_mut().zip(&ids) {
            preparers.extend(cdna.mtm_children_with_self_id(*id));
        }

        diesel::insert_into(cdna_preparers::table)
            .values(preparers)
            .execute(db_conn)?;

        CdnaQuery::builder().ids(ids).build().execute(db_conn)
    }
}
