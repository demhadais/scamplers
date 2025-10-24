#[macro_export]
macro_rules! define_ordering_enum {
    {$name:ident { $($variant_name:ident($field:path)),* }, default = $default_variant:ident($default_field:path)} => {
        #[derive(Clone, Debug, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
        #[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
        #[serde(tag = "field", rename_all = "snake_case")]
        #[cfg_attr(feature = "schema", schemars(inline))]
        pub enum $name {
            $(
                #[cfg(feature = "app")]
                $variant_name {
                    #[serde(skip)]
                    field: $field,
                    descending: bool
                },
                #[cfg(not(feature = "app"))]
                $variant_name { descending: bool },
            )*
        }

        impl Default for $name {
            fn default() -> Self {
                #[cfg(feature = "app")]
                {
                    return Self::$default_variant {
                        field: $default_field,
                        descending: false
                    };
                }

                #[cfg(not(feature = "app"))]
                {
                    return Self::$default_variant {
                        descending: false
                    };
                }
            }
        }
    };
}

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
