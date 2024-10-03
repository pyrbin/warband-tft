use super::CRATE_IDENT;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::DeriveInput;

pub(super) fn impl_spell_effect_derive(ast: &DeriveInput) -> TokenStream {
    let crate_ident = match crate_name(CRATE_IDENT)
        .unwrap_or_else(|_| panic!("expected {CRATE_IDENT:?} is present in `Cargo.toml`"))
    {
        FoundCrate::Itself => quote!(crate::spells::effect),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!( #ident::spells::effect )
        }
    };

    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let cast = quote! {
            let system_id = world.register_system(#name::cast);
            world.observe(
                move |trigger: Trigger<SpellCastEvent>,
                        effect: Query<Entity, With<#name>>,
                        mut commands: Commands| {
                    let entity = trigger.entity();
                    let _ = or_return!(effect.get(entity));
                    commands.run_system_with_input(system_id, (trigger.event().clone(), #crate_ident::Spell(entity)));
                },
            );
    };

    let impact = quote! {
            let system_id = world.register_system(#name::impact);
            world.observe(
                move |trigger: Trigger<SpellImpactEvent>,
                      effect: Query<Entity, With<#name>>,
                      mut commands: Commands| {
                    let entity = trigger.entity();
                    let _ = or_return!(effect.get(entity));
                    commands.run_system_with_input(system_id, (trigger.event().clone(), #crate_ident::Spell(entity)));
                },
            );
    };

    let gen = quote! {
        impl #impl_generics #crate_ident::SpellEffect for #name #ty_generics #where_clause {
            fn configure(app: &mut App) {
                let world = app.world_mut();
                #cast
                #impact
            }
        }
    };

    gen.into()
}
