#![feature(concat_idents)]
mod bevy;
mod spell;
mod stat;

pub(crate) const CRATE_IDENT: &str = "warband_lib";

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, DeriveInput};

// Macro to register a variadic number of types with the bevy app.
#[proc_macro]
pub fn app_register_types(input: TokenStream) -> TokenStream {
    crate::bevy::app_register_types_impl(input)
}

/// Derive macro generating an impl of the trait `Stat`.
#[proc_macro_error]
#[proc_macro_derive(Stat, attributes(stat, clamp, round))]
pub fn stat_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    crate::stat::impl_stat_derive(&ast)
}

#[proc_macro_attribute]
pub fn spell_effect(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::ItemImpl);
    crate::spell::impl_spell_effect(&ast)
}

// TODO: do I need this dummy export just to make the macro work?
#[proc_macro_attribute]
pub fn on(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
