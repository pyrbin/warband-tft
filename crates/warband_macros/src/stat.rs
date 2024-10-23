use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::{DeriveInput, Expr, ExprLit, Fields, Lit};

use super::CRATE_IDENT;

pub(super) fn impl_stat_derive(ast: &DeriveInput) -> TokenStream {
    let crate_ident = match crate_name(CRATE_IDENT)
        .unwrap_or_else(|_| panic!("expected {CRATE_IDENT:?} is present in `Cargo.toml`"))
    {
        FoundCrate::Itself => quote!(crate::stats::stat),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!( #ident::stats::stat )
        },
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
                },
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
        },
        _ => panic!("Stat can only be derived for structs"),
    }
}

fn extract_clamp_and_round_functions(
    ast: &DeriveInput,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let mut clamp_fn: Option<proc_macro2::TokenStream> = None;
    let mut round_fn: Option<Ident> = None;

    for attr in &ast.attrs {
        if attr.path().is_ident("clamp") {
            clamp_fn = Some(extract_clamp(attr));
        } else if attr.path().is_ident("round") {
            round_fn = crate::util::ident_from_attr(attr);
        }
    }

    let clamp_fn = clamp_fn.unwrap_or_else(|| quote!());
    let round_fn = round_fn
        .map(|ident| quote!(#ident))
        .unwrap_or_else(|| quote!());

    (clamp_fn, round_fn)
}

fn extract_clamp(attr: &syn::Attribute) -> proc_macro2::TokenStream {
    let mut min_val: Option<f32> = None;
    let mut max_val: Option<f32> = None;
    let mut clamp_fn: Option<Ident> = None;

    let meta_list = attr
        .parse_args_with(syn::punctuated::Punctuated::<Expr, syn::Token![,]>::parse_terminated)
        .expect("Failed to parse attribute arguments");

    for expr in meta_list {
        if let Expr::Assign(assign) = expr {
            if let Expr::Path(path) = *assign.left {
                if path.path.is_ident("min") {
                    if let Expr::Lit(ExprLit {
                        lit: Lit::Float(flt),
                        ..
                    }) = *assign.right
                    {
                        min_val = Some(flt.base10_parse().expect("Invalid float for `min`"));
                    }
                } else if path.path.is_ident("max") {
                    if let Expr::Lit(ExprLit {
                        lit: Lit::Float(flt),
                        ..
                    }) = *assign.right
                    {
                        max_val = Some(flt.base10_parse().expect("Invalid float for `max`"));
                    }
                }
            }
        } else if let Expr::Path(path) = expr {
            if let Some(ident) = path.path.get_ident() {
                clamp_fn = Some(ident.clone());
            }
        }
    }

    match (clamp_fn, min_val, max_val) {
        (Some(func), _, _) => quote!(#func),
        (None, Some(min), Some(max)) => quote!(
            fn clamp_fn(value: f32) -> f32 {
                value.clamp(#min, #max)
            }
            clamp_fn
        ),
        (None, Some(min), None) => quote!(
            fn clamp_fn(value: f32) -> f32 {
                value.max(#min)
            }
            clamp_fn
        ),
        (None, None, Some(max)) => quote!(
            fn clamp_fn(value: f32) -> f32 {
                value.min(#max)
            }
            clamp_fn
        ),
        _ => quote!(),
    }
}
