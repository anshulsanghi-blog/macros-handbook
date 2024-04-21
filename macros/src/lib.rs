mod cached_fn;
mod constant_string;
mod custom_model;
mod hash_mapify;
mod log_duration;

extern crate proc_macro2;

use crate::cached_fn::cached_fn_impl;
use crate::constant_string::constant_string_impl;
use crate::custom_model::derive_custom_model_impl;
use crate::hash_mapify::hash_mapify_impl;
use crate::log_duration::log_duration_impl;
use proc_macro::TokenStream;
use quote::quote;
use syn::Data;

#[proc_macro_derive(IntoStringHashMap)]
pub fn into_hash_map(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let struct_identifier = &input.ident;

    match &input.data {
        Data::Struct(syn::DataStruct { fields, .. }) => {
            let field_identifiers = fields.iter().map(|item| item.ident.as_ref().unwrap()).collect::<Vec<_>>();

            quote! {
                impl From<#struct_identifier> for std::collections::HashMap<String, String> {
                    fn from(value: #struct_identifier) -> Self {
                        let mut hash_map = std::collections::HashMap::<String, String>::new();

                        #(
                            hash_map.insert(stringify!(#field_identifiers).to_string(), String::from(value.#field_identifiers));
                        )*

                        hash_map
                    }
                }
            }
        }
        _ => unimplemented!()
    }.into()
}

#[proc_macro_derive(DeriveCustomModel, attributes(custom_model))]
pub fn derive_custom_model(item: TokenStream) -> TokenStream {
    derive_custom_model_impl(item)
}

#[proc_macro_attribute]
pub fn log_duration(args: TokenStream, item: TokenStream) -> TokenStream {
    log_duration_impl(args, item)
}

#[proc_macro_attribute]
pub fn cached_fn(args: TokenStream, item: TokenStream) -> TokenStream {
    cached_fn_impl(args, item)
}

#[proc_macro]
pub fn constant_string(item: TokenStream) -> TokenStream {
    constant_string_impl(item)
}

#[proc_macro]
pub fn hash_mapify(item: TokenStream) -> TokenStream {
    hash_mapify_impl(item)
}
