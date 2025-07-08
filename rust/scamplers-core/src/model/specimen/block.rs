use crate::model::specimen::common::NewCommitteeApproval;

use super::common::NewSpecimenCommon;
use getset::MutGetters;
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
#[derive(MutGetters)]
#[cfg_attr(feature = "backend", diesel(table_name = specimen))]
pub struct NewFixedBlock {
    #[serde(flatten)]
    #[garde(dive)]
    #[getset(get_mut)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    inner: NewSpecimenCommon,
    #[serde(skip)]
    type_: BlockType,
    embedded_in: FixedBlockEmbeddingMatrix,
    fixative: BlockFixative,
}

#[db_enum]
pub enum FrozenBlockEmbeddingMatrix {
    CarboxymethylCellulose,
    OptimalCuttingTemperatureCompound,
}

#[db_insertion]
#[derive(MutGetters)]
#[cfg_attr(feature = "backend", diesel(table_name = specimen))]
pub struct NewFrozenBlock {
    #[serde(flatten)]
    #[garde(dive)]
    #[getset(get_mut)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    inner: NewSpecimenCommon,
    #[serde(skip)]
    type_: BlockType,
    embedded_in: FrozenBlockEmbeddingMatrix,
    fixative: Option<BlockFixative>,
    #[garde(custom(super::common::is_true))]
    frozen: bool,
}

#[base_api_model]
#[serde(tag = "preservation")]
pub enum NewBlock {
    Fixed(#[garde(dive)] NewFixedBlock),
    Frozen(#[garde(dive)] NewFrozenBlock),
}
impl NewBlock {
    pub(super) fn inner_mut(&mut self) -> &mut NewSpecimenCommon {
        match self {
            Self::Fixed(b) => b.inner_mut(),
            Self::Frozen(b) => b.inner_mut(),
        }
    }

    pub(super) fn committee_approvals_mut(&mut self) -> &mut [NewCommitteeApproval] {
        self.inner_mut().committee_approvals_mut()
    }
}
