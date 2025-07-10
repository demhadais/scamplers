#[cfg(feature = "backend")]
use {
    diesel::{
        backend::Backend,
        deserialize::{FromSql, FromSqlRow},
        expression::AsExpression,
        pg::Pg,
        serialize::{Output, ToSql},
        sql_types,
    },
    garde::rules::AsStr,
    std::{fmt::Display, str},
};

#[cfg(feature = "typescript")]
use wasm_bindgen::prelude::*;

#[cfg_attr(
    feature = "backend",
    derive(
        serde::Deserialize,
        serde::Serialize,
        garde::Validate,
        valuable::Valuable,
        FromSqlRow,
        AsExpression,
        Debug
    ),
    garde(transparent),
    valuable(transparent)
)]
#[cfg_attr(feature = "backend", diesel(sql_type = sql_types::Text))]
#[cfg_attr(feature = "typescript", wasm_bindgen, derive(serde::Serialize))]
#[derive(Clone)]
#[serde(transparent)]
pub struct ValidString(#[cfg_attr(feature = "backend", garde(length(min = 1)))] String);

#[derive(Debug, thiserror::Error)]
#[error("cannot construct `ValidString` from empty string")]
#[cfg_attr(feature = "typescript", wasm_bindgen)]
pub struct EmptyStringError;

trait ToValidString {
    fn to_non_empty_string(&self) -> std::result::Result<ValidString, EmptyStringError>;
}

impl ToValidString for str {
    fn to_non_empty_string(&self) -> std::result::Result<ValidString, EmptyStringError> {
        if self.is_empty() {
            return Err(EmptyStringError);
        }

        Ok(ValidString(self.to_string()))
    }
}

#[cfg(feature = "backend")]
impl AsRef<str> for ValidString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(feature = "backend")]
impl PartialEq<String> for ValidString {
    fn eq(&self, other: &String) -> bool {
        self.0.eq(other)
    }
}

#[cfg(feature = "backend")]
impl Display for ValidString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<&str> for ValidString {
    fn from(value: &str) -> Self {
        value.to_non_empty_string().unwrap()
    }
}

impl From<String> for ValidString {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

#[cfg(feature = "backend")]
impl FromSql<sql_types::Text, Pg> for ValidString {
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        <String as FromSql<sql_types::Text, Pg>>::from_sql(bytes).map(Self)
    }
}

#[cfg(feature = "backend")]
impl ToSql<sql_types::Text, Pg> for ValidString {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        let Self(inner) = self;
        <String as ToSql<sql_types::Text, Pg>>::to_sql(inner, out)
    }
}

#[cfg(feature = "backend")]
impl AsStr for ValidString {
    fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(feature = "typescript")]
#[wasm_bindgen]
impl ValidString {
    #[wasm_bindgen(constructor)]
    pub fn new(s: &str) -> std::result::Result<Self, EmptyStringError> {
        s.to_non_empty_string()
    }
}
