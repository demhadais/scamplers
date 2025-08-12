#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{db_enum, db_insertion};
#[cfg(feature = "app")]
use scamplers_schema::library_type_specification;

use crate::db::models::Jsonify;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
#[db_enum]
#[derive(PartialOrd, Eq, Ord)]
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
