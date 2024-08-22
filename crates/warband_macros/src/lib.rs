#![feature(concat_idents)]
mod bevy;

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn app_register_types(input: TokenStream) -> TokenStream {
    crate::bevy::app_register_types_impl(input)
}
