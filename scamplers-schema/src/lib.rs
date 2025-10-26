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
    institutions::id,
    institutions::name,
    people::id,
    people::name,
    people::email,
    labs::id,
    labs::name
);
