use scamplers_core::model::{
    chromium_run::{
        ChromiumRun, NewChipLoadingCommon, NewChromiumRun, NewMultiplexGems,
        NewSingleplexChipLoading, NewSingleplexGems,
    },
    suspension::{MeasurementDataCore, SuspensionMeasurement},
};
use uuid::Uuid;

use crate::db::model::WriteToDb;

trait MeasurementDataCoreExt {
    fn validate_volume(&self) -> crate::db::error::Result<()>;
}

impl MeasurementDataCoreExt for MeasurementDataCore {
    fn validate_volume(&self) -> crate::db::error::Result<()> {
        if !matches!(self, Self::Volume { .. }) {
            return Err(crate::db::error::Error::Other {
                message: "invalid chip loading volume".to_string(),
            });
        }

        Ok(())
    }
}

trait NewChipLoadingCommonExt {
    fn validate_volumes(&self) -> crate::db::error::Result<()>;
}

impl NewChipLoadingCommonExt for NewChipLoadingCommon {
    fn validate_volumes(&self) -> crate::db::error::Result<()> {
        let volumes = [&self.suspension_volume_loaded, &self.buffer_volume_loaded];

        Ok(())
    }
}

enum NewGemsWrapper<'a> {
    Singleplex(&'a [NewSingleplexGems]),
    Multiplex(&'a [NewMultiplexGems]),
}

impl NewGemsWrapper {
    fn validate_volumes(&self) -> crate::db::error::Result<()> {
        let (suspension_volume, buffer_volume) = match self {
            Self::Singleplex(NewSingleplexGems {
                loading: NewSingleplexChipLoading { inner, .. },
                ..
            }) => [inner.suspension_volume_loaded, inner.buffer_volume_loaded],
        };
    }
}

trait NewChromiumRunExt {
    fn gems_chip_loading(&mut self, self_id: Uuid) -> NewGemsWrapper;
}

impl NewChromiumRunExt for NewChromiumRun {
    fn gems_chip_loading(&mut self, self_id: Uuid) -> NewGemsWrapper {
        let gems = match self {
            Self::Singleplex(singleplex) => singleplex.gems,
        };
    }
}

impl WriteToDb for NewChromiumRun {
    type Returns = ChromiumRun;

    async fn write_to_db(
        self,
        db_conn: &mut diesel_async::AsyncPgConnection,
    ) -> crate::db::error::Result<Self::Returns> {
        match self {
            NewChromiumRun::Singleplex(singleplex) => todo!(),
            NewChromiumRun::Ocm(ocm) => todo!(),
            NewChromiumRun::PoolMultiplex(multiplex) => todo!(),
        };

        Ok(todo!())
    }
}
