use scamplers_macros::{db_enum, db_insertion};
#[cfg(feature = "backend")]
use scamplers_schema::library_type_specification;

#[db_enum]
#[derive(Clone)]
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

#[db_insertion]
#[derive(Clone)]
#[cfg_attr(feature = "backend", diesel(table_name = library_type_specification))]
pub struct NewLibraryTypeSpecification {
    chemistry: String,
    library_type: LibraryType,
    index_kit: String,
    #[garde(range(min = 0.0))]
    #[cfg_attr(feature = "backend", diesel(column_name = cdna_volume_l))]
    cdna_volume_µl: f32,
    #[garde(range(min = 0.0))]
    #[cfg_attr(feature = "backend", diesel(column_name = library_volume_l))]
    library_volume_µl: f32,
}
