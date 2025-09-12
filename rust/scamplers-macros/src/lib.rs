use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Item, ItemEnum, ItemStruct, parse_macro_input};

#[proc_macro_attribute]
pub fn base_model(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Item);

    quote! {
        #[derive(Clone, Debug, PartialEq, ::serde::Deserialize, ::serde::Serialize, ::garde::Validate, ::valuable::Valuable)]
        #[garde(allow_unvalidated)]
        #item
    }.into()
}

#[proc_macro_derive(Jsonify)]
pub fn derive_jsonify(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let name = &derive_input.ident;

    quote! {
        impl crate::db::models::Jsonify for #name {}
    }
    .into()
}

#[proc_macro_derive(WasmJsonify)]
pub fn derive_wasm_jsonify(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let name = &derive_input.ident;

    quote! {
        use crate::db::models::Jsonify as _;
        #[cfg(target_arch = "wasm32")]
        #[::wasm_bindgen::prelude::wasm_bindgen]
        impl #name {
            #[wasm_bindgen(js_name = "to_json_bytes")]
            pub fn wasm_to_json_bytes(&self) -> Vec<u8> {
                self.to_json_bytes()
            }

            #[wasm_bindgen(js_name = "to_json_string")]
            pub fn wasm_to_json_string(&self) -> String {
                self.to_json_string()
            }

            #[wasm_bindgen(js_name = "to_base64_json")]
            pub fn wasm_to_base64_json(&self) -> String {
                self.to_base64_json()
            }

            #[wasm_bindgen(js_name = "from_json_bytes")]
            pub fn wasm_from_json_bytes(json_bytes: &[u8]) -> Self {
                use ::wasm_bindgen::prelude::*;

                Self::from_json_bytes(json_bytes).unwrap_throw()
            }

            #[wasm_bindgen(js_name = "from_json_string")]
            pub fn wasm_from_json_string(json_str: &str) -> Self {
                use ::wasm_bindgen::prelude::*;

                Self::from_json_string(json_str).unwrap_throw()
            }

            #[wasm_bindgen(js_name = "from_base64_json")]
            pub fn wasm_from_base64_json(base64_json_bytes: &str) -> Self {
                use ::wasm_bindgen::prelude::*;

                Self::from_base64_json(base64_json_bytes).unwrap_throw()
            }
        }
    }
    .into()
}

#[proc_macro_derive(PyJsonify)]
pub fn derive_py_jsonify(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let name = &derive_input.ident;

    quote! {
        use crate::db::models::Jsonify as _;

        #[cfg(feature = "python")]
        #[::pyo3_stub_gen::derive::gen_stub_pymethods]
        #[::pyo3::pymethods]
        impl #name {
            #[pyo3(name = "to_json_bytes")]
            pub fn py_to_json_bytes(&self) -> Vec<u8> {
                self.to_json_bytes()
            }

            #[pyo3(name = "to_json_string")]
            pub fn py_to_json_string(&self) -> String {
                self.to_json_string()
            }

            #[pyo3(name = "to_base64_json")]
            pub fn py_to_base64_json(&self) -> String {
                self.to_base64_json()
            }

            #[pyo3(name = "from_json_bytes")]
            #[staticmethod]
            pub fn py_from_json_bytes(json_bytes: Vec<u8>) -> ::anyhow::Result<Self> {
                Self::from_json_bytes(&json_bytes)
            }

            #[pyo3(name = "from_json_string")]
            #[staticmethod]
            pub fn py_from_json_string(json_str: &str) -> ::anyhow::Result<Self> {
                Self::from_json_string(json_str)
            }

            #[pyo3(name = "from_base64_json")]
            #[staticmethod]
            pub fn py_from_base64_json(base64_json_bytes: &str) -> ::anyhow::Result<Self> {
                Self::from_base64_json(base64_json_bytes)
            }
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn scamplers_error(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Item);

    let output = quote! {
        #[cfg_attr(feature = "python", ::pyo3_stub_gen::derive::gen_stub_pyclass)]
        #[::scamplers_macros::base_model]
        #[cfg_attr(target_arch = "wasm32", ::wasm_bindgen::prelude::wasm_bindgen(getter_with_clone))]
        #[cfg_attr(feature = "python", ::pyo3::pyclass(get_all, module = "scamplepy.errors"))]
        #[derive(Default, ::scamplers_macros::Jsonify, ::scamplers_macros::WasmJsonify, ::scamplers_macros::PyJsonify, ::thiserror::Error, ::bon::Builder)]
        #[builder(on(_, into))]
        #item
    };

    output.into()
}

#[proc_macro_attribute]
pub fn db_insertion(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let struct_item = parse_macro_input!(input as Item);

    let output = quote! {
        #[::scamplers_macros::base_model]
        #[cfg_attr(feature = "python", ::pyo3_stub_gen::derive::gen_stub_pyclass)]
        #[cfg_attr(feature = "python", ::pyo3::pyclass(get_all, set_all, eq, module = "scamplepy.create"))]
        #[cfg_attr(
            feature = "app",
            derive(::diesel::Insertable),
            diesel(check_for_backend(::diesel::pg::Pg))
        )]
        #[derive(::scamplers_macros::Jsonify, ::scamplers_macros::PyJsonify, ::bon::Builder)]
        #[builder(on(_, into), derive(Clone, Debug, Into))]
        #struct_item
    };

    output.into()
}

