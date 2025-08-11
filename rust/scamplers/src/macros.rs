#[macro_export]
macro_rules! define_ordering_enum {
    ($name:ident; $wasm_python_orderby:path; $($variant:ident),*; default = $default_variant:ident; $($enum_attribute:meta),*) => {
        #[derive(Clone, Debug, PartialEq, ::serde::Serialize, ::serde::Deserialize, ::strum::EnumString, ::strum::Display, ::valuable::Valuable)]
        #[serde(tag = "field", rename_all = "snake_case")]
        #[strum(serialize_all = "snake_case")]
        $(#[$enum_attribute])*
        pub enum $name {
            $(
                $variant {
                    descending: bool
                },
            )*
        }

        impl Default for $name {
            fn default() -> Self {
                Self::$default_variant {
                    descending: false
                }
            }
        }

        #[allow(dead_code)]
        impl $name {
            fn new(field: &str, descending: bool) -> Result<Self, ::strum::ParseError> {
                use std::str::FromStr;

                let mut orderby = Self::from_str(field)?;
                orderby.set_descending(descending);

                Ok(orderby)
            }

            fn set_descending(&mut self, descending: bool) {
                match self {
                    $( Self::$variant{ descending: current, ..} )|* => { *current = descending; }
                }
            }

            fn descending(&self) -> bool {
                match self {
                    $( Self::$variant{ descending, ..} )|* => *descending
                }
            }
        }

        #[cfg(target_arch = "wasm32")]
        impl From<$wasm_python_orderby> for $name {
            fn from(wasm: $wasm_python_orderby) -> Self {
                use wasm_bindgen::prelude::*;

                Self::new(&wasm.field, wasm.descending).unwrap_throw()
            }
        }

        #[cfg(any(target_arch = "wasm32", feature = "python"))]
        impl From<$name> for $wasm_python_orderby {
            fn from(order_by: $name) -> $wasm_python_orderby {
                $wasm_python_orderby {
                    field: order_by.to_string(),
                    descending: order_by.descending()
                }
            }
        }
    };
}
