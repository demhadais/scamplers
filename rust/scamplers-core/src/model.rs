use pyo3::pyclass;
use scamplers_macros::{base_api_model, base_api_model_with_default};

pub mod chemistry;
pub mod chromium_run;
pub mod index_sets;
pub mod institution;
pub mod lab;
pub mod library_type_specification;
pub mod person;
pub mod sequencing_run;
pub mod specimen;
pub mod suspension;
pub mod units;

#[pyo3::pyclass(get_all, set_all)]
#[wasm_bindgen::prelude::wasm_bindgen]
#[base_api_model]
pub struct Pagination {
    pub limit: i64,
    #[garde(range(min = 1))]
    pub offset: i64,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            limit: 500,
            offset: 0,
        }
    }
}

pub trait IsUpdate {
    fn is_update(&self) -> bool;
}

#[base_api_model_with_default]
#[derive(PartialEq)]
pub struct SortBy<C>
where
    C: valuable::Valuable + Default,
{
    by: C,
    descending: bool,
}

#[base_api_model]
#[serde(transparent)]
#[valuable(transparent)]
#[derive(PartialEq)]
pub struct SortByGroup<C>(Vec<SortBy<C>>)
where
    C: valuable::Valuable + Default;

impl<C> Default for SortByGroup<C>
where
    C: valuable::Valuable + Default,
{
    fn default() -> Self {
        Self(vec![SortBy::<C>::default()])
    }
}

impl<C> SortByGroup<C>
where
    C: valuable::Valuable + Default,
{
    pub fn as_slice(&self) -> &[SortBy<C>] {
        self.0.as_slice()
    }

    pub fn push(&mut self, value: SortBy<C>) {
        self.0.push(value);
    }
}
