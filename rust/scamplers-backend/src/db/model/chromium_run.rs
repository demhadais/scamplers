use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use scamplers_core::{
    model::{
        chromium_run::{
            ChromiumRun, ChromiumRunSummary, GemsHandle, NewChipLoadingCommon, NewChromiumRun,
            NewOcmGems, NewPoolMultiplexChipLoading, NewPoolMultiplexGems,
            NewSingleplexChipLoading, NewSingleplexGems,
        },
        suspension::MeasurementDataCore,
    },
    result::InvalidMeasurementError,
};
use scamplers_schema::{
    chip_loading,
    chromium_run::{self},
    gems::{self},
};
use uuid::Uuid;

use crate::{
    db::model::WriteToDb,
    result::{ScamplersError, ScamplersResult},
};

trait MeasurementDataCoreExt {
    fn validate_volume(&self) -> ScamplersResult<()>;
}

impl MeasurementDataCoreExt for MeasurementDataCore {
    fn validate_volume(&self) -> ScamplersResult<()> {
        if !matches!(self, Self::Volume { .. }) {
            return Err(ScamplersError::new_unprocessable_entity_error(
                InvalidMeasurementError {
                    message: "invalid chip-loading volume".to_string(),
                },
            ));
        }

        Ok(())
    }
}

enum NewChipLoadingWrapper {
    SingleplexOcm(Vec<NewSingleplexChipLoading>),
    PoolMultiplex(Vec<NewPoolMultiplexChipLoading>),
}
impl NewChipLoadingWrapper {
    async fn write_to_db(&self, db_conn: &mut AsyncPgConnection) -> ScamplersResult<()> {
        match self {
            Self::SingleplexOcm(l) => {
                diesel::insert_into(chip_loading::table)
                    .values(l)
                    .execute(db_conn)
                    .await?
            }
            Self::PoolMultiplex(l) => {
                diesel::insert_into(chip_loading::table)
                    .values(l)
                    .execute(db_conn)
                    .await?
            }
        };

        Ok(())
    }
}

enum NewGemsWrapper {
    Singleplex(Vec<NewSingleplexGems>),
    Ocm(Vec<NewOcmGems>),
    PoolMultiplex(Vec<NewPoolMultiplexGems>),
}
impl NewGemsWrapper {
    fn loadings(mut self, self_ids: Vec<Uuid>) -> ScamplersResult<NewChipLoadingWrapper> {
        let inner_loadings: Vec<&mut NewChipLoadingCommon> = match &mut self {
            Self::Singleplex(gems) => gems.iter_mut().map(|g| &mut g.loading.inner).collect(),
            Self::Ocm(gems) => gems
                .iter_mut()
                .flat_map(|g| g.loading.iter_mut().map(|l| &mut l.0.inner))
                .collect(),
            Self::PoolMultiplex(gems) => gems.iter_mut().map(|g| &mut g.loading.inner).collect(),
        };

        for (loading, gems_id) in inner_loadings.into_iter().zip(self_ids) {
            loading.suspension_volume_loaded.validate_volume()?;
            loading.buffer_volume_loaded.validate_volume()?;

            loading.gems_id = gems_id;
        }

        let chip_loadings = match self {
            Self::Singleplex(gems) => {
                NewChipLoadingWrapper::SingleplexOcm(gems.into_iter().map(|g| g.loading).collect())
            }
            Self::Ocm(gems) => NewChipLoadingWrapper::SingleplexOcm(
                gems.into_iter()
                    .flat_map(|g| g.loading.into_iter().map(|l| l.0))
                    .collect(),
            ),
            Self::PoolMultiplex(gems) => {
                NewChipLoadingWrapper::PoolMultiplex(gems.into_iter().map(|g| g.loading).collect())
            }
        };

        Ok(chip_loadings)
    }

    async fn write_to_db(
        &self,
        db_conn: &mut AsyncPgConnection,
    ) -> ScamplersResult<Vec<GemsHandle>> {
        let handles = match self {
            NewGemsWrapper::Singleplex(g) => {
                diesel::insert_into(gems::table)
                    .values(g)
                    .returning(GemsHandle::as_select())
                    .get_results(db_conn)
                    .await?
            }
            NewGemsWrapper::Ocm(g) => {
                diesel::insert_into(gems::table)
                    .values(g)
                    .returning(GemsHandle::as_select())
                    .get_results(db_conn)
                    .await?
            }
            NewGemsWrapper::PoolMultiplex(g) => {
                diesel::insert_into(gems::table)
                    .values(g)
                    .returning(GemsHandle::as_select())
                    .get_results(db_conn)
                    .await?
            }
        };

        Ok(handles)
    }
}

trait NewChromiumRunExt {
    fn gems(self, self_id: Uuid) -> NewGemsWrapper;
}

impl NewChromiumRunExt for NewChromiumRun {
    fn gems(self, self_id: Uuid) -> NewGemsWrapper {
        match self {
            Self::Singleplex(mut chromium_run) => {
                for g in &mut chromium_run.gems {
                    g.inner.chromium_run_id = self_id;
                }

                NewGemsWrapper::Singleplex(chromium_run.gems)
            }
            Self::Ocm(mut chromium_run) => {
                for g in &mut chromium_run.gems {
                    g.inner.chromium_run_id = self_id;
                }

                NewGemsWrapper::Ocm(chromium_run.gems)
            }
            Self::PoolMultiplex(mut chromium_run) => {
                for g in &mut chromium_run.gems {
                    g.inner.chromium_run_id = self_id;
                }

                NewGemsWrapper::PoolMultiplex(chromium_run.gems)
            }
        }
    }
}

impl WriteToDb for NewChromiumRun {
    type Returns = ChromiumRun;

    async fn write_to_db(
        self,
        db_conn: &mut diesel_async::AsyncPgConnection,
    ) -> ScamplersResult<Self::Returns> {
        let summary = match &self {
            NewChromiumRun::Singleplex(singleplex) => {
                diesel::insert_into(chromium_run::table)
                    .values(singleplex)
                    .returning(ChromiumRunSummary::as_returning())
                    .get_result(db_conn)
                    .await?
            }
            NewChromiumRun::Ocm(ocm) => {
                diesel::insert_into(chromium_run::table)
                    .values(ocm)
                    .returning(ChromiumRunSummary::as_returning())
                    .get_result(db_conn)
                    .await?
            }
            NewChromiumRun::PoolMultiplex(multiplex) => {
                diesel::insert_into(chromium_run::table)
                    .values(multiplex)
                    .returning(ChromiumRunSummary::as_returning())
                    .get_result(db_conn)
                    .await?
            }
        };

        let new_gems = self.gems(summary.handle.id);
        let gems_handles = new_gems.write_to_db(db_conn).await?;

        let loadings = new_gems.loadings(gems_handles.iter().map(|g| g.id).collect())?;
        loadings.write_to_db(db_conn).await?;

        Ok(ChromiumRun {
            summary,
            gems: gems_handles,
        })
    }
}
