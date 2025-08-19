use std::fmt::Display;

use uuid::Uuid;

pub trait AsIlike: Display {
    fn as_ilike(&self) -> String {
        format!("%{self}%")
    }
}
impl<T> AsIlike for T where T: Display {}

#[macro_export]
macro_rules! init_stmt {
    (stmt = $stmt_base:expr, query = $query:expr, output_type = $output_struct:ident, orderby_spec = {$($enum_variant:path => $db_col:expr),*}) => {
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
    ($stmt:expr, filters = {$($col:expr => $values:expr),*}) => {{
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
    ($stmt:expr, filters = {$($col:expr => $value:expr),*}) => {{
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
    ($stmt:expr, filters = {$($col:expr => $string:expr),*}) => {{
        use $crate::db::util::AsIlike;
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
    {$name:ident { $($variant:ident),* }, default = $default:ident} => {
        #[derive(Clone, Debug, PartialEq, ::serde::Serialize, ::serde::Deserialize, ::strum::EnumString, ::strum::Display, ::valuable::Valuable)]
        #[serde(tag = "field", rename_all = "snake_case")]
        #[strum(serialize_all = "snake_case")]
        pub enum $name {
            $(
                $variant {
                    descending: bool
                },
            )*
        }

        impl Default for $name {
            fn default() -> Self {
                Self::$default { descending: false }
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
        impl From<$crate::db::models::WasmPythonOrderBy> for $name {
            fn from(wasm: $crate::db::models::WasmPythonOrderBy) -> Self {
                use wasm_bindgen::prelude::*;

                Self::new(&wasm.field, wasm.descending).unwrap_throw()
            }
        }

        #[cfg(feature = "python")]
        impl From<$crate::db::models::WasmPythonOrderBy> for $name {
            fn from(py: $crate::db::models::WasmPythonOrderBy) -> Self {
                // TODO: bad unwrap
                Self::new(&py.field, py.descending).unwrap()
            }
        }

        #[cfg(any(target_arch = "wasm32", feature = "python"))]
        impl From<$name> for $crate::db::models::WasmPythonOrderBy {
            fn from(order_by: $name) -> $crate::db::models::WasmPythonOrderBy {
                $crate::db::models::WasmPythonOrderBy {
                    field: order_by.to_string(),
                    descending: order_by.descending()
                }
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
            pub fn get_order_by(&self) -> Vec<$crate::db::models::WasmPythonOrderBy> {
                self.order_by.clone().into_iter().map(Into::into).collect()
            }

            #[wasm_bindgen(setter)]
            pub fn set_order_by(&mut self, orderings: Vec<$crate::db::models::WasmPythonOrderBy>) {
                orderings.into_iter().map(From::from).collect();
            }
        }
    };
}

#[macro_export]
macro_rules! uuid_newtype {
    ($name:ident) => {
        #[derive(Clone, Copy, serde::Deserialize, serde::Serialize, valuable::Valuable)]
        #[cfg_attr(feature = "python", derive(pyo3::IntoPyObject, pyo3::FromPyObject))]
        #[cfg_attr(feature = "python", pyo3(transparent))]
        #[serde(transparent)]
        #[valuable(transparent)]
        pub struct $name(pub uuid::Uuid);

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

        #[cfg(feature = "python")]
        impl pyo3_stub_gen::PyStubType for $name {
            fn type_output() -> pyo3_stub_gen::TypeInfo {
                pyo3_stub_gen::TypeInfo::with_module("uuid.UUID", "uuid".into())
            }
        }
    };
}

#[macro_export]
macro_rules! impl_id_db_operation {
    (id_type = $type_:ty,delegate_to = $delegated:ident,returns = $ret:ty) => {
        impl $crate::db::DbOperation<$ret> for $type_ {
            fn execute(
                self,
                db_conn: &mut diesel::PgConnection,
            ) -> $crate::result::ScamplersResult<$ret> {
                let results = $delegated::builder().ids(self).build().execute(db_conn)?;

                return Ok(results.into_iter().next().ok_or(
                    $crate::result::ResourceNotFoundError::builder()
                        .requested_resource_id(self)
                        .build(),
                )?);
            }
        }
    };
}

#[macro_export]
macro_rules! group_otm_children {
    (parents = $parents:expr,children = $children:expr) => {
        $children.grouped_by(&$parents).into_iter()
    };
}

#[macro_export]
macro_rules! group_mtm_children {
    (parents = $parents:expr,children = $children:expr) => {
        $children
            .grouped_by(&$parents)
            .into_iter()
            .map(|map_children_tuples| {
                map_children_tuples
                    .into_iter()
                    .map(|(_, child)| child)
                    .collect()
            })
    };
}

#[macro_export]
macro_rules! attach_children_to_parents {
    (parents = $parents:expr, children = [$($children:expr),*], transform_fn = $transform_fn:expr) => {{
        let parents = $parents.into_iter();

        $(
            let parents = parents.zip($children);
        )*

        parents.map($transform_fn).collect()
    }};
}

#[macro_export]
macro_rules! impl_constrained_py_setter {
    {$struct_name:ident::$setter_name:ident($field_type:ty) = $valid_value:expr} => {
        #[cfg(feature = "python")]
        #[pyo3_stub_gen::derive::gen_stub_pymethods]
        #[pymethods]
        impl $struct_name {
            #[setter]
            fn $setter_name(&mut self, value: $field_type) -> PyResult<()> {
                use pyo3::exceptions::PyValueError;

                if value != $valid_value {
                    return Err(PyValueError::new_err(format!(
                        "field can only be {}",
                        $valid_value
                    )));
                }

                Ok(())
            }
        }
    };
}

pub trait SetParentId {
    fn parent_id_mut(&mut self) -> &mut Uuid;

    fn set_parent_id(&mut self, id: Uuid) {
        let entity_id = self.parent_id_mut();
        *entity_id = id;
    }
}

pub trait ChildrenWithSelfId<Child: SetParentId> {
    fn children(&mut self) -> &mut [Child];

    fn children_with_self_id(&mut self, self_id: Uuid) -> &mut [Child] {
        let children = self.children();

        for child in &mut *children {
            child.set_parent_id(self_id);
        }

        children
    }
}
pub trait ManyToMany {
    fn new(parent_id: Uuid, child_id: Uuid) -> Self;
}

pub trait ManyToManyChildrenWithSelfId<Mapping: ManyToMany> {
    fn mtm_children(&self) -> &[Uuid];

    fn mtm_children_with_self_id(&self, self_id: Uuid) -> Vec<Mapping> {
        self.mtm_children()
            .iter()
            .map(|child_id| Mapping::new(self_id, *child_id))
            .collect()
    }
}
