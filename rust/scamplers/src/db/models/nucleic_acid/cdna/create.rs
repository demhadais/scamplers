use std::collections::HashMap;

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
            nucleic_acid::cdna::{
                Cdna, CdnaPreparer, CdnaQuery, NewCdna, NewCdnaGroup, NewCdnaMeasurement,
            },
            tenx_assay::chromium::LibraryType,
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
    fn library_types(&self) -> Vec<LibraryType> {
        match self {
            Self::Single(c) => vec![c.library_type],
            Self::Ocm(c) | Self::Multiple(c) => c.iter().map(|c| c.library_type).collect(),
        }
    }

    fn library_types_and_volumes(&self) -> Vec<(LibraryType, f32)> {
        match self {
            Self::Single(c) => vec![(c.library_type, c.volume_µl)],
            Self::Ocm(c) | Self::Multiple(c) => {
                c.iter().map(|c| (c.library_type, c.volume_µl)).collect()
            }
        }
    }

    fn group_ocm(&self) -> Option<Vec<Vec<&NewCdna>>> {
        let Self::Ocm(ocm_cdnas) = self else {
            return None;
        };

        let mut grouped_cdnas = Vec::with_capacity(ocm_cdnas.len());
        let mut seen_cdnas = Vec::with_capacity(ocm_cdnas.len());

        for (i, c1) in ocm_cdnas.iter().enumerate() {
            let mut group = Vec::with_capacity(ocm_cdnas.len());
            group.push(c1);

            for c2 in &ocm_cdnas[i..ocm_cdnas.len()] {
                if c1.library_type != c2.library_type && !seen_cdnas.contains(&c2) {
                    group.push(c2);
                    seen_cdnas.push(c2);
                }
            }

            grouped_cdnas.push(group)
        }

        Some(grouped_cdnas)
    }

    fn validate_library_types(&self, db_conn: &mut PgConnection) -> ScamplersResult<()> {
        let mut found_library_types_and_volumes = self.library_types_and_volumes();

        found_library_types_and_volumes.sort_by_key(|(lib_type, _)| *lib_type);

        let mut expected_specifications: Vec<LibraryTypeSpe> = library_type_specification::table
            .filter(library_type_specification::library_type.eq_any(self.library_types()))
            .order_by((
                library_type_specification::chemistry,
                library_type_specification::library_type,
            ))
            .select(LibraryTypeSpecification::as_select())
            .load(db_conn)?;

        let expected_specifications_grouped_by_chemistry =
            expected_specifications.chunk_by(|spec1, spec2| spec1.chemistry == spec2.chemistry);

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

trait VecExt {
    fn library_types(&self) -> Vec<LibraryType>;
    fn library_types_and_volumes(&self) -> Vec<(LibraryType, f32)>;
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

trait IsAtac {
    fn is_atac(&self) -> bool;
}

impl IsAtac for Vec<(LibraryType, f32)> {
    fn is_atac(&self) -> bool {
        if self.len() == 1 && self[0].0 == LibraryType::ChromatinAccessibility {
            return true;
        }

        false
    }
}

impl VecExt for Vec<NewCdna> {
    fn library_types(&self) -> Vec<LibraryType> {
        self.iter().map(|c| c.library_type).collect()
    }

    fn library_types_and_volumes(&self) -> Vec<(LibraryType, f32)> {
        self.iter().map(|c| (c.library_type, c.volume_µl)).collect()
    }

    fn validate_library_types(&self, db_conn: &mut PgConnection) -> ScamplersResult<()> {
        let mut found_library_types_and_volumes = self.library_types_and_volumes();

        if found_library_types_and_volumes.is_atac() {
            return Ok(validate_chromatin_accessibility_cdna(
                &found_library_types_and_volumes,
            )?);
        }

        found_library_types_and_volumes.sort_by_key(|(lib_type, _)| *lib_type);

        let mut expected_specifications: Vec<LibraryTypeSpecification> =
            library_type_specification::table
                .filter(library_type_specification::library_type.eq_any(self.library_types()))
                .order_by((
                    library_type_specification::chemistry,
                    library_type_specification::library_type,
                ))
                .select(LibraryTypeSpecification::as_select())
                .load(db_conn)?;

        let expected_specifications_grouped_by_chemistry =
            expected_specifications.chunk_by(|spec1, spec2| spec1.chemistry == spec2.chemistry);

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
        let mut cdnas = self.into_vec();

        cdnas.validate_library_types(db_conn)?;

        let ids = diesel::insert_into(cdna::table)
            .values(&cdnas)
            .returning(cdna::id)
            .get_results(db_conn)?;

        let mut measurements = Vec::with_capacity(cdnas.len());
        let mut preparers = Vec::with_capacity(cdnas.len());

        // We have to do two iterations for the borrow checker (one for measurements and
        // one for preparers)
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
