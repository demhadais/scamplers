#[macro_export]
macro_rules! define_ordering_enum {
    {$name:ident { $($variant:ident),* }, default = $default:ident} => {
        #[allow(non_camel_case_types)]
        #[derive(Clone, Debug, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
        #[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
        #[serde(tag = "field", rename_all = "snake_case")]
        enum $name {
            $(
                $variant {
                    #[cfg(feature = "app")]
                    #[serde(skip)]
                    field: $variant,
                    #[serde(default)]
                    descending: bool
                },
            )*
        }

        impl Default for $name {
            fn default() -> Self {
                Self::$default {
                    #[cfg(feature = "app")]
                    field: $default,
                    descending: false
                }
            }
        }
    };
}
