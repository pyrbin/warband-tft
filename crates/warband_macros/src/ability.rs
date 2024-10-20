use super::CRATE_IDENT;
use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::{Data, DeriveInput, Ident, PathArguments, Type};

pub(super) fn impl_ability_action_derive(ast: &DeriveInput) -> TokenStream {
    const ACTION_ATTRIBUTE_IDENT: &str = "ability_action";
    const ACTION_PROP_TYPE_IDENT: &str = "Prop";

    let name = &ast.ident;
    let crate_ident = match crate_name(CRATE_IDENT)
        .unwrap_or_else(|_| panic!("expected {CRATE_IDENT:?} is present in `Cargo.toml`"))
    {
        FoundCrate::Itself => quote!(crate::ability::action),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!( #ident::ability::action )
        }
    };

    let generics = &ast.generics;
    let vis = &ast.vis;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields = if let Data::Struct(data) = &ast.data {
        &data.fields
    } else {
        panic!("AbilityAction can only be derived for structs.");
    };

    let mut action_fn_name: Option<Ident> = None;
    for attr in &ast.attrs {
        if attr.path().is_ident(ACTION_ATTRIBUTE_IDENT) {
            action_fn_name = crate::util::ident_from_attr(attr);
        }
    }

    let mut prop_fields = Vec::new();
    let mut non_prop_fields = Vec::new();

    // TODO: support tuple structs
    for field in fields.iter() {
        if let Type::Path(type_path) = &field.ty {
            if let Some(segment) = type_path.path.segments.first() {
                // TODO: gotta be a nicer say to detect this?
                if segment.ident == ACTION_PROP_TYPE_IDENT {
                    if let PathArguments::AngleBracketed(args) = &segment.arguments {
                        if let Some(syn::GenericArgument::Type(arg_type)) = args.args.first() {
                            let ident = field
                                .ident
                                .clone()
                                .expect("field should have an identifier, tuples are not supports");
                            prop_fields.push((ident, arg_type.clone()));
                        }
                    }
                } else {
                    let ident = field
                        .ident
                        .clone()
                        .expect("field should have an identifier, tuples are not supports");
                    let ty = &field.ty;
                    non_prop_fields.push((ident, ty.clone()));
                }
            }
        }
    }

    let action_fn_name =
        action_fn_name.expect("no action system found, usage: #[ability_action({system})]");
    let action_data_name = Ident::new(&format!("{name}ActionData"), Span::call_site());
    let action_fn_data_name =
        Ident::new(&format!("{action_fn_name}_extract_data"), Span::call_site());

    let action_data_prop_fields = prop_fields.iter().map(|(ident, ty)| {
        quote! { pub(crate) #ident: #ty }
    });

    let action_data_fields = non_prop_fields.iter().map(|(ident, ty)| {
        quote! { pub(crate) #ident: #ty }
    });

    let action_data_from_fields = non_prop_fields.iter().map(|(ident, _)| {
        quote! {
            #ident: value.#ident.clone()
        }
    });

    let prop_extraction = prop_fields.iter().map(|(ident, _)| {
        quote! {
            {
                let #ident = match action.#ident {
                    Prop::Ability => or_return!(input, #ident.get(input.event.ability())),
                    Prop::Parent => or_return!(input, #ident.get(parent.get())),
                    Prop::This => or_return!(input, #ident.get(input.entity)),
                    Prop::Caster => or_return!(input, #ident.get(input.event.caster())),
                    Prop::Target => {
                        if let AbilityTarget::Entity(target) = input.target {
                            or_return!(input, #ident.get(target))
                        } else {
                            return input;
                        }
                    }
                    Prop::_Marker(_) => unreachable!(),
                };
                input.data.#ident = #ident.clone();
            }
        }
    });

    let prop_query_args = prop_fields.iter().map(|(ident, ty)| {
        quote! {
            #ident: Query<&#ty>,
        }
    });

    let expanded = quote! {
        #[derive(Clone, Default, Reflect)]
        #[automatically_derived]
        #vis struct #action_data_name #ty_generics #where_clause {
            #(#action_data_prop_fields,)*
            #(#action_data_fields,)*
        }

        #[automatically_derived]
        impl #impl_generics From<#name #ty_generics> for #action_data_name #ty_generics #where_clause {
            fn from(value: #name #ty_generics) -> Self {
                Self {
                    #(#action_data_from_fields,)*
                    ..Default::default()
                }
            }
        }

        #[automatically_derived]
        impl #impl_generics #crate_ident::AbilityAction for #name #ty_generics #where_clause {
            type Provider = #crate_ident::ActionSystemId<Self>;
            type Data = #action_data_name #ty_generics;
        }

        #[automatically_derived]
        impl #impl_generics FromWorld for #crate_ident::ActionSystemId<#name #ty_generics> #where_clause {
            fn from_world(world: &mut World) -> Self {
                Self {
                    id: world.register_system(#action_fn_data_name.pipe(#action_fn_name)),
                }
            }
        }

        fn #action_fn_data_name #impl_generics (
            input: In<#crate_ident::ActionInput<#name #ty_generics>>,
            parent: Query<&Parent>,
            action: Query<&#crate_ident::Action<#name #ty_generics>>,
            #(#prop_query_args),*
        ) -> #crate_ident::ActionInput<#name #ty_generics> #where_clause {
            let mut input = input.clone();
            let parent = or_return!(input, parent.get(input.entity));
            let action = or_return!(input, action.get(input.entity));
            let action = action.action();

            #(#prop_extraction)*

            input
        }
    };

    TokenStream::from(expanded)
}

// // Create DamageActionData struct
// let action_data_fields = prop_fields.iter().map(|(ident, ty)| {
//     quote! {
//         pub(crate) #ident: #ty
//     }
// });

// // Implement From<Damage> for DamageActionData
// let action_data_from_impl = prop_fields.iter().map(|(ident, _)| {
//     quote! {
//         #ident: value.#ident.value().clone(),
//     }
// });

// // Now we generate the `damage_extract_data` function's logic for Prop fields
// let prop_extraction = prop_fields.iter().map(|(ident, _)| {
//     quote! {
//         let #ident = match action.action().#ident {
//             Prop::Ability => or_return!(input, #ident.get(input.event.ability())),
//             Prop::Parent => or_return!(input, #ident.get(parent.get())),
//             Prop::This => or_return!(input, #ident.get(input.entity)),
//             Prop::Caster => or_return!(input, #ident.get(input.event.caster())),
//             _ => unreachable!(),
//         };
//         input.data.#ident = #ident.clone();
//     }
// });

// let action_data_name = quote! { #name ActionData };

// let action_fn_name_extract_data = quote! { #action_fn_name _extract_data };

// ----------------

// impl From<#name> for #action_data_name {
//     fn from(value: #name) -> Self {
//         Self {
//             #(#action_data_from_impl)*
//             #(#non_prop_fields: value.#non_prop_fields),*,
//         }
//     }
// }

// impl AbilityAction for #name {
//     type Provider = ActionSystemId<Self>;
//     type Data = #name ActionData;
// }

// impl FromWorld for ActionSystemId<#name> {
//     fn from_world(world: &mut World) -> Self {
//         Self {
//             id: world.register_system(#action_fn_name_extract_data.pipe(#action_fn_name)),
//         }
//     }
// }

// fn #action_fn_name_extract_data(
//     input: In<ActionInput<#name>>,
//     parent: Query<&Parent>,
//     action: Query<&Action<#name>>,
//     #(#prop_fields),*
// ) -> ActionInput<#name> {
//     let mut input = input.clone();
//     let parent = or_return!(input, parent.get(input.entity));
//     let action = or_return!(input, action.get(input.entity));

//     #(#prop_extraction)*

//     input
// }
