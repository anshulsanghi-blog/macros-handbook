use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

pub(crate) fn log_duration_impl(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input as `ItemFn` which is a type provided
    // by `syn` to represent a function.
    let input = parse_macro_input!(input as ItemFn);

    let ItemFn {
        // The function signature
        sig,
        // The visibility specifier of this function
        vis,
        // The function block or body
        block,
        // Other attributes applied to this function
        attrs,
    } = input;

    // Extract statements in the body of the functions
    let statements = block.stmts;
    
    // Store the function identifier for logging
    let function_identifier = sig.ident.clone();

    // Reconstruct the function as output using parsed input
    quote!(
        #(
            #attrs
        )*
        #vis #sig {
            // At the beginning of the function, create an instance of `Instant`
            let __start = std::time::Instant::now();
            
            // Create a new block, the body of which is the body of the function.
            // Store the return value of this block as a variable so that we can
            // return it later from the parent function.
            let __result = {
                #(#statements)*
            };

            // Log the duration information for this function
            println!("{} took {}μs", stringify!(#function_identifier), __start.elapsed().as_micros());

            // Return the result (if any)
            return __result;
        }
    )
    .into()
}
