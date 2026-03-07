use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote};
use serde_derive_internals::ast::{Data, Field, Style};

use serde_derive_internals::attr::Default as SerdeDefault;

use crate::{Container, GENERATOR, container::get_description, idents::STRUCT_DEFAULT};

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
    match &cont.cont.data {
        Data::Struct(Style::Unit, _) => todo!(),
        Data::Struct(Style::Newtype, fields) => todo!(),
        Data::Struct(Style::Tuple, fields) => todo!(),
        Data::Struct(Style::Struct, fields) => expr_for_struct(cont, fields),
        Data::Enum(variants) => todo!(),
    }
}

fn expr_for_struct(cont: &Container, fields: &[Field]) -> TokenStream {
    let struct_default = match cont.cont.attrs.default() {
        SerdeDefault::None => {
            quote!(None)
        }
        SerdeDefault::Default => {
            quote!(Some(rust_schema2::to_value(Self::default())))
        }
        SerdeDefault::Path(path) => {
            quote!(Some(rust_schema2::to_value(#path())))
        }
    };

    let fields: Vec<TokenStream> = fields
        .iter()
        .map(|field| {
            let ty = &field.ty;

            let name = field.attrs.name().deserialize_name();

            let description = get_description(&field.original.attrs);

            let field_default = match field.attrs.default() {
                SerdeDefault::None => {
                    quote!(None)
                }
                SerdeDefault::Default => {
                    quote!(Some(rust_schema2::to_value(#ty::default())))
                }
                SerdeDefault::Path(path) => {
                    quote!(Some(rust_schema2::to_value(#path())))
                }
            };

            quote! {
                fields.insert(
                    String::from(#name),
                    rust_schema2::StructField {
                        description: #description,
                        default: #field_default,
                        schema: #GENERATOR.schema_for::<#ty>()
                    }
                );
            }
        })
        .collect();

    let name = cont.name();

    let description = get_description(&cont.cont.original.attrs);

    quote! {
        let mut fields = std::collections::BTreeMap::new();

        #(#fields)*

        rust_schema2::RustSchema {
            kind: rust_schema2::RustSchemaKind::Struct(
                rust_schema2::Struct {
                    name: #name.into(),
                    description: #description,
                    default: #struct_default,
                    fields
                }
            ),
        }
    }
}
