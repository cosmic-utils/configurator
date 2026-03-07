use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote};
use serde_derive_internals::ast::{Data, Field, Style, Variant};

use serde_derive_internals::attr::Default as SerdeDefault;

use crate::container::get_name;
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
        Data::Struct(Style::Unit, _) => expr_for_unit_struct(cont),
        Data::Struct(Style::Newtype, fields) => expr_for_tuple_struct(cont, fields),
        Data::Struct(Style::Tuple, fields) => expr_for_tuple_struct(cont, fields),
        Data::Struct(Style::Struct, fields) => expr_for_struct(cont, fields),
        Data::Enum(variants) => expr_for_enum(cont, variants),
    }
}

fn expr_for_enum(cont: &Container, variants: &[Variant]) -> TokenStream {
    let name = cont.name();
    let description = get_description(&cont.cont.original.attrs);

    let variants: Vec<TokenStream> = variants
        .iter()
        .map(|variant| {
            let kind = match variant.style {
                Style::Struct => todo!(),
                Style::Tuple => {
                    let fields: Vec<TokenStream> = variant
                        .fields
                        .iter()
                        .map(|field| {
                            let ty = &field.ty;
                            quote! {
                                #GENERATOR.schema_for::<#ty>()
                            }
                        })
                        .collect();

                    quote! {
                        rust_schema2::EnumVariantKind::Tuple(vec![#(#fields),*])
                    }
                }
                Style::Newtype => {
                    let field = &variant.fields[0];
                    let ty = &field.ty;

                    quote! {
                        rust_schema2::EnumVariantKind::Tuple(vec![#GENERATOR.schema_for::<#ty>()])
                    }
                }
                Style::Unit => quote! {
                    rust_schema2::EnumVariantKind::Unit
                },
            };

            let name = get_name(variant.attrs.name());
            let description = get_description(&variant.original.attrs);

            quote! {
                variants.push(rust_schema2::EnumVariant {
                    name: String::from(#name),
                    description: #description,
                    kind: #kind
                });
            }
        })
        .collect();

    quote! {

        let mut variants = Vec::new();

        #(#variants)*

        rust_schema2::RustSchema {
            kind: rust_schema2::RustSchemaKind::Enum(
                rust_schema2::Enum {
                    name: String::from(#name),
                    description: #description,
                    variants
                }
            ),
        }
    }
}

fn expr_for_unit_struct(cont: &Container) -> TokenStream {
    let name = cont.name();
    let description = get_description(&cont.cont.original.attrs);

    quote! {
        rust_schema2::RustSchema {
            kind: rust_schema2::RustSchemaKind::Struct(
                rust_schema2::Struct {
                    name: String::from(#name),
                    description: #description,
                    default: None,
                    fields: std::collections::BTreeMap::new()
                }
            ),
        }
    }
}

fn expr_for_tuple_struct(cont: &Container, fields: &[Field]) -> TokenStream {
    let name = cont.name();
    let description = get_description(&cont.cont.original.attrs);

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

            quote! {
                #GENERATOR.schema_for::<#ty>()
            }
        })
        .collect();

    quote! {

        rust_schema2::RustSchema {
            kind: rust_schema2::RustSchemaKind::TupleStruct(
                rust_schema2::TupleStruct {
                    name: String::from(#name),
                    description: #description,
                    default: #struct_default,
                    fields: vec![#(#fields),*]
                }
            ),
        }
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

            let name = get_name(field.attrs.name());

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
                    name: String::from(#name),
                    description: #description,
                    default: #struct_default,
                    fields
                }
            ),
        }
    }
}
