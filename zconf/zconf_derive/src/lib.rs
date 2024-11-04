use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(ZConf)]
pub fn cosmic_config_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_cosmic_config_macro(&ast)
}

fn impl_cosmic_config_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    // Get the fields of the struct
    let fields = match ast.data {
        syn::Data::Struct(ref data_struct) => match data_struct.fields {
            syn::Fields::Named(ref fields) => &fields.named,
            _ => unimplemented!("Only named fields are supported"),
        },
        _ => unimplemented!("Only structs are supported"),
    };

    let setters = fields.iter().filter_map(|field| {
        let field_name = &field.ident.as_ref()?;
        let field_type = &field.ty;
        let setter_name = quote::format_ident!("set_{}", field_name);
        let doc = format!("Sets [`{name}::{field_name}`]");
        Some(quote! {
            #[doc = #doc]
            ///
            /// todo
            pub fn #setter_name(&mut self, value: #field_type) {
                ::zconf::zconf_derive_impl::write(stringify!(#field_name), &value);
                self.#field_name = value;
            }
        })
    });

    let gen = quote! {

        impl #name {

            ///
            /// todo
            pub fn init(appid: &'static str, path: std::path::PathBuf) -> Self {
                ::zconf::zconf_derive_impl::init(appid, path)
            }

            #(#setters)*
        }

    };

    gen.into()
}
