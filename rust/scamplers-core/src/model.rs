#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{base_api_model, base_api_model_with_default};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub mod chemistry;
pub mod chromium_run;
pub mod dataset;
pub mod index_sets;
pub mod institution;
pub mod lab;
pub mod library_type_specification;
pub mod nucleic_acid;
pub mod person;
pub mod sequencing_run;
pub mod specimen;
pub mod suspension;
pub mod units;

#[cfg_attr(feature = "python", pyclass(get_all, set_all))]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
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

#[base_api_model_with_default]
#[derive(PartialEq)]
pub struct SortBy<C>
where
    C: valuable::Valuable + Default,
{
    pub by: C,
    pub descending: bool,
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

impl<C> From<SortBy<C>> for SortByGroup<C>
where
    C: valuable::Valuable + Default,
{
    fn from(value: SortBy<C>) -> Self {
        Self(vec![value])
    }
}

impl<C> From<(C, bool)> for SortBy<C>
where
    C: valuable::Valuable + Default,
{
    fn from((by, descending): (C, bool)) -> Self {
        Self { by, descending }
    }
}

impl<C> From<(C, bool)> for SortByGroup<C>
where
    C: valuable::Valuable + Default,
{
    fn from(value: (C, bool)) -> Self {
        SortByGroup(vec![value.into()])
    }
}

impl<C> SortByGroup<C>
where
    C: valuable::Valuable + Default,
{
    #[must_use]
    pub fn as_slice(&self) -> &[SortBy<C>] {
        self.0.as_slice()
    }

    pub fn push(&mut self, value: SortBy<C>) {
        self.0.push(value);
    }
}
