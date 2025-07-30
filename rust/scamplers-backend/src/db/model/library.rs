use axum::http;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use scamplers_core::{
    model::nucleic_acid::{LibraryHandle, NewLibrary, NewLibraryMeasurement, NewLibraryPreparer},
    result::{LibraryIndexSetError, ScamplersCoreErrorResponse},
};
use scamplers_schema::{library, library_measurement, library_preparers};

use crate::{
    db::model::{HasMeasurements, HasPreparers, Mappping, SetParentId, WriteToDb},
    result::ScamplersResult,
};

impl SetParentId for NewLibraryMeasurement {
    fn parent_id_mut(&mut self) -> &mut uuid::Uuid {
        &mut self.library_id
    }
}

impl Mappping for NewLibraryPreparer {
    fn new(parent_id: uuid::Uuid, child_id: uuid::Uuid) -> Self {
        Self {
            library_id: parent_id,
            prepared_by: child_id,
        }
    }
}

impl HasMeasurements for NewLibrary {
    type Measurement = NewLibraryMeasurement;

    fn measurements(&mut self) -> &mut [Self::Measurement] {
        &mut self.measurements
    }
}

impl HasPreparers for NewLibrary {
    type Preparers = NewLibraryPreparer;

    fn children(&self) -> &[uuid::Uuid] {
        &self.preparer_ids
    }
}

impl WriteToDb for &[NewLibraryMeasurement] {
    type Returns = ();

    async fn write_to_db(
        self,
        db_conn: &mut diesel_async::AsyncPgConnection,
    ) -> ScamplersResult<Self::Returns> {
        diesel::insert_into(library_measurement::table)
            .values(self)
            .execute(db_conn)
            .await?;

        Ok(())
    }
}

impl WriteToDb for NewLibrary {
    type Returns = LibraryHandle;

    async fn write_to_db(
        mut self,
        db_conn: &mut diesel_async::AsyncPgConnection,
    ) -> ScamplersResult<Self::Returns> {
        if self.single_index_set_name.is_none() == self.dual_index_set_name.is_none() {
            return Err(ScamplersCoreErrorResponse::builder()
                .status(http::StatusCode::UNPROCESSABLE_ENTITY)
                .error(LibraryIndexSetError {
                    message: "must provide exactly one of `single_index_set_name` or \
                              'dual_index_set_name'"
                        .to_string(),
                })
                .build()
                .into());
        }

        let handle = diesel::insert_into(library::table)
            .values(&self)
            .returning(LibraryHandle::as_select())
            .get_result(db_conn)
            .await?;

        let measurements = self.measurements_with_self_id(handle.id);
        measurements.write_to_db(db_conn).await?;

        let preparers = self.preparers(handle.id);
        diesel::insert_into(library_preparers::table)
            .values(preparers)
            .execute(db_conn)
            .await?;

        Ok(handle)
    }
}
