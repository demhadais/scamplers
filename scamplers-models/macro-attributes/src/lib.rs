use proc_macro::TokenStream;
use quote::quote;

fn base_derives(with_default: bool) -> proc_macro2::TokenStream {
    if with_default {
        quote! {
            #[derive(Clone, Debug, Default, PartialEq, ::serde::Deserialize, ::serde::Serialize)]
            #[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
            #[serde(default)]
        }
    } else {
        quote! {
            #[derive(Clone, Debug, PartialEq, ::serde::Deserialize, ::serde::Serialize)]
            #[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
        }
    }
}

#[proc_macro_attribute]
pub fn base_model(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let derives = base_derives(false);
    let input: proc_macro2::TokenStream = input.into();

    quote! {
        #derives
        #input
    }
    .into()
}

fn diesel_insertable() -> proc_macro2::TokenStream {
    quote! {
        #[cfg_attr(feature = "app", derive(::diesel::Insertable))]
    }
}

fn diesel_has_query() -> proc_macro2::TokenStream {
    quote! {
        #[cfg_attr(feature = "app", derive(::diesel::HasQuery))]
    }
}

fn diesel_check_for_backend() -> proc_macro2::TokenStream {
    quote! {
        #[cfg_attr(feature = "app", diesel(check_for_backend(::diesel::pg::Pg)))]
    }
}

#[proc_macro_attribute]
pub fn insert_select(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let base_derives = base_derives(false);
    let insertable = diesel_insertable();
    let has_query = diesel_has_query();
    let check_for_backend = diesel_check_for_backend();

    let input: proc_macro2::TokenStream = input.into();

    quote! {
        #base_derives
        #insertable
        #has_query
        #check_for_backend
        #input
    }
    .into()
}

#[proc_macro_attribute]
pub fn insert(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let base_derives = base_derives(false);
    let insertable = diesel_insertable();
    let check_for_backend = diesel_check_for_backend();

    let input: proc_macro2::TokenStream = input.into();

    quote! {
        #base_derives
        #insertable
        #check_for_backend
        #input
    }
    .into()
}

#[proc_macro_attribute]
pub fn query(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let base_derives = base_derives(true);

    let input: proc_macro2::TokenStream = input.into();

    quote! {
        #base_derives
        #input
    }
    .into()
}

#[proc_macro_attribute]
pub fn select(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let base_derives = base_derives(false);
    let has_query = diesel_has_query();
    let check_for_backend = diesel_check_for_backend();

    let input: proc_macro2::TokenStream = input.into();

    quote! {
        #base_derives
        #has_query
        #check_for_backend
        #input
    }
    .into()
}

#[proc_macro_attribute]
pub fn update(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let base_derives = base_derives(true);
    let check_for_backend = diesel_check_for_backend();

    let input: proc_macro2::TokenStream = input.into();

    quote! {
        #base_derives
        #[cfg_attr(feature = "app", derive(::diesel::AsChangeSet, ::diesel::Identifiable))]
        #check_for_backend
        #input
    }
    .into()
}

#[proc_macro_attribute]
pub fn simple_enum(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let base_derives = base_derives(false);

    let input: proc_macro2::TokenStream = input.into();

    quote! {
        #base_derives
        #[derive(Copy, ::strum::EnumString, ::strum::IntoStaticStr)]
        #[cfg_attr(feature = "app", derive(::diesel::deserialize::FromSqlRow, ::diesel::expression::AsExpression))]
        #[cfg_attr(feature = "app", diesel(sql_type = ::diesel::sql_types::Text))]
        #[serde(rename_all = "snake_case")]
        #[strum(serialize_all = "snake_case")]
        #input
    }.into()
}
