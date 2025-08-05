use std::fmt::Display;

#[cfg(feature = "backend")]
use diesel::{deserialize::FromSqlRow, expression::AsExpression, sql_types};
#[cfg(feature = "python")]
use pyo3::prelude::*;

const OFFSET_DATETIME_AS_VALUE: &str = "OffsetDateTime";

#[derive(
    Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Debug, Hash, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "python", derive(IntoPyObject, FromPyObject))]
#[cfg_attr(feature = "backend", derive(FromSqlRow, AsExpression))]
#[cfg_attr(feature = "python", pyo3(transparent))]
#[cfg_attr(feature = "backend", diesel(sql_type = sql_types::Timestamptz))]
#[serde(transparent)]
pub struct OffsetDateTime(#[serde(with = "_time::serde::iso8601")] _time::OffsetDateTime);
impl OffsetDateTime {
    #[must_use]
    pub fn now_utc() -> Self {
        Self(_time::OffsetDateTime::now_utc())
    }

    pub fn from_unix_timestamp(timestamp: i64) -> Result<Self, _time::error::ComponentRange> {
        _time::OffsetDateTime::from_unix_timestamp(timestamp).map(Self)
    }
}

impl Display for OffsetDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(inner) = self;
        inner.fmt(f)
    }
}

impl valuable::Valuable for OffsetDateTime {
    fn as_value(&self) -> valuable::Value<'_> {
        OFFSET_DATETIME_AS_VALUE.as_value()
    }

    fn visit(&self, visit: &mut dyn valuable::Visit) {
        OFFSET_DATETIME_AS_VALUE.visit(visit);
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm32 {
    use wasm_bindgen::{
        JsValue,
        convert::{
            FromWasmAbi, IntoWasmAbi, OptionFromWasmAbi, OptionIntoWasmAbi, TryFromJsValue,
            VectorFromWasmAbi, VectorIntoWasmAbi, js_value_vector_from_abi,
            js_value_vector_into_abi,
        },
        describe::{WasmDescribe, WasmDescribeVector},
    };

    use super::OffsetDateTime;

    impl WasmDescribe for OffsetDateTime {
        fn describe() {
            js_sys::Date::describe();
        }
    }

    impl From<OffsetDateTime> for JsValue {
        fn from(val: OffsetDateTime) -> Self {
            js_sys::Date::from(val.0).into()
        }
    }

    impl TryFromJsValue for OffsetDateTime {
        type Error = _time::Error;

        fn try_from_js_value(value: JsValue) -> Result<Self, Self::Error> {
            Ok(Self(js_sys::Date::new(&value).into()))
        }
    }

    impl IntoWasmAbi for OffsetDateTime {
        type Abi = <js_sys::Date as IntoWasmAbi>::Abi;

        fn into_abi(self) -> Self::Abi {
            js_sys::Date::from(self.0).into_abi()
        }
    }

    impl FromWasmAbi for OffsetDateTime {
        type Abi = <Self as IntoWasmAbi>::Abi;

        unsafe fn from_abi(js: Self::Abi) -> Self {
            unsafe { Self(js_sys::Date::from_abi(js).into()) }
        }
    }

    impl OptionIntoWasmAbi for OffsetDateTime {
        fn none() -> Self::Abi {
            <js_sys::Date as OptionIntoWasmAbi>::none()
        }
    }

    impl OptionFromWasmAbi for OffsetDateTime {
        fn is_none(abi: &Self::Abi) -> bool {
            <js_sys::Date as OptionFromWasmAbi>::is_none(abi)
        }
    }

    impl WasmDescribeVector for OffsetDateTime {
        fn describe_vector() {
            Vec::<js_sys::Date>::describe();
        }
    }

    impl VectorIntoWasmAbi for OffsetDateTime {
        type Abi = <js_sys::Date as VectorIntoWasmAbi>::Abi;

        fn vector_into_abi(vector: Box<[Self]>) -> Self::Abi {
            js_value_vector_into_abi(
                vector
                    .into_iter()
                    .map(|Self(dt)| js_sys::Date::from(dt))
                    .collect(),
            )
        }
    }

    impl VectorFromWasmAbi for OffsetDateTime {
        type Abi = <js_sys::Date as VectorFromWasmAbi>::Abi;

        unsafe fn vector_from_abi(js: Self::Abi) -> Box<[Self]> {
            unsafe { js_value_vector_from_abi(js) }
        }
    }
}

#[cfg(feature = "backend")]
mod backend {
    use diesel::{
        backend::Backend,
        deserialize::FromSql,
        pg::Pg,
        serialize::{Output, ToSql},
        sql_types,
    };

    use super::OffsetDateTime;

    impl FromSql<sql_types::Timestamptz, Pg> for OffsetDateTime {
        fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
            Ok(Self(_time::OffsetDateTime::from_sql(bytes)?))
        }
    }

    impl ToSql<sql_types::Timestamptz, Pg> for OffsetDateTime {
        fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
            let Self(inner) = self;
            <_time::OffsetDateTime as ToSql<sql_types::Timestamptz, Pg>>::to_sql(inner, out)
        }
    }
}
