use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote};
use serde_derive_internals::ast::{Data, Field, Style};

use crate::{Container, GENERATOR, idents::STRUCT_DEFAULT};

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
    let set_container_default = match cont.cont.attrs.default() {
        serde_derive_internals::attr::Default::None => None,
        serde_derive_internals::attr::Default::Default => {
            Some(quote!(let #STRUCT_DEFAULT = Self::default();))
        }
        serde_derive_internals::attr::Default::Path(path) => {
            Some(quote!(let #STRUCT_DEFAULT = #path();))
        }
    };

    let fields: Vec<TokenStream> = fields
        .iter()
        .map(|field| {
            let ty = &field.ty;

            let name = field.attrs.name().deserialize_name();

            quote! {
                fields.insert(
                    String::from(#name),
                    rust_schema2::StructField {
                        description: None,
                        default: None,
                        schema: #GENERATOR.schema_for::<#ty>()
                    }
                );
            }
        })
        .collect();

    let name = cont.name();

    quote! {
        let mut fields = std::collections::BTreeMap::new();

        #(#fields)*

        rust_schema2::RustSchema {
            kind: rust_schema2::RustSchemaKind::Struct(
                rust_schema2::Struct {
                    name: #name.into(),
                    description: None,
                    default: None,
                    fields
                }
            ),
        }
    }
}
