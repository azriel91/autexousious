#![recursion_limit = "128"]

//! Provides the `#[logic_clock]` attribute to generate a newtype around
//! `LogicClock`.
//!
//! See the `logic_clock` crate for example usage.

// TODO: Test using `compiletest_rs`.
//
// At the time of writing, we cannot test using `compiletest_rs` as it does not
// appear to be able to link to external crates, so it never resolves
// `derive_more` as a dependency.

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_roids::{DeriveInputExt, FieldsUnnamedAppend};
use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput, FieldsUnnamed};

#[proc_macro_attribute]
pub fn logic_clock(_args: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as DeriveInput);

    derive_append(&mut ast);
    fields_append(&mut ast);

    let struct_name = &ast.ident;
    let constructor_doc = format!("Returns a new `{}`.", struct_name);
    let constructor2_doc = format!("Returns a new `{}` with an initial value.", struct_name);

    let token_stream2 = quote! {
        #ast

        impl #struct_name {
            #[doc = #constructor_doc]
            pub fn new(limit: usize) -> Self {
                Self(logic_clock::LogicClock::new(limit))
            }

            #[doc = #constructor2_doc]
            pub fn new_with_value(limit: usize, value: usize) -> Self {
                Self(logic_clock::LogicClock::new_with_value(limit, value))
            }
        }

        impl AsRef<logic_clock::LogicClock> for #struct_name {
            fn as_ref(&self) -> &logic_clock::LogicClock {
                &self.0
            }
        }
    };

    TokenStream::from(token_stream2)
}

fn derive_append(ast: &mut DeriveInput) {
    let derives = parse_quote!(
        Clone,
        Component,
        Copy,
        Debug,
        Default,
        Deref,
        DerefMut,
        Deserialize,
        From,
        Hash,
        PartialEq,
        Eq,
        Serialize,
    );

    ast.append_derives(derives);
}

fn fields_append(ast: &mut DeriveInput) {
    let fields: FieldsUnnamed = parse_quote! {(pub logic_clock::LogicClock)};
    ast.append_unnamed(fields);
}
