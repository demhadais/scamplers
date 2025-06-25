#[cfg(feature = "backend")]
use scamplers_macros::backend_db_enum;

#[cfg_attr(feature = "backend", backend_db_enum)]
pub enum MassUnit {
    #[cfg_attr(feature = "backend", serde(rename = "ng"))]
    Nanogram,
    #[cfg_attr(feature = "backend", serde(rename = "pg"))]
    Picogram,
}

#[cfg_attr(feature = "backend", backend_db_enum)]
pub enum VolumeUnit {
    #[cfg_attr(feature = "backend", serde(rename = "µl"))]
    Microliter,
    #[cfg_attr(feature = "backend", serde(rename = "ml"))]
    Millliter,
}

#[cfg_attr(feature = "backend", backend_db_enum)]
pub enum LengthUnit {
    #[cfg_attr(feature = "backend", serde(rename = "µm"))]
    Micrometer,
}
