use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use scamplers_core::{
    model::{
        library_type_specification::LibraryType,
        nucleic_acid::{CdnaHandle, NewCdna, NewCdnaGroup, NewCdnaMeasurement, NewCdnaPreparer},
    },
    result::{CdnaGemsError, CdnaLibraryTypeError},
};
use scamplers_schema::{
    cdna, cdna_measurement, cdna_preparers, chemistry, gems, library_type_specification,
};

use crate::{
    db::model::WriteToDb,
    result::{ScamplersError, ScamplersResult},
};

trait NewCdnaGroupExt {
    fn should_have_same_library_type(&self) -> bool;
    fn to_vec(self) -> Vec<NewCdna>;
}

impl NewCdnaGroupExt for NewCdnaGroup {
    fn should_have_same_library_type(&self) -> bool {
        match self {
            Self::Single(_) | Self::Multiple(_) => false,
            Self::Ocm(_) => true,
        }
    }

    fn to_vec(self) -> Vec<NewCdna> {
        match self {
            Self::Single(c) => vec![c],
            Self::Multiple(g) | Self::Ocm(g) => g,
        }
    }
}

trait NewCdnaVecExt {
    fn validate_gems_id(&self) -> ScamplersResult<()>;
    async fn validate_library_types(
        &self,
        should_have_same_library_type: bool,
        db_conn: &mut AsyncPgConnection,
    ) -> ScamplersResult<()>;
    fn preparers(&self, self_ids: &[CdnaHandle]) -> Vec<NewCdnaPreparer>;
    fn measurements_with_self_ids(self, self_ids: &[CdnaHandle]) -> Vec<NewCdnaMeasurement>;
}

impl NewCdnaVecExt for Vec<NewCdna> {
    fn validate_gems_id(&self) -> ScamplersResult<()> {
        if self.iter().any(|c| c.gems_id != self[0].gems_id) {
            return Err(ScamplersError::new_unprocessable_entity_error(
                CdnaGemsError {
                    message: "all cDNA in a group must come from the same GEMs".to_string(),
                },
            ));
        }

        Ok(())
    }

    async fn validate_library_types(
        &self,
        should_have_same_library_type: bool,
        db_conn: &mut AsyncPgConnection,
    ) -> ScamplersResult<()> {
        fn transform(lib_types_and_volumes: &[(LibraryType, f32)]) -> (Vec<LibraryType>, Vec<f32>) {
            let mut lib_types = Vec::with_capacity(lib_types_and_volumes.len());
            let mut volumes = Vec::with_capacity(lib_types_and_volumes.len());

            for (lib_type, volume) in lib_types_and_volumes {
                lib_types.push(*lib_type);
                volumes.push(*volume);
            }

            (lib_types, volumes)
        }

        let chemistry: Option<String> = gems::table
            .inner_join(chemistry::table)
            .filter(gems::id.eq(&self[0].gems_id))
            .select(gems::chemistry)
            .first(db_conn)
            .await?;

        let mut found_library_types_and_volumes: Vec<_> =
            self.iter().map(|c| (c.library_type, c.volume_µl)).collect();
        found_library_types_and_volumes.sort_by_key(|(lib_type, _)| *lib_type);

        let err = |expected_library_types_and_volumes| {
            let (found_library_types, found_volumes) = transform(&found_library_types_and_volumes);
            let (expected_library_types, expected_volumes) =
                transform(expected_library_types_and_volumes);

            Err(ScamplersError::new_unprocessable_entity_error(
                CdnaLibraryTypeError {
                    found_library_types,
                    expected_library_types,
                    found_volumes,
                    expected_volumes,
                },
            ))
        };

        let Some(chemistry) = chemistry else {
            let expected_library_types_and_volumes = [(LibraryType::ChromatinAccessibility, 40.0)];

            if found_library_types_and_volumes != expected_library_types_and_volumes {
                return err(&expected_library_types_and_volumes);
            }

            return Ok(());
        };

        let expected_library_types_and_volumes: Vec<(LibraryType, f32)> =
            library_type_specification::table
                .filter(library_type_specification::chemistry.eq(chemistry))
                .order_by(library_type_specification::library_type)
                .select((
                    library_type_specification::library_type,
                    library_type_specification::cdna_volume_l,
                ))
                .load(db_conn)
                .await?;

        let mismatch1 = should_have_same_library_type
            && expected_library_types_and_volumes[0] != found_library_types_and_volumes[0];
        let mismatch2 = !(should_have_same_library_type
            || expected_library_types_and_volumes == found_library_types_and_volumes);

        if mismatch1 || mismatch2 {
            return err(&expected_library_types_and_volumes);
        }

        Ok(())
    }

    fn preparers(&self, handles: &[CdnaHandle]) -> Vec<NewCdnaPreparer> {
        self.iter()
            .zip(handles)
            .flat_map(|(cdna, handle)| {
                cdna.preparer_ids.iter().map(|p| NewCdnaPreparer {
                    cdna_id: handle.id,
                    prepared_by: *p,
                })
            })
            .collect()
    }

    fn measurements_with_self_ids(self, handles: &[CdnaHandle]) -> Vec<NewCdnaMeasurement> {
        let mut measurements = Vec::new();

        for (cdna, CdnaHandle { id, .. }) in self.into_iter().zip(handles) {
            for mut m in cdna.measurements {
                m.cdna_id = *id;
                measurements.push(m);
            }
        }

        measurements
    }
}

impl WriteToDb for NewCdnaGroup {
    type Returns = Vec<CdnaHandle>;

    async fn write_to_db(
        self,
        db_conn: &mut diesel_async::AsyncPgConnection,
    ) -> ScamplersResult<Self::Returns> {
        let cdnas_should_have_same_library_type = self.should_have_same_library_type();

        let cdnas = self.to_vec();

        cdnas.validate_gems_id()?;
        cdnas
            .validate_library_types(cdnas_should_have_same_library_type, db_conn)
            .await?;

        let handles = diesel::insert_into(cdna::table)
            .values(&cdnas)
            .returning(CdnaHandle::as_returning())
            .get_results(db_conn)
            .await?;

        let preparers = cdnas.preparers(&handles);
        diesel::insert_into(cdna_preparers::table)
            .values(preparers)
            .execute(db_conn)
            .await?;

        let measurements = cdnas.measurements_with_self_ids(&handles);
        diesel::insert_into(cdna_measurement::table)
            .values(measurements)
            .execute(db_conn)
            .await?;

        Ok(handles)
    }
}
