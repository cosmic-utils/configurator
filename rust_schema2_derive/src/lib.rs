use std::fmt::Debug;

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_derive(RustSchema, attributes(serde))]
pub fn derive_rust_schema_wrapper(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    derive_rust_schema(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn derive_rust_schema(input: syn::DeriveInput) -> syn::Result<TokenStream> {

    dbg!(&input);

    Ok(quote! {

        
    })
}
