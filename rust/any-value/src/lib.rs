#[cfg(not(target_arch = "wasm32"))]
use std::fmt::Display;

#[cfg(feature = "app")]
use diesel::{deserialize::FromSqlRow, expression::AsExpression, sql_types};
use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Eq, PartialEq, Debug, Deserialize, Serialize, Clone, Default)]
#[cfg_attr(feature = "app", derive(FromSqlRow, AsExpression))]
#[serde(transparent)]
#[cfg_attr(feature = "app", diesel(sql_type = sql_types::Jsonb))]
pub struct AnyValue(serde_json::Value);

#[macro_export]
macro_rules! any_value {
    ($($json:tt)+) => {
        $crate::AnyValue::from(serde_json::json!($($json)+))
    };
}

#[cfg(target_arch = "wasm32")]
#[derive(Deserialize, Clone, Debug, Serialize, Default, PartialEq)]
pub struct AnyValue(#[serde(with = "serde_wasm_bindgen::preserve")] JsValue);

#[cfg(not(target_arch = "wasm32"))]
impl<'a> std::ops::Index<&'a str> for AnyValue {
    type Output = serde_json::Value;

    fn index(&self, index: &'a str) -> &Self::Output {
        &self.0[index]
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Display for AnyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<T> From<T> for AnyValue
where
    serde_json::Value: From<T>,
{
    fn from(val: T) -> Self {
        Self(serde_json::Value::from(val))
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub trait WithSnakeCaseKeys {
    #[must_use]
    fn with_snake_case_keys(self) -> Self;
}

#[cfg(not(target_arch = "wasm32"))]
impl WithSnakeCaseKeys for serde_json::Value {
    fn with_snake_case_keys(self) -> Self {
        use heck::ToSnekCase;

        match self {
            Self::Object(old_obj) => {
                let mut new_obj = serde_json::Map::with_capacity(old_obj.len());
                for (key, val) in old_obj {
                    new_obj.insert(key.to_snek_case(), val.with_snake_case_keys());
                }
                Self::Object(new_obj)
            }
            value => value,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl AnyValue {
    #[must_use]
    pub fn is_string(&self) -> bool {
        self.0.is_string()
    }

    #[must_use]
    pub fn get<I: serde_json::value::Index>(&self, index: I) -> Option<&serde_json::Value> {
        self.0.get(index)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl WithSnakeCaseKeys for AnyValue {
    fn with_snake_case_keys(self) -> Self {
        Self(self.0.with_snake_case_keys())
    }
}

impl valuable::Valuable for AnyValue {
    fn as_value(&self) -> valuable::Value<'_> {
        "AnyValue".as_value()
    }

    fn visit(&self, visit: &mut dyn valuable::Visit) {
        "AnyValue".visit(visit);
    }
}

#[cfg(feature = "app")]
mod app {
    use diesel::{
        backend::Backend,
        deserialize::FromSql,
        pg::Pg,
        serialize::{Output, ToSql},
        sql_types,
    };

    use super::AnyValue;

    impl FromSql<sql_types::Jsonb, Pg> for AnyValue {
        fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
            let json = <serde_json::Value as FromSql<sql_types::Jsonb, Pg>>::from_sql(bytes)?;
            Ok(Self(json))
        }
    }

    impl ToSql<sql_types::Jsonb, Pg> for AnyValue {
        fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
            let Self(inner) = self;
            <serde_json::Value as ToSql<sql_types::Jsonb, Pg>>::to_sql(inner, out)
        }
    }
}

#[cfg(feature = "python")]
mod python {
    use pyo3::{
        BoundObject,
        exceptions::PyValueError,
        prelude::*,
        types::{PyBool, PyDict, PyDictMethods, PyFloat, PyInt, PyList, PyNone, PyString},
    };
    use serde_json::Value as JsonValue;

    use super::AnyValue;

    impl FromPyObject<'_> for AnyValue {
        fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
            if ob.is_instance_of::<PyList>() {
                let py_list = ob.downcast::<PyList>()?;
                let rust_list: Vec<Self> = py_list
                    .iter()
                    .map(|item| Self::extract_bound(&item))
                    .collect::<Result<Vec<_>, _>>()?;

                return Ok(Self(serde_json::Value::Array(
                    rust_list.into_iter().map(|Self(item)| item).collect(),
                )));
            }

            if ob.is_instance_of::<PyDict>() {
                let py_dict = ob.downcast::<PyDict>()?;
                let mut rust_map = serde_json::Map::with_capacity(py_dict.len());

                for (k, v) in py_dict {
                    let k: String = k.extract()?;
                    rust_map.insert(k, AnyValue::extract_bound(&v)?.0);
                }

                return Ok(Self(serde_json::Value::Object(rust_map)));
            }

            if ob.is_instance_of::<PyInt>() {
                let rust_num: i64 = ob.extract()?;

                return Ok(Self(serde_json::Value::Number(serde_json::Number::from(
                    rust_num,
                ))));
            }

            if ob.is_instance_of::<PyFloat>() {
                let rust_num: f64 = ob.extract()?;

                return Ok(Self(serde_json::Value::Number(
                    serde_json::Number::from_f64(rust_num).unwrap(),
                )));
            }

            if ob.is_instance_of::<PyString>() {
                let rust_str: String = ob.extract()?;

                return Ok(Self(serde_json::Value::String(rust_str)));
            }

            if ob.is_instance_of::<PyNone>() {
                return Ok(Self(serde_json::Value::Null));
            }

            Err(PyValueError::new_err(format!(
                "type {} is not supported",
                ob.get_type()
            )))
        }
    }

    impl<'py> IntoPyObject<'py> for AnyValue {
        type Error = <Py<PyAny> as IntoPyObject<'py>>::Error;
        type Output = <Py<PyAny> as IntoPyObject<'py>>::Output;
        type Target = <Py<PyAny> as IntoPyObject<'py>>::Target;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let ret = match self.0 {
                JsonValue::Array(a) => PyList::new(
                    py,
                    a.into_iter().map(|v| Self(v).into_pyobject(py).unwrap()),
                )
                .unwrap()
                .into_any(),
                JsonValue::Bool(b) => PyBool::new(py, b).into_any().into_bound(),
                JsonValue::Null => PyNone::get(py).into_any().into_bound(),
                JsonValue::Number(n) => PyFloat::new(py, n.as_f64().unwrap()).into_any(),
                JsonValue::Object(o) => {
                    let dict = PyDict::new(py);
                    for (key, val) in o {
                        dict.set_item(key, Self(val).into_pyobject(py).unwrap())
                            .unwrap();
                    }

                    dict.into_any()
                }
                JsonValue::String(s) => PyString::new(py, &s).into_any(),
            };

            Ok(ret)
        }
    }

    impl pyo3_stub_gen::PyStubType for AnyValue {
        fn type_output() -> pyo3_stub_gen::TypeInfo {
            pyo3_stub_gen::TypeInfo::with_module("typing", "Any".into())
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm32 {
    use wasm_bindgen::{
        JsCast, JsValue,
        convert::{FromWasmAbi, IntoWasmAbi, OptionFromWasmAbi, OptionIntoWasmAbi},
        describe::WasmDescribe,
    };

    use super::AnyValue;

    impl WasmDescribe for AnyValue {
        fn describe() {
            JsValue::describe();
        }
    }

    impl AsRef<JsValue> for AnyValue {
        fn as_ref(&self) -> &JsValue {
            &self.0
        }
    }

    impl From<AnyValue> for JsValue {
        fn from(val: AnyValue) -> Self {
            val.0
        }
    }

    impl JsCast for AnyValue {
        fn instanceof(val: &JsValue) -> bool {
            JsValue::instanceof(val)
        }

        fn unchecked_from_js(val: JsValue) -> Self {
            Self(JsValue::unchecked_from_js(val))
        }

        fn unchecked_from_js_ref(_val: &JsValue) -> &Self {
            panic!("there's actually no way to do this")
        }
    }

    impl IntoWasmAbi for AnyValue {
        type Abi = <JsValue as IntoWasmAbi>::Abi;

        fn into_abi(self) -> Self::Abi {
            self.0.into_abi()
        }
    }

    impl OptionIntoWasmAbi for AnyValue {
        fn none() -> Self::Abi {
            <bool as OptionIntoWasmAbi>::none()
        }
    }

    impl FromWasmAbi for AnyValue {
        type Abi = <Self as IntoWasmAbi>::Abi;

        unsafe fn from_abi(js: Self::Abi) -> Self {
            unsafe { Self(JsValue::from_abi(js)) }
        }
    }

    impl OptionFromWasmAbi for AnyValue {
        fn is_none(_abi: &Self::Abi) -> bool {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::WithSnakeCaseKeys;

    #[rstest]
    fn snake_case() {
        let original = any_value!(
            {
                "My String": "A Titlecased String",
                "My Number": 100.0,
                "My Map": {"CamelcaseKey": "a value"},
                "My Array": ["Another String"]
            }
        );

        let snake_cased = original.with_snake_case_keys();

        let expected = any_value!(
            {
                "my_string": "A Titlecased String",
                "my_number": 100.0,
                "my_map": {"camelcase_key": "a value"},
                "my_array": ["Another String"]
            }
        );
        let expected = expected;

        assert_eq!(snake_cased, expected);
    }
}
