use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Ident, ImplItem, ImplItemFn, Item, ItemEnum, ItemImpl, ItemStruct, parse, parse_macro_input,
    parse2,
};

trait GetIdent {
    fn get_ident(&self) -> &Ident;
}
impl GetIdent for Item {
    fn get_ident(&self) -> &Ident {
        match self {
            Item::Enum(enum_item) => &enum_item.ident,
            Item::Struct(struct_item) => &struct_item.ident,
            _ => panic!("expected enum or struct"),
        }
    }
}

fn base_api_model_derives(input: TokenStream) -> proc_macro2::TokenStream {
    let item: Item = parse(input).unwrap();

    let first_lines = quote! {
        #[derive(Debug, ::serde::Deserialize, ::serde::Serialize, Clone, ::garde::Validate, ::valuable::Valuable, ::thiserror::Error)]
        #[garde(allow_unvalidated)]
        #[error("{self:#?}")]
    };

    match item {
        Item::Enum(_) => quote! {
            #first_lines
            #[serde(rename_all = "snake_case")]
        },
        Item::Struct(_) => quote! {
            #first_lines
        },
        _ => panic!("expected enum or struct"),
    }
}

#[proc_macro_attribute]
pub fn to_from_json(attr: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Item);
    let ident = item.get_ident();

    let attr = attr.to_string();
    let python = if attr == "python" {
        true
    } else if attr.is_empty() {
        false
    } else {
        panic!("unexpected proc macro attribute {attr}")
    };

    let mut from_json_method = quote! {
        pub fn from_json(
            json: &str,
        ) -> ::std::result::Result<Self, crate::result::ScamplersCoreErrorResponse> {
            let error = |err| {
                let inner = crate::result::ClientError {
                    message: format!("failed to deserialize provided json string: {err}")
                };

                crate::result::ScamplersCoreErrorResponse::builder().error(inner).build()
            };

            ::serde_json::from_str(json).map_err(error)
        }
    };

    if python {
        from_json_method = quote! {
            #[staticmethod]
            #from_json_method
        };
    }

    let implementation = quote! {
        impl #ident {
            #from_json_method

            pub fn to_json(&self) -> String {
                ::serde_json::to_string(self).unwrap()
            }
        }
    };

    let result = if python {
        quote! {
            #item
            #[cfg(feature = "python")]
            #[::pyo3::pymethods]
            #implementation
        }
    } else {
        quote! {
            #item
            #[cfg(not(feature = "python"))]
            #implementation
        }
    };

    result.into()
}

#[proc_macro_attribute]
pub fn base_api_model(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let derives = base_api_model_derives(input.clone());
    let item = parse_macro_input!(input as Item);

    let output = quote! {
        #derives
        #item
    };

    output.into()
}

#[proc_macro_attribute]
pub fn scamplers_error(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let derives = base_api_model_derives(input.clone());
    let item = parse_macro_input!(input as Item);

    let output = quote! {
        #[cfg_attr(target_arch = "wasm32", ::wasm_bindgen::prelude::wasm_bindgen(inspectable, getter_with_clone))]
        #[cfg_attr(feature = "python", ::pyo3::pyclass(get_all, str))]
        #[derive(::bon::Builder)]
        #[builder(on(_, into))]
        #derives
        #item
    };

    output.into()
}

#[proc_macro_attribute]
pub fn base_api_model_with_default(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let derives = base_api_model_derives(input.clone());
    let item = parse_macro_input!(input as Item);

    let first_lines = match item {
        Item::Enum(_) => quote! {
            #derives
            #[derive(Default)]
        },
        Item::Struct(_) => quote! {
            #derives
            #[derive(Default)]
            #[serde(default)]
        },
        _ => panic!("expected enum or struct"),
    };

    quote! {
        #first_lines
        #item
    }
    .into()
}

#[proc_macro_attribute]
pub fn db_insertion(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = base_api_model(attr, input);
    let struct_item = parse_macro_input!(input as ItemStruct);

    let output = quote! {
        #[cfg_attr(feature = "python", ::pyo3::pyclass(get_all, set_all, str))]
        #[cfg_attr(
            feature = "backend",
            derive(::diesel::Insertable),
            diesel(check_for_backend(::diesel::pg::Pg))
        )]
        #[derive(::bon::Builder)]
        #[builder(on(_, into))]
        #struct_item
    };

    output.into()
}

#[proc_macro_attribute]
pub fn db_query(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = base_api_model_with_default(attr, input);
    let struct_item = parse_macro_input!(input as ItemStruct);

    let struct_name = &struct_item.ident;

    let output = quote! {
        #[cfg_attr(feature = "python", ::pyo3::pyclass(str))]
        #[cfg_attr(target_arch = "wasm32", ::wasm_bindgen::prelude::wasm_bindgen(inspectable, getter_with_clone))]
        #[derive(::bon::Builder)]
        #[builder(on(_, into))]
        #struct_item

        #[cfg(target_arch = "wasm32")]
        #[::wasm_bindgen::prelude::wasm_bindgen]
        impl #struct_name {
            #[wasm_bindgen(constructor)]
            pub fn new() -> Self {
                Self::default()
            }
        }
    };

    output.into()
}

#[proc_macro_attribute]
pub fn db_selection(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = base_api_model(attr, input);
    let struct_item = parse_macro_input!(input as ItemStruct);

    let output = quote! {
        #[cfg_attr(feature = "python", ::pyo3::pyclass(get_all, str))]
        #[cfg_attr(target_arch = "wasm32", ::wasm_bindgen::prelude::wasm_bindgen(inspectable, getter_with_clone))]
        #[cfg_attr(feature = "backend", derive(::diesel::Selectable, ::diesel::Queryable), diesel(check_for_backend(::diesel::pg::Pg)))]
        #struct_item
    };

    output.into()
}

