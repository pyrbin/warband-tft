use super::CRATE_IDENT;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::{DeriveInput, Fields, Path};

pub(super) fn impl_stat_derive(ast: &DeriveInput) -> TokenStream {
    let crate_ident = match crate_name(CRATE_IDENT)
        .unwrap_or_else(|_| panic!("expected {CRATE_IDENT:?} is present in `Cargo.toml`"))
    {
        FoundCrate::Itself => quote!(crate::stats::stat),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!( #ident::stats::stat )
        }
    };

    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let value_field = extract_stat_value_field(ast);
    let (clamp_fn, round_fn) = extract_clamp_and_round_functions(ast);

    let gen = quote! {
        impl #impl_generics #crate_ident::Stat for #name #ty_generics #where_clause {
            fn new(value: f32) -> Self {
                Self { #value_field: value, ..Default::default() }
            }

            fn value(&self) -> f32 {
                self.#value_field
            }

            fn clamp(value: f32) -> f32 {
                #clamp_fn(value)
            }

            fn round(value: f32) -> f32 {
                #round_fn(value)
            }
        }

        impl #impl_generics Into<#name #ty_generics> for f32 #where_clause {
            fn into(self) -> #name #ty_generics {
                #name::new(self)
            }
        }

        impl #impl_generics std::ops::Deref for #name #ty_generics #where_clause {
            type Target = f32;
            fn deref(&self) -> &f32 {
                &self.#value_field
            }
        }
    };
    gen.into()
}

fn extract_stat_value_field(ast: &DeriveInput) -> proc_macro2::TokenStream {
    match &ast.data {
        syn::Data::Struct(data) => {
            match &data.fields {
                // Handle tuple structs: Assumes single f32 field
                Fields::Unnamed(fields_unnamed) if fields_unnamed.unnamed.len() == 1 => {
                    let index = syn::Index::from(0);
                    quote!(#index)
                }
                // Handle named fields that might be annotated or using single named field
                Fields::Named(fields_named) => fields_named
                    .named
                    .iter()
                    .find_map(|f| {
                        if f.attrs.iter().any(|a| a.path().is_ident("stat")) {
                            f.ident.as_ref().map(|ident| quote!(#ident))
                        } else {
                            None
                        }
                    })
                    .unwrap_or_else(|| {
                        panic!(
                            "No field marked with #[stat(value)] and structure is not a simple \
                             tuple struct"
                        )
                    }),
                _ => panic!("Stat can only be derived for structs with exactly one field"),
            }
        }
        _ => panic!("Stat can only be derived for structs"),
    }
}

fn extract_clamp_and_round_functions(
    ast: &DeriveInput,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let mut clamp_fn: Option<Ident> = None;
    let mut round_fn: Option<Ident> = None;

    for attr in &ast.attrs {
        if attr.path().is_ident("clamp") {
            clamp_fn = extract_function_from_attr(attr);
        } else if attr.path().is_ident("round") {
            round_fn = extract_function_from_attr(attr);
        }
    }

    // Default to no-op if no functions are provided
    let clamp_fn = clamp_fn
        .map(|ident| quote!(#ident))
        .unwrap_or_else(|| quote!());

    let round_fn = round_fn
        .map(|ident| quote!(#ident))
        .unwrap_or_else(|| quote!());

    (clamp_fn, round_fn)
}

fn extract_function_from_attr(attr: &syn::Attribute) -> Option<Ident> {
    attr.parse_args::<Path>()
        .ok()
        .and_then(|path| path.get_ident().cloned())
}
