use macro_rules_attribute::{attribute_alias, derive_alias};
use scamplers_schema::institution::name;

derive_alias! {
    #[derive(ApiModel!)] = #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, ::garde::Validate, ::valuable::Valuable)];
}

macro_rules! define_ordering_enum {
    ($name:ident; $wasm_python_orderby:path; $($column:ident),*; default = $default_variant:ident; $($enum_attribute:meta),*) => {
        #[derive(Clone, Debug, ::serde::Serialize, ::serde::Deserialize, ::strum::EnumString, ::strum::Display)]
        #[serde(tag = "field")]
        $(#[$enum_attribute])*
        pub enum $name {
            $(
                #[allow(non_camel_case_types)]
                $column {
                    #[serde(skip)]
                    field: $column,
                    descending: bool
                },
            )*
        }

        impl Default for $name {
            fn default() -> Self {
                Self::$default_variant {
                    field: Default::default(),
                    descending: false
                }
            }
        }

        impl $name {
            fn new(field: &str, descending: bool) -> Result<Self, ::strum::ParseError> {
                use std::str::FromStr;

                let mut orderby = Self::from_str(field)?;
                orderby.set_descending(descending);

                Ok(orderby)
            }

            fn set_descending(&mut self, descending: bool) {
                match self {
                    $( Self::$column{ descending: current, ..} )|* => { *current = descending; }
                }
            }

            fn descending(&self) -> bool {
                match self {
                    $( Self::$column{ descending, ..} )|* => *descending
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

define_ordering_enum!(InstitutionOrderBy; super::routes::WasmPythonOrderBy; name; default = name;);
