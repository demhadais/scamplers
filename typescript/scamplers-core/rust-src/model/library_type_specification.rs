#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{db_enum, db_insertion};
#[cfg(feature = "backend")]
use scamplers_schema::library_type_specification;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
#[db_enum]
#[derive(PartialOrd, Eq, Ord)]
pub enum LibraryType {
    #[serde(rename = "Antibody Capture")]
    #[strum(serialize = "Antibody Capture")]
    AntibodyCapture,

    #[serde(rename = "Antigen Capture")]
    #[strum(serialize = "Antigen Capture")]
    AntigenCapture,

    #[serde(rename = "Chromatin Accessibility")]
    #[strum(serialize = "Chromatin Accessibility")]
    ChromatinAccessibility,

    #[serde(rename = "CRISPR Guide Capture")]
    #[strum(serialize = "CRISPR Guide Capture")]
    CrisprGuideCapture,

    Custom,

    #[serde(rename = "Gene Expression")]
    #[strum(serialize = "Gene Expression")]
    GeneExpression,

    #[serde(rename = "Multiplexing Capture")]
    #[strum(serialize = "Multiplexing Capture")]
    MultiplexingCapture,

    #[serde(rename = "VDJ")]
    #[strum(serialize = "VDJ")]
    Vdj,

    #[serde(rename = "VDJ-B")]
    #[strum(serialize = "VDJ-B")]
    VdjB,

    #[serde(rename = "VDJ-T")]
    #[strum(serialize = "VDJ-T")]
    VdjT,

    #[serde(rename = "VDJ-T-GD")]
    #[strum(serialize = "VDJ-T-GD")]
    VdjTGd,
}

#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(scamplers_macros::FromJson, scamplers_macros::ToJson)
)]
#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = library_type_specification))]
pub struct NewLibraryTypeSpecification {
    pub chemistry: String,
    pub library_type: LibraryType,
    pub index_kit: String,
    #[garde(range(min = 0.0))]
    #[cfg_attr(feature = "app", diesel(column_name = cdna_volume_l))]
    pub cdna_volume_µl: f32,
    #[garde(range(min = 0.0))]
    #[cfg_attr(feature = "app", diesel(column_name = library_volume_l))]
    pub library_volume_µl: f32,
}

#[cfg(feature = "python")]
#[pymethods]
impl NewLibraryTypeSpecification {
    #[new]
    #[pyo3(signature = (*, chemistry, library_type, index_kit, cdna_volume_µl, library_volume_µl))]
    fn new(
        chemistry: String,
        library_type: LibraryType,
        index_kit: String,
        cdna_volume_µl: f32,
        library_volume_µl: f32,
    ) -> Self {
        Self {
            chemistry,
            library_type,
            index_kit,
            cdna_volume_µl,
            library_volume_µl,
        }
    }
}
