use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use scamplers_core::model::nucleic_acid::{
    CdnaHandle, NewCdna, NewCdnaMeasurement, NewCdnaPreparer,
};
use scamplers_schema::{
    cdna, cdna_measurement, cdna_preparers, chemistry, gems, library_type_specification,
};
use uuid::Uuid;

use crate::db::model::WriteToDb;

trait NewCdnaExt {
    fn measurements(&mut self, self_id: Uuid) -> &[NewCdnaMeasurement];
    fn preparers(&mut self, self_id: Uuid) -> Vec<NewCdnaPreparer>;
}

impl NewCdnaExt for NewCdna {
    fn measurements(&mut self, self_id: Uuid) -> &[NewCdnaMeasurement] {
        for m in &mut self.measurements {
            m.cdna_id = self_id;
        }

        &self.measurements
    }

    fn preparers(&mut self, self_id: Uuid) -> Vec<NewCdnaPreparer> {
        self.preparer_ids
            .iter()
            .map(|p| NewCdnaPreparer {
                cdna_id: self_id,
                prepared_by: *p,
            })
            .collect()
    }
}

// TODO: VALIDATE LIBRARY TYPE
impl WriteToDb for NewCdna {
    type Returns = CdnaHandle;

    async fn write_to_db(
        mut self,
        db_conn: &mut diesel_async::AsyncPgConnection,
    ) -> crate::db::error::Result<Self::Returns> {
        let chemistry: Option<String> = gems::table
            .inner_join(chemistry::table)
            .filter(gems::id.eq(&self.gems_id))
            .select(gems::chemistry)
            .first(db_conn)
            .await?;

        if let Some(chemistry) = chemistry {
            let expected_library_types: Vec<String> = library_type_specification::table
                .filter(library_type_specification::chemistry.eq(chemistry))
                .select(library_type_specification::library_type)
                .load(db_conn)
                .await?;

            let library_type = self.library_type.to_string();

            if !expected_library_types.contains(&library_type) {
                return Err(crate::db::error::Error::Other {
                    message: format!(
                        "invalid library type {library_type} - expected one of {expected_library_types:?}"
                    ),
                });
            }
        }

        let handle = diesel::insert_into(cdna::table)
            .values(&self)
            .returning(CdnaHandle::as_returning())
            .get_result(db_conn)
            .await?;

        let measurements = self.measurements(handle.id);
        diesel::insert_into(cdna_measurement::table)
            .values(measurements)
            .execute(db_conn)
            .await?;

        let preparers = self.preparers(handle.id);
        diesel::insert_into(cdna_preparers::table)
            .values(preparers)
            .execute(db_conn)
            .await?;

        Ok(handle)
    }
}
