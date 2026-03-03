use std::fmt::{self, Debug};

use proc_macro2::TokenStream;
use quote::quote;
use serde_derive_internals::{Ctxt, Derive, ast as serde_ast, attr};
use syn::parse_macro_input;

use crate::container_debug::ContainerDebug;

mod container_debug;

#[proc_macro_derive(RustSchema, attributes(serde))]
pub fn derive_rust_schema_wrapper(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    derive_rust_schema(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn derive_rust_schema(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let ctxt = Ctxt::new();

    let result = serde_ast::Container::from_ast(&ctxt, &input, Derive::Deserialize).unwrap();

    let res = ctxt.check();

    //dbg!(&res);
    dbg!(ContainerDebug(result));

    Ok(quote! {})
}
