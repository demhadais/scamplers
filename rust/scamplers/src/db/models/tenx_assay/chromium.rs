use crate::db::models::tenx_assay::common::NewTenxAssayCommon;
use scamplers_macros::{db_insertion, db_simple_enum};
use uuid::Uuid;
use valid_string::ValidString;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[db_simple_enum]
#[cfg_attr(feature = "python", pyo3(module = "scamplepy.common"))]
#[derive(Eq, PartialOrd, Ord)]
pub enum LibraryType {
    AntibodyCapture,
    AntigenCapture,
    ChromatinAccessibility,
    CrisprGuideCapture,
    Custom,
    GeneExpression,
    MultiplexingCapture,
    Vdj,
    VdjB,
    VdjT,
    VdjTGd,
}

#[db_simple_enum]
pub enum SampleMultiplexing {
    Cellplex,
    FlexBarcode,
    Hashtag,
    OnChipMultiplexing,
    Singleplex,
}

#[db_insertion]
#[cfg_attr(feature = "app", derive(diesel::Selectable, diesel::Queryable))]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::library_type_specification))]
pub struct LibraryTypeSpecification {
    #[builder(default)]
    #[serde(default)]
    pub assay_id: Uuid,
    pub library_type: LibraryType,
    pub index_kit: String,
    #[garde(range(min = 0.0))]
    #[cfg_attr(feature = "app", diesel(column_name = cdna_volume_l))]
    pub cdna_volume_µl: f32,
    #[garde(range(min = 0.0))]
    #[cfg_attr(feature = "app", diesel(column_name = library_volume_l))]
    pub library_volume_µl: f32,
}

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::tenx_assay))]
pub struct NewChromiumAssay {
    #[cfg_attr(feature = "app", diesel(embed))]
    pub inner: NewTenxAssayCommon,
    #[garde(dive, length(min = 1))]
    pub library_types: Vec<LibraryType>,
    #[garde(dive)]
    pub sample_multiplexing: SampleMultiplexing,
    #[garde(dive)]
    pub chromium_chip: ValidString,
    #[garde(dive)]
    pub cmdline: ValidString,
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
    pub library_type_specifications: Vec<LibraryTypeSpecification>,
}
