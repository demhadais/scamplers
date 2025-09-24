use scamplers_macros::db_insertion;
use valid_string::ValidString;

use crate::db::validators::is_10x_genomics_url;

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::tenx_assay))]
pub struct NewTenxAssayCommon {
    #[garde(dive)]
    pub name: ValidString,
    #[garde(dive)]
    pub chemistry_version: ValidString,
    #[garde(dive, custom(is_10x_genomics_url))]
    pub protocol_url: ValidString,
}
