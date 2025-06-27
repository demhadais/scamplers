use std::fmt::Display;

#[cfg(feature = "backend")]
use diesel::{
    backend::Backend,
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    pg::Pg,
    serialize::{Output, ToSql},
    sql_types,
};
use scamplers_macros::base_api_model;
use wasm_bindgen::prelude::*;

#[base_api_model]
#[wasm_bindgen]
#[derive(Clone)]
#[valuable(transparent)]
#[garde(transparent)]
#[serde(transparent)]
#[cfg_attr(feature = "backend", derive(FromSqlRow, AsExpression))]
#[cfg_attr(feature = "backend", diesel(sql_type = sql_types::Text))]
pub struct NonEmptyString(#[garde(length(min = 1))] String);

#[wasm_bindgen]
#[derive(Debug, thiserror::Error)]
#[error("cannot construct `NonEmptyString` from empty string")]
pub struct EmptyStringError;

#[wasm_bindgen]
impl NonEmptyString {
    #[wasm_bindgen(constructor)]
    pub fn new(s: &str) -> Result<Self, EmptyStringError> {
        s.to_non_empty_string()
    }
}

trait ToNonEmptyString {
    fn to_non_empty_string(&self) -> std::result::Result<NonEmptyString, EmptyStringError>;
}

impl ToNonEmptyString for str {
    fn to_non_empty_string(&self) -> std::result::Result<NonEmptyString, EmptyStringError> {
        if self.is_empty() {
            return Err(EmptyStringError);
        }

        Ok(NonEmptyString(self.to_string()))
    }
}

impl AsRef<str> for NonEmptyString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl PartialEq<String> for NonEmptyString {
    fn eq(&self, other: &String) -> bool {
        self.0.eq(other)
    }
}

impl Display for NonEmptyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<&str> for NonEmptyString {
    fn from(value: &str) -> Self {
        value.to_non_empty_string().unwrap()
    }
}

impl From<String> for NonEmptyString {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

#[cfg(feature = "backend")]
impl FromSql<sql_types::Text, Pg> for NonEmptyString {
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        <String as FromSql<sql_types::Text, Pg>>::from_sql(bytes).map(Self)
    }
}

#[cfg(feature = "backend")]
impl ToSql<sql_types::Text, Pg> for NonEmptyString {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        let Self(inner) = self;
        <String as ToSql<sql_types::Text, Pg>>::to_sql(inner, out)
    }
}
