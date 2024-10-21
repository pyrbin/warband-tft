#![feature(concat_idents)]

mod ability;
mod bevy;
mod spawner;
mod stat;
mod util;

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

/// Derive macro generating an impl of the trait `AbilityAction`.
#[proc_macro_error]
#[proc_macro_derive(AbilityAction, attributes(ability_action))]
pub fn ability_action_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    crate::ability::impl_ability_action_derive(&ast)
}

/// Derive macro generating an impl of an `Ability`.
#[proc_macro_error]
#[proc_macro_derive(Ability, attributes(ability))]
pub fn ability_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    crate::ability::impl_ability_derive(&ast)
}

/// Macro to specify a spawner.
#[proc_macro_attribute]
pub fn spawner(attr: TokenStream, input: TokenStream) -> TokenStream {
    let item_fn = parse_macro_input!(input as syn::ItemFn);
    let spawner_ty = parse_macro_input!(attr as syn::Path);
    crate::spawner::impl_spawner(&spawner_ty, &item_fn)
}
