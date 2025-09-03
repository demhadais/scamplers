#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{db_insertion, db_simple_enum};

#[cfg(feature = "app")]
mod create;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
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

#[db_insertion]
#[cfg_attr(feature = "app", derive(diesel::Selectable, diesel::Queryable))]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::library_type_specification))]
pub struct LibraryTypeSpecification {
    #[builder(default)]
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

#[must_use]
pub fn chromatin_accessibility_library_specification() -> LibraryTypeSpecification {
    // See https://cdn.10xgenomics.com/image/upload/v1728078402/support-documents/CG000496_Chromium_NextGEM_SingleCell_ATAC_ReagentKits_v2_UserGuide_RevC.pdf.pdf, page 37, bullet 'n' and page 41 bullet 'q'
    LibraryTypeSpecification::builder()
        .library_type(LibraryType::ChromatinAccessibility)
        .index_kit("NA")
        .cdna_volume_µl(40.0)
        .library_volume_µl(20.0)
        .build()
}

#[cfg(feature = "python")]
#[pymethods]
impl LibraryTypeSpecification {
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
