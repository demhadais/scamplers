use proc_macro::TokenStream;
use quote::quote;
use syn::{Item, ItemEnum, ItemStruct, parse, parse_macro_input};

fn base_api_model_derives(input: TokenStream) -> proc_macro2::TokenStream {
    let item: Item = parse(input).unwrap();

    let first_lines = quote! {
        #[derive(Debug, ::serde::Deserialize, ::serde::Serialize, ::valuable::Valuable, ::garde::Validate)]
        #[garde(allow_unvalidated)]
    };

    match item {
        Item::Enum(_) => quote! {
            #first_lines
            #[serde(rename_all = "snake_case")]
        },
        Item::Struct(_) => quote! {
            #first_lines
            #[derive(::getset::Getters)]
            #[getset(get = "pub")]
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

fn builder_derive() -> TokenStream {
    let output = quote! {
        #[derive(::derive_builder::Builder)]
        #[builder(pattern = "owned", build_fn(error = crate::model::BuilderError))]
        #[cfg_attr(target_arch = "wasm32", builder_struct_attr(::wasm_bindgen::prelude::wasm_bindgen))]
        #[cfg_attr(target_arch = "wasm32", builder_impl_attr(::wasm_bindgen::prelude::wasm_bindgen))]
        #[cfg_attr(target_arch = "wasm32", builder_field_attr(::wasm_bindgen::prelude::wasm_bindgen))]
        #[cfg_attr(not(target_arch = "wasm32"), builder(setter(strip_option, into)))]
    };

    output.into()
}

#[proc_macro_attribute]
pub fn db_insertion(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = base_api_model(_attr, input);
    let struct_item = parse_macro_input!(input as ItemStruct);

    let builder_attributes = builder_derive();
    let builder_attributes = parse_macro_input!(builder_attributes as proc_macro2::TokenStream);

    let output = quote! {
        #builder_attributes
        #[cfg_attr(
            feature = "backend",
            derive(::diesel::Insertable),
            diesel(check_for_backend(::diesel::pg::Pg))
        )]
        #struct_item
    };

    output.into()
}

#[proc_macro_attribute]
pub fn db_insertion_with_wasm(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = base_api_model(_attr, input);
    let struct_item = parse_macro_input!(input as ItemStruct);

    let builder_attributes = builder_derive();
    let builder_attributes = parse_macro_input!(builder_attributes as proc_macro2::TokenStream);

    let output = quote! {
        #[cfg_attr(target_arch = "wasm32", ::wasm_bindgen::prelude::wasm_bindgen)]
        #builder_attributes
        #[cfg_attr(
            feature = "backend",
            derive(::diesel::Insertable),
            diesel(check_for_backend(::diesel::pg::Pg))
        )]
        #struct_item
    };

    output.into()
}

#[proc_macro_attribute]
pub fn db_query(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = base_api_model_with_default(_attr, input);
    let struct_item = parse_macro_input!(input as ItemStruct);

    let builder_attributes = builder_derive();
    let builder_attributes = parse_macro_input!(builder_attributes as proc_macro2::TokenStream);

    let output = quote! {
        #[cfg_attr(target_arch = "wasm32", ::wasm_bindgen::prelude::wasm_bindgen)]
        #builder_attributes
        #[builder(default)]
        #struct_item
    };

    output.into()
}

#[proc_macro_attribute]
pub fn db_selection(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = base_api_model(_attr, input);
    let struct_item = parse_macro_input!(input as ItemStruct);

    let output = quote! {
        #[cfg_attr(target_arch = "wasm32", ::wasm_bindgen::prelude::wasm_bindgen)]
        #[cfg_attr(feature = "backend", derive(::diesel::Selectable, ::diesel::Queryable), diesel(check_for_backend(::diesel::pg::Pg)))]
        #struct_item
    };

    output.into()
}

#[proc_macro_attribute]
pub fn db_update(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = base_api_model_with_default(_attr, input);
    let struct_item = parse_macro_input!(input as ItemStruct);

    let builder_attributes = builder_derive();
    let builder_attributes = parse_macro_input!(builder_attributes as proc_macro2::TokenStream);

    let output = quote! {
        #builder_attributes
        #[cfg_attr(feature = "backend", derive(::diesel::AsChangeset, ::diesel::Identifiable), diesel(check_for_backend(::diesel::pg::Pg)))]
        #struct_item
    };

    output.into()
}

#[proc_macro_attribute]
pub fn db_update_with_wasm(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = base_api_model(_attr, input);
    let struct_item = parse_macro_input!(input as ItemStruct);

    let builder_attributes = builder_derive();
    let builder_attributes = parse_macro_input!(builder_attributes as proc_macro2::TokenStream);

    let output = quote! {
        #[cfg_attr(target_arch = "wasm32", ::wasm_bindgen::prelude::wasm_bindgen)]
        #builder_attributes
        #[cfg_attr(feature = "backend", derive(::diesel::AsChangeset, ::diesel::Identifiable), diesel(check_for_backend(::diesel::pg::Pg)))]
        #struct_item
    };

    output.into()
}

#[proc_macro_attribute]
pub fn db_enum(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = base_api_model(_attr, input);
    let enum_item = parse_macro_input!(input as ItemEnum);

    let ItemEnum { ident, .. } = &enum_item;

    let output = quote! {
        #[derive(::strum::EnumString, ::strum::IntoStaticStr)]
        #[strum(serialize_all = "snake_case")]
        #[cfg_attr(target_arch = "wasm32", ::wasm_bindgen::prelude::wasm_bindgen)]
        #[cfg_attr(feature = "backend", derive(::diesel::deserialize::FromSqlRow, ::diesel::expression::AsExpression))]
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
pub fn db_json(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = base_api_model(_attr, input);
    let item = parse_macro_input!(input as Item);

    let (Item::Struct(ItemStruct { ident, .. }) | Item::Enum(ItemEnum { ident, .. })) = &item
    else {
        panic!("backend_db_json can only be used on structs and enums")
    };

    let output = quote! {
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
