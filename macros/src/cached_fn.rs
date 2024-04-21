use darling::ast::NestedMeta;
use darling::{Error, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, ItemFn};

#[derive(FromMeta)]
struct CachedParams {
    // Accept any expression that we should use to compute the
    // key. This can be a constant string, or some computation
    // based on function arguments.
    keygen: Option<Expr>,
}

pub fn cached_fn_impl(args: TokenStream, item: TokenStream) -> TokenStream {
    // Parse argument tokens as a list of NestedMeta items
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            // Write error to output token stream if there is one
            return proc_macro::TokenStream::from(Error::from(e).write_errors());
        }
    };

    // Parse the nested meta list as our `CachedParams` struct
    let CachedParams { keygen } = match CachedParams::from_list(&attr_args) {
        Ok(params) => params,
        Err(error) => {
            // Write error to output token stream if there is one
            return proc_macro::TokenStream::from(Error::from(error).write_errors());
        }
    };

    // Parse the input target item as a function
    let ItemFn {
        // The function signature
        sig,
        // The visibility specifier of this function
        vis,
        // The function block or body
        block,
        // Other attributes applied to this function
        attrs,
    } = parse_macro_input!(item as ItemFn);

    // Generate our key statement based on given param (or lack thereof)
    let key_statement = if let Some(keygen) = keygen {
        // If the user specified a `keygen`, use that as an expression to
        // get the cache key.
        quote! {
            let __cache_key = #keygen;
        }
    } else {
        // If no `keygen` was provided, use the name of the function
        // as cache key.
        let fn_name = sig.ident.clone().to_string();
        quote! {
            let __cache_key = #fn_name;
        }
    };

    // Reconstruct the function as output using parsed input
    quote!(
        // Apply other attributes from original function to the generated function
        #(#attrs)*
        #vis #sig {
            // Include the key_statement we generated above as the first
            // thing in the function body
            #key_statement

            // Try to read the value from cache
            match cacache::read_sync("./__cache", __cache_key.clone()) {
                // If the value exists, parse it as string and return it
                Ok(value) => {
                    println!("Data is fetched from cached");
                    from_utf8(&value).unwrap().to_string()
                },
                Err(_) => {
                    println!("Data is not fetched from cached");
                    // Save the output of original function block into
                    // a variable.
                    let output = #block;

                    // Write the output value to cache as bytes
                    cacache::write_sync("./__cache", __cache_key, output.as_bytes()).unwrap();

                    // Return the original output
                    output
                }
            }
        }
    )
    .into()
}
