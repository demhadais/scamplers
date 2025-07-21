#[cfg(feature = "backend")]
use diesel::{deserialize::FromSqlRow, expression::AsExpression, sql_types};
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

#[cfg(feature = "backend")]
#[derive(Eq, PartialEq, Debug, Deserialize, Serialize)]
#[serde(transparent)]
#[derive(FromSqlRow, AsExpression, Clone)]
#[diesel(sql_type = sql_types::Jsonb)]
pub struct AnyValue(serde_json::Value);

impl valuable::Valuable for AnyValue {
    fn as_value(&self) -> valuable::Value<'_> {
        "AnyValue".as_value()
    }

    fn visit(&self, visit: &mut dyn valuable::Visit) {
        "AnyValue".visit(visit);
    }
}

#[cfg(feature = "backend")]
mod backend {
    use std::fmt::Display;
    use {
        super::AnyValue,
        diesel::{
            backend::Backend,
            deserialize::FromSql,
            pg::Pg,
            serialize::{Output, ToSql},
            sql_types,
        },
    };

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

    impl Display for AnyValue {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl<T> From<T> for AnyValue
    where
        serde_json::Value: From<T>,
    {
        fn from(val: T) -> Self {
            Self(serde_json::Value::from(val))
        }
    }

    impl AnyValue {
        #[must_use]
        pub fn is_string(&self) -> bool {
            self.0.is_string()
        }
    }
}

#[cfg(feature = "python")]
#[derive(Deserialize, IntoPyObject, Debug)]
#[pyo3(transparent)]
pub struct AnyValue(#[serde(deserialize_with = "python::py_deserialize")] PyObject);

#[cfg(feature = "python")]
mod python {
    use pyo3::{
        BoundObject,
        prelude::*,
        types::{PyBool, PyDict, PyDictMethods, PyFloat, PyList, PyNone, PyString},
    };
    use serde::{Deserialize, Deserializer};
    use serde_json::Value as JsonValue;

    fn json_value_to_py_object(py: Python<'_>, json_value: JsonValue) -> PyObject {
        match json_value {
            JsonValue::Array(a) => {
                PyList::new(py, a.into_iter().map(|v| json_value_to_py_object(py, v)))
                    .unwrap()
                    .into_any()
                    .unbind()
            }
            JsonValue::Bool(b) => PyBool::new(py, b).into_any().unbind(),
            JsonValue::Null => PyNone::get(py).into_any().unbind(),
            JsonValue::Number(n) => PyFloat::new(py, n.as_f64().unwrap()).into_any().unbind(),
            JsonValue::Object(o) => {
                let dict = PyDict::new(py);
                for (key, val) in o {
                    dict.set_item(key, json_value_to_py_object(py, val))
                        .unwrap();
                }

                dict.into_any().unbind()
            }
            JsonValue::String(s) => PyString::new(py, &s).into_any().unbind(),
        }
    }

    pub fn py_deserialize<'de, D>(deserializer: D) -> Result<PyObject, D::Error>
    where
        D: Deserializer<'de>,
    {
        let json_value = serde_json::Value::deserialize(deserializer)?;

        Ok(Python::with_gil(|py| {
            json_value_to_py_object(py, json_value)
        }))
    }
}

#[cfg(target_arch = "wasm32")]
#[derive(Deserialize, Clone, Debug)]
pub struct AnyValue(#[serde(with = "serde_wasm_bindgen::preserve")] JsValue);

#[cfg(target_arch = "wasm32")]
mod wasm32 {
    use super::AnyValue;
    use std::str::FromStr;

    use wasm_bindgen::{
        JsValue,
        convert::{
            FromWasmAbi, IntoWasmAbi, OptionFromWasmAbi, OptionIntoWasmAbi, TryFromJsValue,
            VectorFromWasmAbi, VectorIntoWasmAbi, js_value_vector_from_abi,
            js_value_vector_into_abi,
        },
        describe::{WasmDescribe, WasmDescribeVector},
    };

    impl WasmDescribe for AnyValue {
        fn describe() {
            JsValue::describe();
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
            <JsValue as OptionIntoWasmAbi>::none()
        }
    }
}
