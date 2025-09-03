use scamplers_macros::db_simple_enum;

#[db_simple_enum]
#[cfg_attr(feature = "python", pyo3(module = "scamplepy.common"))]
pub enum MassUnit {
    #[serde(rename = "ng")]
    #[strum(serialize = "ng")]
    Nanogram,
    #[serde(rename = "pg")]
    #[strum(serialize = "pg")]
    Picogram,
}

#[db_simple_enum]
#[cfg_attr(feature = "python", pyo3(module = "scamplepy.common"))]
pub enum VolumeUnit {
    #[serde(rename = "µl")]
    #[strum(serialize = "µl")]
    Microliter,
    #[serde(rename = "ml")]
    #[strum(serialize = "ml")]
    Millliter,
}

#[db_simple_enum]
#[cfg_attr(feature = "python", pyo3(module = "scamplepy.common"))]
pub enum LengthUnit {
    #[serde(rename = "µm")]
    #[strum(serialize = "µm")]
    Micrometer,
}
