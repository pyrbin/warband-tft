use super::CRATE_IDENT;
use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::{Ident, ItemFn};

pub fn impl_spawner(spawner_ty: &syn::Path, input: &ItemFn) -> TokenStream {
    let fn_name = &input.sig.ident;

    let crate_ident = match crate_name(CRATE_IDENT)
        .unwrap_or_else(|_| panic!("expected {CRATE_IDENT:?} is present in `Cargo.toml`"))
    {
        FoundCrate::Itself => quote!(crate::core::spawn),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!( #ident::core::spawn )
        }
    };

    let expanded = quote! {
        #input

        impl FromWorld for #crate_ident::SpawnSystemId<#spawner_ty> {
            fn from_world(world: &mut World) -> Self {
                Self {
                    id: world.register_system(#fn_name),
                }
            }
        }
    };

    TokenStream::from(expanded)
}
