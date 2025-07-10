use super::common::NewSpecimenCommon;
use scamplers_macros::{base_api_model, db_enum, db_insertion};
#[cfg(feature = "backend")]
use scamplers_schema::specimen;

#[db_enum]
#[derive(Default)]
pub enum BlockType {
    #[default]
    Block,
}

#[db_enum]
pub enum FixedBlockEmbeddingMatrix {
    Paraffin,
}

#[db_enum]
pub enum BlockFixative {
    FormaldehydeDerivative,
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = specimen))]
pub struct NewFixedBlock {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub inner: NewSpecimenCommon,
    #[serde(skip)]
    pub type_: BlockType,
    pub embedded_in: FixedBlockEmbeddingMatrix,
    pub fixative: BlockFixative,
}

#[db_enum]
pub enum FrozenBlockEmbeddingMatrix {
    CarboxymethylCellulose,
    OptimalCuttingTemperatureCompound,
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = specimen))]
pub struct NewFrozenBlock {
    #[serde(flatten)]
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub inner: NewSpecimenCommon,
    #[serde(skip)]
    pub type_: BlockType,
    pub embedded_in: FrozenBlockEmbeddingMatrix,
    pub fixative: Option<BlockFixative>,
    #[garde(custom(super::common::is_true))]
    pub frozen: bool,
}

#[base_api_model]
#[serde(tag = "preservation")]
pub enum NewBlock {
    Fixed(#[garde(dive)] NewFixedBlock),
    Frozen(#[garde(dive)] NewFrozenBlock),
}
