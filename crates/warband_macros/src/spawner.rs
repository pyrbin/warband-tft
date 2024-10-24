use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

use super::CRATE_IDENT;

pub fn impl_spawner(spawner_ty: &syn::Path, input: &ItemFn) -> TokenStream {
    let fn_name = &input.sig.ident;
    let crate_ident = crate::util::crate_ident(CRATE_IDENT, "::core::spawn");

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
