use scamplers_macros::db_insertion;
use time::OffsetDateTime;
use uuid::Uuid;
use valid_string::ValidString;

use crate::db::models::suspension::common::SuspensionMeasurementFields;

pub(super) const MAX_GEMS_IN_NON_OCM_RUN: usize = 8;

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::gems))]
#[cfg_attr(feature = "python", pyo3(name = "_NewGemsCommon"))]
pub struct NewGemsCommon {
    #[garde(dive)]
    pub readable_id: ValidString,
    #[serde(skip)]
    #[builder(skip)]
    pub chromium_run_id: Uuid,
}

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::chip_loading))]
#[cfg_attr(feature = "python", pyo3(name = "_NewChipLoadingCommon"))]
pub struct NewChipLoadingCommon {
    #[serde(skip)]
    #[builder(skip)]
    pub gems_id: Uuid,
    #[garde(custom(SuspensionMeasurementFields::is_volume), dive)]
    pub suspension_volume_loaded: SuspensionMeasurementFields,
    #[garde(custom(SuspensionMeasurementFields::is_volume), dive)]
    pub buffer_volume_loaded: SuspensionMeasurementFields,
    #[garde(dive)]
    pub notes: Option<ValidString>,
}

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::chromium_run))]
#[cfg_attr(feature = "python", pyo3(name = "_NewChipLoadingCommon"))]
pub struct NewChromiumRunCommon {
    #[garde(dive)]
    pub readable_id: ValidString,
    pub run_at: OffsetDateTime,
    pub run_by: Uuid,
    pub succeeded: bool,
    #[garde(dive)]
    pub notes: Option<ValidString>,
}
