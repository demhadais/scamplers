use scamplers_macros::db_insertion;
#[cfg(feature = "backend")]
use scamplers_schema::{chip_loading, chromium_run, gems};
use time::OffsetDateTime;
use uuid::Uuid;
use valid_string::ValidString;

use crate::model::suspension::MeasurementDataCore;

pub(super) const MAX_GEMS_IN_CHROMIUM_RUN: usize = 8;

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = gems))]
pub(super) struct NewGemsCommon {
    #[garde(dive)]
    pub readable_id: ValidString,
    #[serde(skip)]
    pub n_samples: i32,
    #[garde(dive)]
    pub chemistry: ValidString,
    #[serde(skip)]
    pub chromium_run_id: Uuid,
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = chip_loading))]
pub(super) struct NewChipLoadingCommon {
    #[serde(skip)]
    pub gems_id: Uuid,
    #[garde(dive)]
    pub suspension_volume_loaded: MeasurementDataCore,
    #[garde(dive)]
    pub buffer_volume_loaded: MeasurementDataCore,
    #[garde(dive)]
    pub notes: Option<ValidString>,
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = chromium_run))]
pub(super) struct NewChromiumRunCommon {
    #[garde(dive)]
    pub readable_id: ValidString,
    pub run_at: OffsetDateTime,
    pub succeeded: bool,
    #[garde(dive)]
    pub notes: Option<ValidString>,
    pub run_by: Uuid,
}
