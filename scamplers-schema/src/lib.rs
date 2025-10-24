mod schema;

pub use schema::*;

macro_rules! impl_partial_eq {
    ($($column:path),*) => {
        $(
            impl PartialEq for $column {
                fn eq(&self, _: &Self) -> bool {
                    true
                }
            }
        )*
    };
}

impl_partial_eq!(
    institution::id,
    institution::name,
    person::id,
    person::name,
    person::email,
    lab::id,
    lab::name
);
