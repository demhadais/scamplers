use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use scamplers_core::model::{
    library_type_specification::LibraryType,
    nucleic_acid::{CdnaHandle, NewCdna, NewCdnaGroup, NewCdnaMeasurement, NewCdnaPreparer},
};
use scamplers_schema::{
    cdna, cdna_measurement, cdna_preparers, chemistry, gems, library_type_specification,
};

use crate::db::{error::Error, model::WriteToDb};

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
    fn validate_gems_id(&self) -> crate::db::error::Result<()>;
    async fn validate_library_types(
        &self,
        should_have_same_library_type: bool,
        db_conn: &mut AsyncPgConnection,
    ) -> crate::db::error::Result<()>;
    fn preparers(&self, self_ids: &[CdnaHandle]) -> Vec<NewCdnaPreparer>;
    fn measurements_with_self_ids(self, self_ids: &[CdnaHandle]) -> Vec<NewCdnaMeasurement>;
}

impl NewCdnaVecExt for Vec<NewCdna> {
    fn validate_gems_id(&self) -> crate::db::error::Result<()> {
        if self.iter().any(|c| c.gems_id != self[0].gems_id) {
            return Err(crate::db::error::Error::Other {
                message: "all cDNA in a group must derive from the same GEMs".to_string(),
            });
        }

        Ok(())
    }

    async fn validate_library_types(
        &self,
        should_have_same_library_type: bool,
        db_conn: &mut AsyncPgConnection,
    ) -> crate::db::error::Result<()> {
        let chemistry: Option<String> = gems::table
            .inner_join(chemistry::table)
            .filter(gems::id.eq(&self[0].gems_id))
            .select(gems::chemistry)
            .first(db_conn)
            .await?;

        let mut library_types: Vec<_> = self.iter().map(|c| c.library_type).collect();
        library_types.sort();

        let Some(chemistry) = chemistry else {
            if library_types != [LibraryType::ChromatinAccessibility] {
                return Err(Error::Other {
                    message: "GEMs without chemistry must produce one chromatin accessibility \
                              library"
                        .to_string(),
                });
            }

            return Ok(());
        };

        let expected_library_types: Vec<LibraryType> = library_type_specification::table
            .filter(library_type_specification::chemistry.eq(chemistry))
            .order_by(library_type_specification::library_type)
            .select(library_type_specification::library_type)
            .load(db_conn)
            .await?;

        let err = Err(Error::Other {
            message: format!(
                "invalid library types {library_types:?} - expected one of \
                 {expected_library_types:?}"
            ),
        });

        if should_have_same_library_type && expected_library_types[0] != library_types[0] {
            return err;
        } else if !should_have_same_library_type && expected_library_types != library_types {
            return err;
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
    ) -> crate::db::error::Result<Self::Returns> {
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
