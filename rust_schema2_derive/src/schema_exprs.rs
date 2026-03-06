use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::Container;

pub struct SchemaExpr {}

impl ToTokens for SchemaExpr {
    fn to_tokens(&self, tokens: &mut TokenStream) {}
}

pub fn expr_for_container(cont: &Container) -> SchemaExpr {
    todo!()
}
