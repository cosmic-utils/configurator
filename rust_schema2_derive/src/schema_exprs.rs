use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote};
use serde_derive_internals::ast::{Data, Field, Style};

use crate::{Container, GENERATOR};

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
    let fields: Vec<TokenStream> = fields
        .iter()
        .map(|field| {
            let ty = &field.ty;

            let name = field.attrs.name().deserialize_name();

            quote! {
                fields.insert(String::from(#name), #GENERATOR.schema_for::<#ty>());
            }
        })
        .collect();

    let name = cont.name();

    quote! {
        let mut fields = std::collections::BTreeMap::new();

        #(#fields)*

        rust_schema2::RustSchema {
            description: None,
            default: None,
            kind: rust_schema2::RustSchemaKind::Struct(#name.into(), fields),
        }
    }
}
