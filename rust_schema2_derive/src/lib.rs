use std::fmt::{self, Debug};

use proc_macro2::TokenStream;
use quote::quote;
use serde_derive_internals::{Ctxt, Derive, ast as serde_ast, attr};
use syn::parse_macro_input;

use container::Container;

use crate::container_debug::ContainerDebug;

use idents::GENERATOR;

mod container;
mod container_debug;
mod idents;
mod schema_exprs;

#[proc_macro_derive(RustSchema, attributes(serde))]
pub fn derive_rust_schema_wrapper(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    derive_rust_schema(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn derive_rust_schema(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let ctxt = Ctxt::new();

    let cont = Container::new(&input);

    ctxt.check().unwrap();

    let type_name = cont.ident();

    let (impl_generics, ty_generics, where_clause) = cont.generics().split_for_impl();

    dbg!(ContainerDebug(&cont.cont));

    let name = cont.name();

    let schema_id = {
        quote! {
            Some(String::from(#name))
        }
    };

    let schema_expr = schema_exprs::expr_for_container(&cont);

    Ok(quote! {
        const _: () = {

            #[automatically_derived]
            impl #impl_generics rust_schema2::RustSchemaTrait for #type_name #ty_generics #where_clause {

                fn schema_id() -> Option<String> {
                    #schema_id
                }

                fn schema(#GENERATOR: &mut schemars::SchemaGenerator) -> rust_schema2::RustSchema {
                    #schema_expr
                }


            };
        };
    })
}