#[proc_macro_attribute]
pub fn db_query(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let struct_item = parse_macro_input!(input as ItemStruct);

    let struct_name = &struct_item.ident;

    let output = quote! {
        #[::scamplers_macros::base_model]
        #[cfg_attr(target_arch = "wasm32", ::wasm_bindgen::prelude::wasm_bindgen(getter_with_clone))]
        #[cfg_attr(feature = "python", ::pyo3_stub_gen::derive::gen_stub_pyclass)]
        #[cfg_attr(feature = "python", ::pyo3::pyclass(eq, get_all, set_all, module = "scamplepy.query"))]
        #[derive(Default, ::bon::Builder, ::scamplers_macros::Jsonify, ::scamplers_macros::WasmJsonify, ::scamplers_macros::PyJsonify)]
        #[serde(default)]
        #[builder(on(_, into), derive(Clone, Debug, Into))]
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
pub fn db_selection(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let struct_item = parse_macro_input!(input as ItemStruct);

    let output = quote! {
        #[::scamplers_macros::base_model]
        #[cfg_attr(target_arch = "wasm32", ::wasm_bindgen::prelude::wasm_bindgen(getter_with_clone, readonly))]
        #[cfg_attr(feature = "python", ::pyo3_stub_gen::derive::gen_stub_pyclass)]
        #[cfg_attr(feature = "python", ::pyo3::pyclass(get_all, eq, module = "scamplepy.responses"))]
        #[derive(::scamplers_macros::Jsonify, ::scamplers_macros::WasmJsonify, ::scamplers_macros::PyJsonify)]
        #[cfg_attr(feature = "app", derive(::diesel::Identifiable, ::diesel::HasQuery), diesel(check_for_backend(::diesel::pg::Pg)))]
        #struct_item
    };

    output.into()
}

#[proc_macro_attribute]
pub fn db_update(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let struct_item = parse_macro_input!(input as ItemStruct);

    let mut is_update_impl = Vec::with_capacity(struct_item.fields.len());
    for field in &struct_item.fields {
        let field_ident = field.ident.as_ref().unwrap();
        if field_ident == "id" {
            continue;
        }

        is_update_impl.push(quote! { self.#field_ident.is_some() });
    }

    let name = &struct_item.ident;
    let is_update_impl = quote! {
        impl #name {
            fn is_update(&self) -> bool {
                #(#is_update_impl) || *
            }
        }
    };

    let output = quote! {
        #[::scamplers_macros::base_model]
        #[cfg_attr(feature = "python", ::pyo3_stub_gen::derive::gen_stub_pyclass)]
        #[cfg_attr(feature = "python", ::pyo3::pyclass(get_all, set_all, eq, module = "scamplepy.update"))]
        #[cfg_attr(
            feature = "app",
            derive(::diesel::AsChangeset, ::diesel::Identifiable),
            diesel(check_for_backend(::diesel::pg::Pg))
        )]
        #[derive(Default, ::scamplers_macros::Jsonify, ::scamplers_macros::PyJsonify, ::bon::Builder)]
        #[builder(on(_, into), derive(Clone, Debug, Into))]
        #struct_item

        #is_update_impl
    };

    output.into()
}

#[proc_macro_attribute]
pub fn db_simple_enum(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let enum_item = parse_macro_input!(input as ItemEnum);

    let ItemEnum { ident, .. } = &enum_item;

    let output = quote! {
        #[::scamplers_macros::base_model]
        #[cfg_attr(feature = "python", ::pyo3_stub_gen::derive::gen_stub_pyclass_enum)]
        #[cfg_attr(feature = "python", ::pyo3::pyclass(eq))]
        #[derive(Copy, ::strum::EnumString, ::strum::IntoStaticStr)]
        #[cfg_attr(feature = "app", derive(::diesel::deserialize::FromSqlRow, ::diesel::expression::AsExpression))]
        #[cfg_attr(feature = "app", diesel(sql_type = ::diesel::sql_types::Text))]
        #[serde(rename_all = "snake_case")]
        #[strum(serialize_all = "snake_case")]
        #enum_item

        #[cfg(feature = "app")]
        impl ::diesel::deserialize::FromSql<::diesel::sql_types::Text, ::diesel::pg::Pg> for #ident {
            fn from_sql(bytes: <::diesel::pg::Pg as ::diesel::backend::Backend>::RawValue<'_>) -> ::diesel::deserialize::Result<Self> {
                use ::diesel::{deserialize::FromSql, sql_types, pg::Pg};
                use ::std::str::FromStr;

                let string: String = FromSql::<sql_types::Text, Pg>::from_sql(bytes)?;
                Ok(Self::from_str(&string).unwrap())
            }
        }

        #[cfg(feature = "app")]
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
    let item = parse_macro_input!(input as Item);

    let (Item::Struct(ItemStruct { ident, .. }) | Item::Enum(ItemEnum { ident, .. })) = &item
    else {
        panic!("this macro can only be used with structs and enums")
    };

    let output = quote! {
        #[::scamplers_macros::base_model]
        #[cfg_attr(feature = "python", ::pyo3::pyclass(get_all, eq))]
        #[derive(::scamplers_macros::Jsonify, ::scamplers_macros::PyJsonify)]
        #[cfg_attr(feature = "app", derive(::diesel::deserialize::FromSqlRow, ::diesel::expression::AsExpression))]
        #[cfg_attr(feature = "app", diesel(sql_type = ::diesel::sql_types::Jsonb))]
        #item

        #[cfg(feature = "app")]
        impl ::diesel::deserialize::FromSql<::diesel::sql_types::Jsonb, ::diesel::pg::Pg> for #ident {
            fn from_sql(bytes: <::diesel::pg::Pg as ::diesel::backend::Backend>::RawValue<'_>) -> ::diesel::deserialize::Result<Self> {
                use ::diesel::{deserialize::FromSql, sql_types, pg::Pg};

                let json: ::serde_json::Value = FromSql::<sql_types::Jsonb, Pg>::from_sql(bytes)?;
                Ok(::serde_json::from_value(json).unwrap())
            }
        }

        #[cfg(feature = "app")]
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
