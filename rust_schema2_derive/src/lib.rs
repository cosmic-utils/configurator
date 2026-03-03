use std::fmt::Debug;

use proc_macro2::TokenStream;
use quote::quote;
use serde_derive_internals::{Ctxt, Derive, ast as serde_ast};
use syn::parse_macro_input;

#[proc_macro_derive(RustSchema, attributes(serde))]
pub fn derive_rust_schema_wrapper(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    derive_rust_schema(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

struct Container<'a> {
    c: serde_ast::Container<'a>,
}

// --- Debug wrapper newtypes ---

struct DebugStyle(serde_ast::Style);

impl Debug for DebugStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            serde_ast::Style::Struct => write!(f, "Struct"),
            serde_ast::Style::Tuple => write!(f, "Tuple"),
            serde_ast::Style::Newtype => write!(f, "Newtype"),
            serde_ast::Style::Unit => write!(f, "Unit"),
        }
    }
}

struct DebugField<'a>(&'a serde_ast::Field<'a>);

impl<'a> Debug for DebugField<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let field = self.0;
        f.debug_struct("Field")
            .field("member", &field.member)
            .field("ty", field.ty)
            .finish()
    }
}

struct DebugVariant<'a>(&'a serde_ast::Variant<'a>);

impl<'a> Debug for DebugVariant<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.0;
        f.debug_struct("Variant")
            .field("ident", &v.ident)
            .field("style", &DebugStyle(v.style))
            .field(
                "fields",
                &v.fields.iter().map(DebugField).collect::<Vec<_>>(),
            )
            .finish()
    }
}

struct DebugData<'a>(&'a serde_ast::Data<'a>);

impl<'a> Debug for DebugData<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            serde_ast::Data::Enum(variants) => f
                .debug_tuple("Enum")
                .field(&variants.iter().map(DebugVariant).collect::<Vec<_>>())
                .finish(),
            serde_ast::Data::Struct(style, fields) => f
                .debug_tuple("Struct")
                .field(&DebugStyle(*style))
                .field(&fields.iter().map(DebugField).collect::<Vec<_>>())
                .finish(),
        }
    }
}

// --- Debug for Container ---

impl<'a> Debug for Container<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = &self.c;
        f.debug_struct("Container")
            .field("ident", &c.ident)
            .field("data", &DebugData(&c.data))
            .finish()
    }
}

fn derive_rust_schema(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let ctxt = Ctxt::new();

    let result = serde_ast::Container::from_ast(&ctxt, &input, Derive::Deserialize).unwrap();

    let res = ctxt.check();

    dbg!(&res);
    dbg!(Container { c: result });

    Ok(quote! {})
}
