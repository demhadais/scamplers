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

        #[cfg(any(target_arch = "wasm32", feature = "python"))]
        impl From<$name> for crate::db::models::WasmPythonOrderBy {
            fn from(order_by: $name) -> crate::db::models::WasmPythonOrderBy {
                crate::db::models::WasmPythonOrderBy {
                    field: order_by.to_string(),
                    descending: order_by.descending()
                }
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod wasm32 {
            use super::$name;

            use wasm_bindgen::{
                JsValue,
                convert::{
                    FromWasmAbi, IntoWasmAbi, OptionFromWasmAbi, OptionIntoWasmAbi, TryFromJsValue,
                    VectorFromWasmAbi, VectorIntoWasmAbi, js_value_vector_from_abi,
                    js_value_vector_into_abi,
                },
                describe::{WasmDescribe, WasmDescribeVector},
            };

            use crate::db::models::WasmPythonOrderBy;

            impl From<WasmPythonOrderBy> for $name {
                fn from(wasm: crate::db::models::WasmPythonOrderBy) -> Self {
                    use wasm_bindgen::prelude::*;

                    Self::new(&wasm.field, wasm.descending).unwrap_throw()
                }
            }

            // impl WasmDescribe for $name {
            //     fn describe() {
            //         WasmPythonOrderBy::describe();
            //     }
            // }

            // impl IntoWasmAbi for $name {
            //     type Abi = <WasmPythonOrderBy as IntoWasmAbi>::Abi;

            //     fn into_abi(self) -> Self::Abi {
            //         let as_wasm: WasmPythonOrderBy = self.into();
            //     }
            // }

            // impl FromWasmAbi for $name {
            //     type Abi = <Self as IntoWasmAbi>::Abi;

            //     unsafe fn from_abi(js: Self::Abi) -> Self {
            //         unsafe { WasmPythonOrderBy::from_abi(js).into() }
            //     }
            // }

            // impl WasmDescribeVector for $name {
            //     fn describe_vector() {
            //         Vec::<WasmPythonOrderBy>::describe();
            //     }
            // }

            // impl VectorIntoWasmAbi for $name {
            //     type Abi = <WasmPythonOrderBy as VectorIntoWasmAbi>::Abi;

            //     fn vector_into_abi(vector: Box<[Self]>) -> Self::Abi {
            //         js_value_vector_into_abi(vector)
            //     }
            // }

            // impl VectorFromWasmAbi for $name {
            //     type Abi = <WasmPythonOrderBy as VectorFromWasmAbi>::Abi;

            //     unsafe fn vector_from_abi(js: Self::Abi) -> Box<[Self]> {
            //         unsafe { js_value_vector_from_abi(js) }
            //     }
            // }
        }

        #[cfg(feature = "python")]
        mod python {
            use super::$name;

            use pyo3::prelude::*;

            use crate::db::models::WasmPythonOrderBy;

            impl From<WasmPythonOrderBy> for $name {
                fn from(py: WasmPythonOrderBy) -> Self {
                    // TODO: bad unwrap
                    Self::new(&py.field, py.descending).unwrap()
                }
            }

            impl pyo3::FromPyObject<'_> for $name {
                fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
                    Ok(crate::db::models::WasmPythonOrderBy::extract_bound(ob)?.into())
                }
            }

            impl<'py> pyo3::IntoPyObject<'py> for $name {
                type Error = <crate::db::models::WasmPythonOrderBy as pyo3::IntoPyObject<'py>>::Error;
                type Output = <crate::db::models::WasmPythonOrderBy as pyo3::IntoPyObject<'py>>::Output;
                type Target = <crate::db::models::WasmPythonOrderBy as pyo3::IntoPyObject<'py>>::Target;

                fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
                    let as_generic: crate::db::models::WasmPythonOrderBy = self.into();

                    as_generic.into_pyobject(py)
                }
            }

            impl pyo3_stub_gen::PyStubType for $name {
                fn type_output() -> pyo3_stub_gen::TypeInfo {
                    crate::db::models::WasmPythonOrderBy::type_output()
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
            pub fn get_order_by(&self) -> Vec<crate::db::models::WasmPythonOrderBy> {
                self.order_by.clone().into_iter().map(Into::into).collect()
            }

            #[wasm_bindgen(setter)]
            pub fn set_order_by(&mut self, orderings: Vec<crate::db::models::WasmPythonOrderBy>) {
                orderings.into_iter().map(From::from).collect();
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
    (id_type = $type_:ty, delegate_to = $delegated:ident, returns = $ret:ty) => {
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
macro_rules! attach_children_to_parents_mtm {
    (parents = $parents:expr, children = $children:expr, transform_fn = $parent_with_children:expr) => {{
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

#[macro_export]
macro_rules! attach_one_to_many_children_to_parents_otm {
    (parents = $parents:expr, children = $children:expr, transform_fn = $parent_with_children:expr) => {{
        let children = $children
            .grouped_by(&$parents)
            .into_iter()
            .map(|children| children.into_iter());

        $parents
            .into_iter()
            .zip(children)
            .map(|(parent, children)| $parent_with_children((parent, children.collect())))
            .collect()
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

    fn children_with_self_id(&mut self, self_id: Uuid) -> &[Child] {
        let children = self.children();

        for child in &mut *children {
            child.set_parent_id(self_id);
        }

        children
    }
}
