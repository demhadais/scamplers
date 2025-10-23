#[cfg(feature = "app")]
use std::{fmt::Debug, str::FromStr};

#[cfg(feature = "app")]
use diesel::{
    deserialize,
    pg::{Pg, PgValue},
    serialize,
};

#[cfg(feature = "app")]
trait EnumFromSql: FromStr
where
    <Self as FromStr>::Err: Debug + std::error::Error + Send + Sync + 'static,
{
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        use diesel::{deserialize::FromSql, sql_types};

        let string: String = FromSql::<sql_types::Text, Pg>::from_sql(bytes)?;
        Ok(Self::from_str(&string)?)
    }
}

#[cfg(feature = "app")]
trait EnumToSql
where
    &'static str: for<'a> From<&'a Self>,
{
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Pg>) -> serialize::Result {
        use ::diesel::{pg::Pg, serialize::ToSql, sql_types};

        let as_str: &'static str = self.into();
        ToSql::<sql_types::Text, Pg>::to_sql(as_str, &mut out.reborrow())
    }
}
