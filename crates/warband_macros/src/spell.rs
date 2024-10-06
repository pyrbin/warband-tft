use super::CRATE_IDENT;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    ImplItem, ItemImpl, Path, Result,
};

// Define a helper struct to parse the #[on(EventType)] attribute
struct OnAttr {
    event: Path,
}

impl Parse for OnAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let event: Path = input.parse()?;
        Ok(OnAttr { event })
    }
}

pub(super) fn impl_spell_effect(input: &ItemImpl) -> TokenStream {
    let crate_ident = match crate_name(CRATE_IDENT)
        .unwrap_or_else(|_| panic!("expected {CRATE_IDENT:?} is present in `Cargo.toml`"))
    {
        FoundCrate::Itself => quote!(crate::spells::effect),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!( #ident::spells::effect )
        }
    };

    let name = &input.self_ty;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut system_registrations = Vec::new();

    // Iterate over each item in the impl block
    for impl_item in &input.items {
        if let ImplItem::Fn(method) = impl_item {
            for attr in &method.attrs {
                if attr.path().is_ident("on") {
                    let on_attr = match attr.parse_args::<OnAttr>() {
                        Ok(parsed) => parsed,
                        Err(err) => {
                            return TokenStream::from(err.to_compile_error());
                        }
                    };

                    let event_type = &on_attr.event;
                    let function_name = &method.sig.ident;

                    // TODO: throw error if multiple #[on(EventType)] attributes are present of same
                    // type

                    // Generate code for system registration
                    system_registrations.push(quote! {
                        let system_id = world.register_system(Self::#function_name);
                        world.observe(
                            move |trigger: Trigger<#event_type>,
                                    effect: Query<Entity, With<#name>>,
                                    mut commands: Commands| {
                                let entity = trigger.entity();
                                let event = trigger.event().clone();
                                let _ = or_return!(effect.get(entity));
                                commands.run_system_with_input(system_id, event);
                            },
                        );
                    });
                }
            }
        }
    }

    let gen = quote! {
        #input

        impl #impl_generics  #crate_ident::SpellEffectConfiguration for #name #ty_generics #where_clause {
            fn configure(app: &mut App) {
                let world = app.world_mut();
                #(#system_registrations)*
            }
        }
    };

    gen.into()
}
