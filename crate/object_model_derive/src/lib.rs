#![recursion_limit = "128"]
//! Provides the `#[derive(GameObject)]` proc_macro to implement the `GameObject` trait.
//!
//! For example usage of this macro, refer to the documentation for the `GameObject` trait.

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use crate::{
    game_object_attribute_args::GameObjectAttributeArgs, game_object_gen::game_object_gen,
};

mod game_object_attribute_args;
mod game_object_gen;
mod game_object_impl;
mod object_wrapper_gen;
mod util;

#[proc_macro_attribute]
pub fn game_object(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as GameObjectAttributeArgs);
    let ast = parse_macro_input!(item as DeriveInput);

    game_object_gen(args, ast)
}
