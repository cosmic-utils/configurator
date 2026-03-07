use std::borrow::Cow;

use proc_macro2::TokenStream;
use quote::quote;
use serde_derive_internals::{
    Ctxt, Derive, ast as serde_ast,
    attr::{self, Name},
};
use syn::{Attribute, Expr, ExprLit, Lit};

pub struct Container<'a> {
    pub cont: serde_ast::Container<'a>,
}

impl<'a> Container<'a> {
    pub fn new(input: &'a syn::DeriveInput) -> Self {
        let ctxt = Ctxt::new();

        let cont = serde_ast::Container::from_ast(&ctxt, input, Derive::Deserialize).unwrap();

        ctxt.check().unwrap();

        Self { cont }
    }

    pub fn name(&self) -> String {
        get_name(self.cont.attrs.name())
    }
}

pub fn get_name(name: &Name) -> String {
    name.deserialize_name().into()
}

pub fn get_description(attrs: &[Attribute]) -> TokenStream {
    let mut lines = Vec::new();

    for (i, line) in attrs
        .iter()
        .filter(|a| a.path().is_ident("doc"))
        .flat_map(|a| a.meta.require_name_value())
        .enumerate()
    {
        if let Expr::Lit(ExprLit {
            lit: Lit::Str(lit_str),
            ..
        }) = &line.value
        {
            let mut s = lit_str.value();

            // remove leading space rustdoc inserts
            if let Some(stripped) = s.strip_prefix(' ') {
                s = stripped.to_string();
            }

            lines.push(s);
        }
    }

    if lines.is_empty() {
        quote! {
            None::<String>
        }
    } else {
        let joined = lines.join("\n");
        quote! {
            Some(String::from(#joined))
        }
    }
}
