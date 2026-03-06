use std::borrow::Cow;

use serde_derive_internals::{Ctxt, Derive, ast as serde_ast, attr};

pub struct Container<'a> {
    pub cont: serde_ast::Container<'a>,
}

impl<'a> Container<'a> {
    pub fn new(input: &'a syn::DeriveInput) -> Self {
        let ctxt = Ctxt::new();

        let cont = serde_ast::Container::from_ast(&ctxt, &input, Derive::Deserialize).unwrap();

        ctxt.check().unwrap();

        Self { cont }
    }

    pub fn name(&self) -> Cow<'_, str> {
        self.cont.attrs.name().deserialize_name().into()
    }

    pub fn ident(&self) -> &syn::Ident {
        &self.cont.ident
    }

    pub fn generics(&self) -> &syn::Generics {
        &self.cont.generics
    }
}
