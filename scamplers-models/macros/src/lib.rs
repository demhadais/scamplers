#[macro_export]
macro_rules! impl_json_from_sql {
    ($name:ident) => {
        #[cfg(feature = "app")]
        impl ::diesel::deserialize::FromSql<::diesel::sql_types::Jsonb, ::diesel::pg::Pg>
            for $name
        {
            fn from_sql(bytes: ::diesel::pg::PgValue<'_>) -> ::diesel::deserialize::Result<Self> {
                Self::from_sql_inner(bytes)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_json_to_sql {
    ($name:ident) => {
        #[cfg(feature = "app")]
        impl ::diesel::serialize::ToSql<::diesel::sql_types::Jsonb, ::diesel::pg::Pg> for $name {
            fn to_sql<'b>(
                &'b self,
                out: &mut diesel::serialize::Output<'b, '_, ::diesel::pg::Pg>,
            ) -> ::diesel::serialize::Result {
                Self::to_sql_inner(bytes)
            }
        }
    };
}
