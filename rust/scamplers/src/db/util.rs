pub trait AsIlike {
    fn as_ilike(&self) -> String;
}

impl AsIlike for &str {
    fn as_ilike(&self) -> String {
        format!("%{self}%")
    }
}

impl AsIlike for String {
    fn as_ilike(&self) -> String {
        self.as_str().as_ilike()
    }
}

#[macro_export]
macro_rules! init_stmt {
    (stmt = $stmt_base:expr, query = $query:expr, output = $output_struct:ident; $($enum_variant:path => $db_col:expr)*) => {
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
                        $enum_variant { .. } => stmt.then_order_by($db_col.asc())
                    )*
                }
            }
            stmt
        }
    };
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
        #[derive(serde::Deserialize)]
        #[serde(transparent)]
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

        impl From<&$name> for Vec<uuid::Uuid> {
            fn from(val: &$name) -> Vec<uuid::Uuid> {
                vec![val.0]
            }
        }
    };
}
