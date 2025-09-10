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

#[cfg(target_arch = "wasm32")]
#[derive(Deserialize, Clone, Debug, Serialize, Default, PartialEq)]
pub struct AnyValue(#[serde(with = "serde_wasm_bindgen::preserve")] JsValue);

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
impl AnyValue {
    #[must_use]
    pub fn is_string(&self) -> bool {
        self.0.is_string()
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
        prelude::*,
        types::{PyBool, PyDict, PyDictMethods, PyFloat, PyList, PyNone, PyString},
    };
    use serde_json::Value as JsonValue;

    use super::AnyValue;

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
}

#[cfg(target_arch = "wasm32")]
mod wasm32 {
    use wasm_bindgen::{
        JsValue,
        convert::{IntoWasmAbi, OptionIntoWasmAbi},
        describe::WasmDescribe,
    };

    use super::AnyValue;

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
            <bool as OptionIntoWasmAbi>::none()
        }
    }
}
