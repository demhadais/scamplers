use scamplers_macros::db_enum;

#[db_enum]
pub enum MassUnit {
    #[cfg_attr(feature = "backend", serde(rename = "ng"))]
    Nanogram,
    #[cfg_attr(feature = "backend", serde(rename = "pg"))]
    Picogram,
}

#[db_enum]
pub enum VolumeUnit {
    #[cfg_attr(feature = "backend", serde(rename = "µl"))]
    Microliter,
    #[cfg_attr(feature = "backend", serde(rename = "ml"))]
    Millliter,
}

#[db_enum]
pub enum LengthUnit {
    #[cfg_attr(feature = "backend", serde(rename = "µm"))]
    Micrometer,
}
