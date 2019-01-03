#![recursion_limit = "128"]
//! Provides the `#[derive(GameObject)]` proc_macro to implement the `GameObject` trait.
//!
//! For example usage of this macro, refer to the documentation for the `GameObject` trait.

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use crate::{
    game_object_attribute_args::GameObjectAttributeArgs, game_object_gen::game_object_gen,
    game_object_impl::game_object_impl,
};

mod game_object_attribute_args;
mod game_object_gen;
mod game_object_impl;
mod util;

#[proc_macro_attribute]
pub fn game_object(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as GameObjectAttributeArgs);
    let ast = parse_macro_input!(item as DeriveInput);

    game_object_gen(args, ast)
}

/// Derives `GameObject` on a struct.
#[proc_macro_derive(GameObject)]
pub fn game_object_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse::<DeriveInput>(input)
        .unwrap_or_else(|e| panic!("Failed to parse token stream. Error: {}", e));

    game_object_impl(&ast)
}
