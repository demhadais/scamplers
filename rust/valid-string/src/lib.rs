use std::{fmt::Display, str::FromStr};

#[cfg(feature = "backend")]
use diesel::{deserialize::FromSqlRow, expression::AsExpression, sql_types};
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    Debug,
    Hash,
    Default,
    Deserialize,
    Serialize,
    garde::Validate,
    valuable::Valuable,
)]
#[cfg_attr(feature = "python", derive(IntoPyObject, FromPyObject))]
#[valuable(transparent)]
#[garde(transparent)]
#[serde(transparent)]
#[cfg_attr(feature = "python", pyo3(transparent))]
#[cfg_attr(feature = "backend", derive(FromSqlRow, AsExpression))]
#[cfg_attr(feature = "backend", diesel(sql_type = sql_types::Text))]
pub struct ValidString(#[garde(length(min = 1))] String);

impl Display for ValidString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for ValidString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl ValidString {
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Debug, thiserror::Error)]
#[error("cannot construct `ValidString` from empty string")]
pub struct EmptyStringError;

trait ToValidString {
    fn to_valid_string(&self) -> std::result::Result<ValidString, EmptyStringError>;
}

impl ToValidString for str {
    fn to_valid_string(&self) -> std::result::Result<ValidString, EmptyStringError> {
        if self.is_empty() {
            return Err(EmptyStringError);
        }

        Ok(ValidString(self.to_string()))
    }
}

impl FromStr for ValidString {
    type Err = EmptyStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.to_valid_string()
    }
}

impl From<&str> for ValidString {
    fn from(value: &str) -> Self {
        value.to_valid_string().unwrap()
    }
}

impl From<String> for ValidString {
    fn from(value: String) -> Self {
        value.to_valid_string().unwrap()
    }
}

impl PartialEq<&str> for ValidString {
    fn eq(&self, other: &&str) -> bool {
        self.0.eq(other)
    }
}
impl PartialEq<String> for ValidString {
    fn eq(&self, other: &String) -> bool {
        self == &other.as_str()
    }
}
impl PartialEq<ValidString> for &str {
    fn eq(&self, other: &ValidString) -> bool {
        other == self
    }
}
impl PartialEq<ValidString> for String {
    fn eq(&self, other: &ValidString) -> bool {
        other == &self.as_str()
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm32 {
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

    use super::{EmptyStringError, ValidString};

    impl WasmDescribe for ValidString {
        fn describe() {
            String::describe();
        }
    }

    impl From<ValidString> for JsValue {
        fn from(val: ValidString) -> Self {
            val.0.into()
        }
    }

    impl TryFromJsValue for ValidString {
        type Error = EmptyStringError;

        fn try_from_js_value(value: JsValue) -> Result<Self, Self::Error> {
            let s = String::try_from_js_value(value).map_err(|_| EmptyStringError)?;
            Self::from_str(&s)
        }
    }

    impl IntoWasmAbi for ValidString {
        type Abi = <String as IntoWasmAbi>::Abi;

        fn into_abi(self) -> Self::Abi {
            self.0.into_abi()
        }
    }

    impl FromWasmAbi for ValidString {
        type Abi = <Self as IntoWasmAbi>::Abi;

        unsafe fn from_abi(js: Self::Abi) -> Self {
            Self(unsafe { String::from_abi(js).parse().unwrap() })
        }
    }

    impl OptionIntoWasmAbi for ValidString {
        fn none() -> Self::Abi {
            <String as OptionIntoWasmAbi>::none()
        }
    }

    impl OptionFromWasmAbi for ValidString {
        fn is_none(abi: &Self::Abi) -> bool {
            <String as OptionFromWasmAbi>::is_none(abi)
        }
    }

    impl WasmDescribeVector for ValidString {
        fn describe_vector() {
            Vec::<String>::describe();
        }
    }

    impl VectorIntoWasmAbi for ValidString {
        type Abi = <String as VectorIntoWasmAbi>::Abi;

        fn vector_into_abi(vector: Box<[Self]>) -> Self::Abi {
            js_value_vector_into_abi(vector)
        }
    }

    impl VectorFromWasmAbi for ValidString {
        type Abi = <String as VectorFromWasmAbi>::Abi;

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

    use super::ValidString;

    impl FromSql<sql_types::Text, Pg> for ValidString {
        fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
            <String as FromSql<sql_types::Text, Pg>>::from_sql(bytes).map(Self)
        }
    }

    impl ToSql<sql_types::Text, Pg> for ValidString {
        fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
            let Self(inner) = self;
            <String as ToSql<sql_types::Text, Pg>>::to_sql(inner, out)
        }
    }
}
