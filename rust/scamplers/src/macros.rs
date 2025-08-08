macro_rules! define_ordering_enum {
    ($name:ident; $wasm_order_by:path; $($column:ident),*; default = $default_variant:ident; $($enum_attribute:meta),*) => {
        #[derive(Clone, Debug, ::serde::Serialize, ::serde::Deserialize, ::strum::EnumString, ::strum::Display)]
        #[serde(tag = "field")]
        $(#[$enum_attribute])*
        pub enum $name {
            $(
                #[allow(non_camel_case_types)]
                $column {
                    #[serde(skip)]
                    column: $column,
                    descending: bool
                },
            )*
        }

        impl Default for $name {
            fn default() -> Self {
                Self::$default_variant {
                    column: Default::default(),
                    descending: false
                }
            }
        }

        #[cfg(feature = "python")]
        #[::pyo3::prelude::pymethods]
        impl $name {
            #[getter(name = "descending")]
            fn py_descending(&self) -> bool {
                self.descending()
            }

            #[setter]
            fn py_set_descending(&mut self, descending: bool) {
                self.set_descending(descending)
            }
        }

        impl $name {
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

            fn from_pair((column, descending): &(&str, bool)) -> Self {
                use std::str::FromStr;

                let mut s = Self::from_str(column).unwrap();
                s.set_descending(*descending);

                s
            }
        }

        #[cfg(target_arch = "wasm32")]
        impl From<$wasm_order_by> for $name {
            fn from(wasm: $wasm_order_by) -> Self {
                Self::from_pair(&wasm.as_tuple())
            }
        }

        #[cfg(target_arch = "wasm32")]
        impl From<$name> for $wasm_order_by {
            fn from(order_by: $name) -> $wasm_order_by {
                $wasm_order_by {
                    column: order_by.to_string(),
                    descending: order_by.descending()
                }
            }
        }
    };
}