fn add_attribute_to_method(
    method: &ImplItemFn,
    attribute: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    quote! {
        #attribute
        #method
    }
}

#[proc_macro_attribute]
pub fn getters_impl(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut impl_block = parse_macro_input!(input as ItemImpl);

    let mut py_impl_block = impl_block.clone();

    let og_methods = impl_block.items.iter_mut().filter_map(|i| match i {
        ImplItem::Fn(f) => Some(f),
        _ => None,
    });

    for (i, og_method) in og_methods.enumerate() {
        let wasmified_method = add_attribute_to_method(
            og_method,
            &quote! { #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))] },
        );

        let pythonized_method = add_attribute_to_method(og_method, &quote! { #[getter] });
        py_impl_block.items[i] = parse2(pythonized_method).unwrap();

        *og_method = parse2(wasmified_method).unwrap();
    }

    let output = quote! {
        #[cfg(not(feature = "python"))]
        #[cfg_attr(target_arch = "wasm32", ::wasm_bindgen::prelude::wasm_bindgen)]
        #impl_block

        #[cfg(feature = "python")]
        #[::pyo3::pymethods]
        #py_impl_block
    };

    output.into()
}

#[proc_macro_attribute]
pub fn db_update(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = base_api_model_with_default(attr, input);
    let struct_item = parse_macro_input!(input as ItemStruct);

    let output = quote! {
        #[cfg_attr(feature = "python", ::pyo3::pyclass(str))]
        #[cfg_attr(feature = "backend", derive(::diesel::AsChangeset, ::diesel::Identifiable), diesel(check_for_backend(::diesel::pg::Pg)))]
        #[derive(::bon::Builder)]
        #[builder(on(_, into))]
        #struct_item
    };

    output.into()
}

#[proc_macro_attribute]
pub fn db_enum(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = base_api_model(attr, input);
    let enum_item = parse_macro_input!(input as ItemEnum);

    let ItemEnum { ident, .. } = &enum_item;

    let output = quote! {
        #[cfg_attr(feature = "python", ::pyo3::pyclass(str))]
        #[cfg_attr(feature = "backend", derive(::diesel::deserialize::FromSqlRow, ::diesel::expression::AsExpression))]
        #[derive(::strum::EnumString, ::strum::IntoStaticStr)]
        #[strum(serialize_all = "snake_case")]
        #[cfg_attr(feature = "backend", diesel(sql_type = ::diesel::sql_types::Text))]
        #enum_item

        #[cfg(feature = "backend")]
        impl ::diesel::deserialize::FromSql<::diesel::sql_types::Text, ::diesel::pg::Pg> for #ident {
            fn from_sql(bytes: <::diesel::pg::Pg as ::diesel::backend::Backend>::RawValue<'_>) -> ::diesel::deserialize::Result<Self> {
                use ::diesel::{deserialize::FromSql, sql_types, pg::Pg};
                use ::std::str::FromStr;

                let string: String = FromSql::<sql_types::Text, Pg>::from_sql(bytes)?;
                Ok(Self::from_str(&string).unwrap())
            }
        }

        #[cfg(feature = "backend")]
        impl ::diesel::serialize::ToSql<::diesel::sql_types::Text, ::diesel::pg::Pg> for #ident {
            fn to_sql<'b>(
                &'b self,
                out: &mut ::diesel::serialize::Output<'b, '_, ::diesel::pg::Pg>,
            ) -> ::diesel::serialize::Result {
                use ::diesel::{serialize::ToSql, sql_types, pg::Pg};

                let as_str: &str = self.into();
                ToSql::<sql_types::Text, Pg>::to_sql(as_str, &mut out.reborrow())
            }
        }
    };

    output.into()
}

#[proc_macro_attribute]
pub fn db_json(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = base_api_model(attr, input);
    let item = parse_macro_input!(input as Item);

    let ident = item.get_ident();

    let output = quote! {
        #[cfg_attr(feature = "python", ::pyo3::pyclass(str))]
        #[cfg_attr(feature = "backend", derive(::diesel::deserialize::FromSqlRow, ::diesel::expression::AsExpression))]
        #[cfg_attr(feature = "backend", diesel(sql_type = ::diesel::sql_types::Jsonb))]
        #item

        #[cfg(feature = "backend")]
        impl ::diesel::deserialize::FromSql<::diesel::sql_types::Jsonb, ::diesel::pg::Pg> for #ident {
            fn from_sql(bytes: <::diesel::pg::Pg as ::diesel::backend::Backend>::RawValue<'_>) -> ::diesel::deserialize::Result<Self> {
                use ::diesel::{deserialize::FromSql, sql_types, pg::Pg};

                let json: ::serde_json::Value = FromSql::<sql_types::Jsonb, Pg>::from_sql(bytes)?;
                Ok(::serde_json::from_value(json).unwrap())
            }
        }

        #[cfg(feature = "backend")]
        impl ::diesel::serialize::ToSql<::diesel::sql_types::Jsonb, ::diesel::pg::Pg> for #ident {
            fn to_sql<'b>(
                &'b self,
                out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
            ) -> ::diesel::serialize::Result {
                use ::diesel::{serialize::ToSql, sql_types, pg::Pg};

                let as_json = ::serde_json::to_value(self).unwrap();
                ToSql::<sql_types::Jsonb, Pg>::to_sql(&as_json, &mut out.reborrow())
            }
        }
    };

    output.into()
}
