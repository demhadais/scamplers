use default_vec::DefaultVec;
use macro_attributes::{base_model, base_model_default};

#[base_model_default]
pub struct OrderBy<O>
where
    O: Default,
{
    pub field: O,
    pub descending: bool,
}

#[base_model]
#[derive(bon::Builder)]
#[serde(default)]
pub struct Query<F, O>
where
    F: Default,
    O: Default,
{
    #[serde(flatten)]
    #[builder(default)]
    pub filter: F,
    #[builder(default)]
    pub limit: i64,
    #[builder(default)]
    pub offset: i64,
    #[builder(default)]
    pub order_by: DefaultVec<OrderBy<O>>,
}

impl<F, O> Default for Query<F, O>
where
    F: Default,
    O: Default,
{
    fn default() -> Self {
        Query {
            filter: F::default(),
            limit: 500,
            offset: 0,
            order_by: DefaultVec::default(),
        }
    }
}
