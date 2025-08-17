use std::fmt::Display;

pub trait AsIlike: Display {
    fn as_ilike(&self) -> String {
        format!("%{self}%")
    }
}
impl<T> AsIlike for T where T: Display {}

#[macro_export]
macro_rules! init_stmt {
    (stmt = $stmt_base:expr, query = $query:expr, output = $output_struct:ident; $($enum_variant:path => $db_col:expr),*) => {
        {
            let mut stmt = $stmt_base
            .select($output_struct::as_select())
            .limit($query.pagination.limit)
            .offset($query.pagination.offset)
            .into_boxed();

            for ordering in &$query.order_by {
                stmt = match ordering {
                    $(
                        $enum_variant { descending: true } => stmt.then_order_by($db_col.desc()),
                        $enum_variant { .. } => stmt.then_order_by($db_col.asc()),
                    )*
                }
            }
            stmt
        }
    };
}

#[macro_export]
macro_rules! apply_eq_any_filters {
    ($stmt:expr; $($col:expr, $values:expr);*) => {{
        $(
            if !$values.is_empty() {
                $stmt = $stmt.filter($col.eq_any($values));
            }
        )*

        $stmt
    }};
}

#[macro_export]
macro_rules! apply_eq_filters {
    ($stmt:expr; $($col:expr, $value:expr);*) => {{
        $(
            if let Some(value) = $value {
                $stmt = $stmt.filter($col.eq(value));
            }
        )*

        $stmt
    }};
}

#[macro_export]
macro_rules! apply_ilike_filters {
    ($stmt:expr; $($col:expr, $string:expr);*) => {{
        use crate::db::util::AsIlike;
        $(
            if let Some(string) = $string {
                $stmt = $stmt.filter($col.ilike(string.as_ilike()));
            }
        )*

        $stmt
    }};
}

#[macro_export]
macro_rules! define_ordering_enum {
    ($name:ident; $($variant:ident),*; default = $default_variant:ident; $($enum_attribute:meta),*) => {
        #[derive(Clone, Debug, PartialEq, ::serde::Serialize, ::serde::Deserialize, ::strum::EnumString, ::strum::Display, ::valuable::Valuable)]
        #[serde(tag = "field", rename_all = "snake_case")]
        #[strum(serialize_all = "snake_case")]
        $(#[$enum_attribute])*
        pub enum $name {
            $(
                $variant {
                    descending: bool
                },
            )*
        }

        impl Default for $name {
            fn default() -> Self {
                Self::$default_variant {
                    descending: false
                }
            }
        }

        #[allow(dead_code)]
        impl $name {
            fn new(field: &str, descending: bool) -> Result<Self, ::strum::ParseError> {
                use std::str::FromStr;

                let mut orderby = Self::from_str(field)?;
                orderby.set_descending(descending);

                Ok(orderby)
            }

            fn set_descending(&mut self, descending: bool) {
                match self {
                    $( Self::$variant{ descending: current, ..} )|* => { *current = descending; }
                }
            }

            fn descending(&self) -> bool {
                match self {
                    $( Self::$variant{ descending, ..} )|* => *descending
                }
            }
        }

        #[cfg(target_arch = "wasm32")]
        impl From<crate::db::models::WasmPythonOrderBy> for $name {
            fn from(wasm: crate::db::models::WasmPythonOrderBy) -> Self {
                use wasm_bindgen::prelude::*;

                Self::new(&wasm.field, wasm.descending).unwrap_throw()
            }
        }

        #[cfg(feature = "python")]
        impl From<crate::db::models::WasmPythonOrderBy> for $name {
            fn from(py: crate::db::models::WasmPythonOrderBy) -> Self {
                // TODO: bad unwrap
                Self::new(&py.field, py.descending).unwrap()
            }
        }

        #[cfg(any(target_arch = "wasm32", feature = "python"))]
        impl From<$name> for crate::db::models::WasmPythonOrderBy {
            fn from(order_by: $name) -> crate::db::models::WasmPythonOrderBy {
                crate::db::models::WasmPythonOrderBy {
                    field: order_by.to_string(),
                    descending: order_by.descending()
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_order_by {
    ($query_struct:ident) => {
        #[cfg(any(target_arch = "wasm32", feature = "python"))]
        impl $query_struct {
            fn order_by_inner(&self) -> Vec<crate::db::models::WasmPythonOrderBy> {
                self.order_by.clone().into_iter().map(Into::into).collect()
            }

            fn set_order_by_inner(&mut self, orderings: Vec<crate::db::models::WasmPythonOrderBy>) {
                self.order_by = orderings.into_iter().map(From::from).collect();
            }
        }
    };
}

#[macro_export]
macro_rules! impl_wasm_order_by {
    ($query_struct:ident) => {
        #[cfg(target_arch = "wasm32")]
        #[::wasm_bindgen::prelude::wasm_bindgen]
        impl $query_struct {
            #[wasm_bindgen(getter)]
            pub fn get_order_by(&self) -> Vec<crate::db::models::WasmPythonOrderBy> {
                self.order_by_inner()
            }

            #[wasm_bindgen(setter)]
            pub fn set_order_by(&mut self, orderings: Vec<crate::db::models::WasmPythonOrderBy>) {
                self.set_order_by_inner(orderings)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_python_order_by {
    ($query_struct:ident) => {
        #[cfg(feature = "python")]
        #[::pyo3_stub_gen::derive::gen_stub_pymethods]
        #[::pyo3::pymethods]
        impl $query_struct {
            #[getter]
            pub fn order_by(&self) -> Vec<crate::db::models::WasmPythonOrderBy> {
                self.order_by_inner()
            }

            #[setter]
            pub fn set_order_by(&mut self, orderings: Vec<crate::db::models::WasmPythonOrderBy>) {
                self.set_order_by_inner(orderings)
            }
        }
    };
}

#[macro_export]
macro_rules! uuid_newtype {
    ($name:ident) => {
        #[cfg_attr(feature = "python", pyo3::pyclass)]
        #[derive(Clone, Copy, serde::Deserialize, serde::Serialize, valuable::Valuable)]
        #[serde(transparent)]
        #[valuable(transparent)]
        pub struct $name(uuid::Uuid);

        impl AsRef<uuid::Uuid> for $name {
            fn as_ref(&self) -> &uuid::Uuid {
                &self.0
            }
        }

        impl From<$name> for uuid::Uuid {
            fn from(val: $name) -> uuid::Uuid {
                val.0
            }
        }

        impl From<$name> for Vec<uuid::Uuid> {
            fn from(val: $name) -> Vec<uuid::Uuid> {
                vec![val.0]
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_id_db_operation {
    ($type_:ty, $delegated:ident, $ret:ty) => {
        impl crate::db::DbOperation<$ret> for $type_ {
            fn execute(
                self,
                db_conn: &mut diesel::PgConnection,
            ) -> crate::result::ScamplersResult<$ret> {
                let results = $delegated::builder().ids(self).build().execute(db_conn)?;

                return Ok(results.into_iter().next().ok_or(
                    crate::result::ResourceNotFoundError::builder()
                        .requested_resource_id(self)
                        .build(),
                )?);
            }
        }
    };
}

#[macro_export]
macro_rules! attach_children_to_parents {
    ($parents:expr, $children:expr, $parent_with_children:expr) => {{
        let children = $children
            .grouped_by(&$parents)
            .into_iter()
            .map(|map_children_tuples| map_children_tuples.into_iter().map(|(_, child)| child));

        $parents
            .into_iter()
            .zip(children)
            .map(|(parent, children)| $parent_with_children((parent, children.collect())))
            .collect()
    }};
}
