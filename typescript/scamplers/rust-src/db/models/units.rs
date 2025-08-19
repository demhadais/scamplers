use scamplers_macros::db_simple_enum;

#[db_simple_enum]
#[cfg_attr(feature = "python", pyo3(module = "scamplepy.common"))]
pub enum MassUnit {
    #[serde(rename = "ng")]
    Nanogram,
    #[serde(rename = "pg")]
    Picogram,
}

#[db_simple_enum]
#[cfg_attr(feature = "python", pyo3(module = "scamplepy.common"))]
pub enum VolumeUnit {
    #[serde(rename = "µl")]
    Microliter,
    #[serde(rename = "ml")]
    Millliter,
}

#[db_simple_enum]
#[cfg_attr(feature = "python", pyo3(module = "scamplepy.common"))]
pub enum LengthUnit {
    #[serde(rename = "µm")]
    Micrometer,
}
