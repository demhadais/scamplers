#[cfg(feature = "backend")]
use diesel::{deserialize::FromSqlRow, expression::AsExpression, sql_types};
#[cfg(feature = "python")]
use pyo3::prelude::*;
use {
    _uuid::Bytes,
    serde::{Deserialize, Serialize},
    std::{fmt::Display, str::FromStr},
};

#[derive(
    Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Debug, Hash, Default, Deserialize, Serialize,
)]
#[cfg_attr(feature = "python", derive(IntoPyObject, FromPyObject))]
#[cfg_attr(feature = "backend", derive(FromSqlRow, AsExpression))]
#[cfg_attr(feature = "python", pyo3(transparent))]
#[cfg_attr(feature = "backend", diesel(sql_type = sql_types::Uuid))]
#[serde(transparent)]
pub struct Uuid(_uuid::Uuid);

impl Display for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(inner) = self;
        inner.fmt(f)
    }
}

impl FromStr for Uuid {
    type Err = <_uuid::Uuid as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(_uuid::Uuid::from_str(s)?))
    }
}

impl Uuid {
    #[must_use]
    pub fn as_bytes(&self) -> &Bytes {
        let Self(inner) = self;
        inner.as_bytes()
    }

    #[must_use]
    pub fn now_v7() -> Self {
        Self(_uuid::Uuid::now_v7())
    }
}

impl valuable::Valuable for Uuid {
    fn as_value(&self) -> valuable::Value<'_> {
        self.as_bytes().as_value()
    }

    fn visit(&self, visit: &mut dyn valuable::Visit) {
        self.as_bytes().visit(visit);
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm32 {
    use super::Uuid;
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

    impl WasmDescribe for Uuid {
        fn describe() {
            String::describe();
        }
    }

    impl From<Uuid> for JsValue {
        fn from(val: Uuid) -> Self {
            val.to_string().into()
        }
    }

    impl TryFromJsValue for Uuid {
        type Error = _uuid::Error;

        fn try_from_js_value(value: JsValue) -> Result<Self, Self::Error> {
            Self::from_str(&String::try_from_js_value(value).unwrap())
        }
    }

    impl IntoWasmAbi for Uuid {
        type Abi = <String as IntoWasmAbi>::Abi;

        fn into_abi(self) -> Self::Abi {
            self.to_string().into_abi()
        }
    }

    impl FromWasmAbi for Uuid {
        type Abi = <Self as IntoWasmAbi>::Abi;

        unsafe fn from_abi(js: Self::Abi) -> Self {
            Self(unsafe { String::from_abi(js).parse().unwrap() })
        }
    }

    impl OptionIntoWasmAbi for Uuid {
        fn none() -> Self::Abi {
            <String as OptionIntoWasmAbi>::none()
        }
    }

    impl OptionFromWasmAbi for Uuid {
        fn is_none(abi: &Self::Abi) -> bool {
            <String as OptionFromWasmAbi>::is_none(abi)
        }
    }

    impl WasmDescribeVector for Uuid {
        fn describe_vector() {
            Vec::<String>::describe();
        }
    }

    impl VectorIntoWasmAbi for Uuid {
        type Abi = <String as VectorIntoWasmAbi>::Abi;

        fn vector_into_abi(vector: Box<[Self]>) -> Self::Abi {
            js_value_vector_into_abi(vector)
        }
    }

    impl VectorFromWasmAbi for Uuid {
        type Abi = <String as VectorFromWasmAbi>::Abi;

        unsafe fn vector_from_abi(js: Self::Abi) -> Box<[Self]> {
            unsafe { js_value_vector_from_abi(js) }
        }
    }
}

#[cfg(feature = "backend")]
mod backend {
    use {
        super::Uuid,
        diesel::{
            backend::Backend,
            deserialize::FromSql,
            pg::Pg,
            serialize::{Output, ToSql},
            sql_types,
        },
    };

    impl FromSql<sql_types::Uuid, Pg> for Uuid {
        fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
            Ok(Self(_uuid::Uuid::from_sql(bytes)?))
        }
    }

    impl ToSql<sql_types::Uuid, Pg> for Uuid {
        fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
            let Self(inner) = self;
            <_uuid::Uuid as ToSql<sql_types::Uuid, Pg>>::to_sql(inner, out)
        }
    }
}
