use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote};
use serde_derive_internals::ast::{Data, Field, Style};

use crate::Container;

pub struct SchemaExpr {
    pub creator: TokenStream,
}

impl ToTokens for SchemaExpr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { creator } = self;

        tokens.extend(quote!({
            #creator
        }));
    }
}

pub fn expr_for_container(cont: &Container) -> TokenStream {
    match cont.data() {
        Data::Struct(Style::Unit, _) => todo!(),
        Data::Struct(Style::Newtype, fields) => todo!(),
        Data::Struct(Style::Tuple, fields) => todo!(),
        Data::Struct(Style::Struct, fields) => expr_for_struct(cont, fields),
        Data::Enum(variants) => todo!(),
    }
}

fn expr_for_struct(cont: &Container, fields: &[Field]) -> TokenStream {
    quote! {
        let fields = std::collections::BTreeMap::new();

        rust_schema2::RustSchema {
            description: None,
            default: None,
            kind: rust_schema2::RustSchemaKind::Struct("name".into(), fields),
        }
    }
}
