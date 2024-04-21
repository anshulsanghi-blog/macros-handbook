use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, LitStr};

pub fn constant_string_impl(item: TokenStream) -> TokenStream {
    // Parse input as a string literal
    let constant_value = parse_macro_input!(item as LitStr);

    // Create a new `Ident` (identifier) from the passed string value.
    // This is going to be the name of the constant variable.
    let constant_value_name = Ident::from_string(&constant_value.value()).unwrap();

    // Generate the code for declaring the constant variable.
    quote!(pub const #constant_value_name: &str = #constant_value;).into()
}
