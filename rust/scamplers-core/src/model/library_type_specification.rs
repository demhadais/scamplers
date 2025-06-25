#[cfg(feature = "backend")]
use {
    scamplers_macros::{backend_db_enum, backend_insertion},
    scamplers_schema::library_type_specification,
};

#[cfg_attr(feature = "backend", backend_db_enum)]
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

#[cfg_attr(
    feature = "backend",
    derive(Clone),
    backend_insertion(library_type_specification)
)]
pub struct NewLibraryTypeSpecification {
    chemistry: String,
    library_type: LibraryType,
    index_kit: String,
    #[cfg_attr(feature = "backend", garde(range(min = 0.0)), diesel(column_name = cdna_volume_l))]
    cdna_volume_µl: f32,
    #[cfg_attr(feature = "backend", garde(range(min = 0.0)), diesel(column_name = library_volume_l))]
    library_volume_µl: f32,
}
