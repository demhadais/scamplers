use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident, Item, ItemEnum, ItemStruct, TypePath, parse, parse_macro_input};

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
        #[derive(Debug, ::serde::Deserialize, ::serde::Serialize, Clone, ::garde::Validate, ::valuable::Valuable, ::thiserror::Error, PartialEq)]
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
        #[cfg_attr(feature = "python", ::pyo3::pyclass(get_all, set_all, str, eq))]
        #[cfg_attr(
            feature = "backend",
            derive(::diesel::Insertable),
            diesel(check_for_backend(::diesel::pg::Pg))
        )]
        #[derive(::bon::Builder)]
        #[builder(on(_, into), derive(Clone, Debug, Into))]
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
        #[cfg_attr(feature = "python", ::pyo3::pyclass(str, eq))]
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
        #[cfg_attr(feature = "python", ::pyo3::pyclass(get_all, str, eq))]
        #[cfg_attr(target_arch = "wasm32", ::wasm_bindgen::prelude::wasm_bindgen(inspectable, getter_with_clone))]
        #[cfg_attr(feature = "backend", derive(::diesel::Selectable, ::diesel::Queryable), diesel(check_for_backend(::diesel::pg::Pg)))]
        #struct_item
    };

    output.into()
}

#[proc_macro_attribute]
pub fn db_update(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = base_api_model_with_default(attr, input);
    let struct_item = parse_macro_input!(input as ItemStruct);

    let output = quote! {
        #[cfg_attr(feature = "python", ::pyo3::pyclass(str, eq))]
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
        #[cfg_attr(feature = "python", ::pyo3::pyclass(str, eq))]
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
        #[cfg_attr(feature = "python", ::pyo3::pyclass(get_all, str, eq))]
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

fn extract_wrapper_type_and_python(
    DeriveInput { attrs, .. }: &DeriveInput,
    attribute_name: &str,
) -> (Option<TypePath>, bool) {
    let mut wrapper_type = None;
    let mut python = false;

    for attribute in attrs {
        if attribute.path().is_ident(attribute_name) {
            attribute
                .parse_nested_meta(|meta| {
                    if meta.path.is_ident("wrapper") {
                        wrapper_type = Some(meta.value()?.parse()?);
                    } else if meta.path.is_ident("python") {
                        python = true;
                    } else {
                        return Err(meta.error(
                            "expected attribute '#[json(wrapper = Type)]' or #[json(wrapper = \
                             Type, python)]'",
                        ));
                    }

                    Ok(())
                })
                .unwrap();
        }
    }

    (wrapper_type, python)
}

#[proc_macro_derive(ToJson, attributes(json))]
pub fn derive_to_json(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let name = &derive_input.ident;

    let (wrapper_type, python) = extract_wrapper_type_and_python(&derive_input, "json");

    let method_body = match wrapper_type {
        Some(wrapper_type) => {
            quote! {
                ::serde_json::to_string(&#wrapper_type::from(self.clone())).unwrap()
            }
        }
        None => {
            quote! {
                ::serde_json::to_string(self).unwrap()
            }
        }
    };

    let mut output = quote! {
        impl #name {
            pub fn to_json(&self) -> String {
                #method_body
            }
        }
    };

    if python {
        output = quote! {
            #[cfg_attr(feature = "python", ::pyo3::pymethods)]
            #output
        }
    }

    output.into()
}

#[proc_macro_derive(FromJson, attributes(json))]
pub fn derive_from_json(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let name = &derive_input.ident;

    let (wrapper_type, python) = extract_wrapper_type_and_python(&derive_input, "json");

    let method_body = if let Some(wrapper_type) = wrapper_type {
        quote! {
            let wrapped: #wrapper_type = serde_json::from_str(json)?;
            Ok(wrapped.try_into()?)
        }
    } else {
        quote! {
            Ok(serde_json::from_str(json)?)
        }
    };

    let mut output = quote! {
        impl #name {
            pub fn from_json(json: &str) -> ::anyhow::Result<Self> {
                #method_body
            }
        }
    };

    if python {
        output = quote! {
            #output

            #[cfg(feature = "python")]
            #[::pyo3::pymethods]
            impl #name {
                #[staticmethod]
                #[pyo3(name = "from_json")]
                fn py_from_json(json: &str) -> ::anyhow::Result<Self> {
                    Self::from_json(json)
                }
            }
        }
    }

    output.into()
}
