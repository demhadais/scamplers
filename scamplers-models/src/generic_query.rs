use default_vec::DefaultVec;
use macro_attributes::{base_model, base_model_default};

#[base_model_default]
pub struct OrderBy<O>
where
    O: Default,
{
    field: O,
    descending: bool,
}

impl<O> OrderBy<O>
where
    O: Default + Copy,
{
    pub fn field(&self) -> O {
        self.field
    }

    pub fn descending(&self) -> bool {
        self.descending
    }
}

#[base_model]
#[cfg_attr(feature = "builder", derive(bon::Builder))]
#[serde(default)]
pub struct Query<F, O>
where
    F: Default,
    O: Default,
{
    #[serde(flatten)]
    #[cfg_attr(feature = "builder", builder(default))]
    filter: F,
    #[cfg_attr(feature = "builder", builder(default = Query::<F, O>::default().limit))]
    limit: i64,
    #[cfg_attr(feature = "builder", builder(default))]
    offset: i64,
    #[cfg_attr(feature = "builder", builder(default))]
    order_by: DefaultVec<OrderBy<O>>,
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

impl<F, O> Query<F, O>
where
    F: Default,
    O: Default,
{
    pub fn filter(&self) -> &F {
        &self.filter
    }

    pub fn limit(&self) -> i64 {
        self.limit
    }

    pub fn offset(&self) -> i64 {
        self.offset
    }

    pub fn order_by(&self) -> &[OrderBy<O>] {
        self.order_by.as_ref()
    }
}
