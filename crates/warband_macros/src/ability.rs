use darling::FromDeriveInput;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    Attribute,
    Data,
    DeriveInput,
    Fields,
    Ident,
    Path,
    PathArguments,
    Result,
    Type,
};

use super::CRATE_IDENT;

pub(super) fn impl_ability_action_derive(ast: &DeriveInput) -> TokenStream {
    const ACTION_ATTRIBUTE_IDENT: &str = "ability_action";
    const ACTION_PROP_TYPE_IDENT: &str = "Prop";

    let name = &ast.ident;
    let crate_ident = crate::util::crate_ident(CRATE_IDENT, "::ability::action");

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
                        if let Target::Entity(target) = input.target {
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

        #[automatically_derived]
        impl #impl_generics Configure for #name #ty_generics #where_clause {
            fn configure(app: &mut App) {
                #crate_ident::configure_action::<#name #ty_generics>(app);
            }
        }
    };

    TokenStream::from(expanded)
}

#[derive(FromDeriveInput)]
#[darling(attributes(ability))]
struct AbilityOpts {
    id: String,
    bundle: syn::Ident,
}

pub(super) fn impl_ability_derive(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let opts = AbilityOpts::from_derive_input(ast).expect("invalid options");
    let crate_ident = crate::util::crate_ident(CRATE_IDENT, "::ability");

    let ability_id = opts.id;
    let ability_bundle = opts.bundle;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics #crate_ident::AbilityData for #name #ty_generics #where_clause {
            const ID: #crate_ident::AbilityId = #crate_ident::AbilityId::new(#ability_id);

            fn bundle() -> impl #crate_ident::AbilityBundle {
                #ability_bundle()
            }
        }

        #[automatically_derived]
        impl #impl_generics Configure for #name #ty_generics #where_clause {
            fn configure(app: &mut App) {
                #crate_ident::configure_ability::<#name #ty_generics>(app);
            }
        }
    };

    TokenStream::from(expanded)
}

struct AbilityEventField {
    target: Path,
}

impl Parse for AbilityEventField {
    fn parse(input: ParseStream) -> Result<Self> {
        let target: Path = input.parse()?;
        Ok(AbilityEventField { target })
    }
}

pub(super) fn impl_ability_event_derive(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let crate_ident = crate::util::crate_ident(CRATE_IDENT, "::ability::event");
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields = match ast.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => &fields.named,
            _ => panic!("AbilityEvent can only be derived for structs with named fields"),
        },
        _ => panic!("AbilityEvent can only be derived for structs"),
    };

    let ability_field = fields
        .iter()
        .find(|field| {
            field
                .attrs
                .iter()
                .any(|attr: &syn::Attribute| has_ability_event_attr(attr, "ability"))
        })
        .expect("AbilityEvent struct must have a field with the ability_event(ability) attribute");

    let caster_field = fields
        .iter()
        .find(|field| {
            field
                .attrs
                .iter()
                .any(|attr: &syn::Attribute| has_ability_event_attr(attr, "caster"))
        })
        .expect("AbilityEvent struct must have a field with the ability_event(caster) attribute");

    let target_field = fields
        .iter()
        .find(|field| {
            field
                .attrs
                .iter()
                .any(|attr: &syn::Attribute| has_ability_event_attr(attr, "target"))
        })
        .expect("AbilityEvent struct must have a field with the ability_event(target) attribute");

    let ability_ident = &ability_field.ident.as_ref().unwrap();
    let caster_ident = &caster_field.ident.as_ref().unwrap();
    let target_ident = &target_field.ident.as_ref().unwrap();

    let expanded = quote! {
        #[automatically_derived]
        impl #impl_generics #crate_ident::AbilityEventType for #name #ty_generics #where_clause {
            fn ability(&self) -> Entity {
                self.#ability_ident
            }
            fn caster(&self) -> Entity {
                self.#caster_ident
            }
            fn target(&self) -> Target {
                self.#target_ident
            }
        }

        #[automatically_derived]
        impl #impl_generics Configure for #name #ty_generics #where_clause {
            fn configure(app: &mut App) {
                #crate_ident::configure_ability_event::<#name #ty_generics>(app);
            }
        }
    };

    TokenStream::from(expanded)
}

fn has_ability_event_attr(attr: &Attribute, name: &str) -> bool {
    let field = match attr.parse_args::<AbilityEventField>() {
        Ok(parsed) => parsed,
        Err(_) => {
            return false;
        },
    };

    let field = &field.target;
    let token_stream = field.to_token_stream();
    let path_str = token_stream.to_string();
    path_str == name
}
