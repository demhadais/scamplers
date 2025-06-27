use derive_builder::UninitializedFieldError;
use scamplers_macros::{base_api_model, base_api_model_with_default};
use std::ops::Deref;

pub use institution::{Institution, InstitutionQuery, NewInstitution};

mod institution;
mod lab;
mod person;
mod specimen;
// pub mod chemistry;
// pub mod index_sets;
// pub mod institution;
// pub mod library_type_specification;
// pub mod person;
// pub mod sequencing_run;
// pub mod suspension;
// pub mod units;

#[cfg_attr(target_arch = "wasm32", ::wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Debug)]
pub struct BuilderError {
    message: String,
}

impl From<UninitializedFieldError> for BuilderError {
    fn from(value: UninitializedFieldError) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

#[base_api_model]
#[cfg_attr(target_arch = "wasm32", ::wasm_bindgen::prelude::wasm_bindgen)]
pub struct Pagination {
    limit: i64,
    #[garde(range(min = 1))]
    offset: i64,
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

impl<C> Deref for SortByGroup<C>
where
    C: valuable::Valuable + Default,
{
    type Target = [SortBy<C>];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

#[cfg(test)]
mod tests {
    use crate::model::SortBy;

    use super::SortByGroup;
    use scamplers_macros::{base_api_model_with_default, db_insertion, db_query};

    #[test]
    fn write_request_builder() {
        // This is just to get rid of IDE errors
        #[cfg(feature = "backend")]
        diesel::table! {
            write_structs(field) {
                field -> Text,
                optional_field -> Nullable<Text>
            }
        }

        #[db_insertion]
        #[derive(PartialEq)]
        struct WriteStruct {
            field: String,
            #[builder(default)]
            optional_field: Option<String>,
        }

        let built_struct = WriteStructBuilder::default()
            .field("field")
            .build()
            .expect("failed to build struct without setting optional field");

        assert_eq!(
            built_struct,
            WriteStruct {
                field: "field".to_string(),
                optional_field: None
            }
        );

        WriteStructBuilder::default().build().unwrap_err();
    }

    #[test]
    fn default_query_request() {
        #[base_api_model_with_default]
        #[derive(PartialEq)]
        pub enum OrdinalColumn {
            #[default]
            Column,
        }

        #[db_query]
        #[derive(PartialEq)]
        struct QueryStruct {
            optional_field: Option<String>,
            #[builder(setter(custom))]
            order_by: SortByGroup<OrdinalColumn>,
        }

        let built_struct = QueryStructBuilder::default()
            .build()
            .expect("failed to build default `QueryStruct`");

        assert_eq!(
            built_struct,
            QueryStruct {
                optional_field: None,
                order_by: SortByGroup(vec![SortBy {
                    by: OrdinalColumn::Column,
                    descending: false
                }])
            }
        );
    }
}
