#![recursion_limit = "128"]

//! Provides the `#[component_sequence]` attribute to generate a newtype around `Vec<C>`.
//!
//! See the `component_sequence` crate for example usage.

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_roids::{DeriveInputDeriveExt, DeriveInputStructExt, FieldsUnnamedAppend, IdentExt};
use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput, FieldsUnnamed, Path};

#[proc_macro_attribute]
pub fn component_sequence(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as DeriveInput);
    let component_path = parse_macro_input!(args as Path);

    ast.assert_fields_unit();

    derive_append(&mut ast);
    fields_append(&mut ast, &component_path);

    let type_name = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();
    let handle_name = type_name.append("Handle");
    let handle_doc = format!("Handle to a {}", type_name);

    let token_stream_2 = quote! {
        #ast

        impl #impl_generics amethyst::assets::Asset for #type_name #type_generics #where_clause {
            const NAME: &'static str = concat!(
                module_path!(),
                "::",
                stringify!(#type_name),
            );

            type Data = Self;
            type HandleStorage = amethyst::ecs::storage::VecStorage<amethyst::assets::Handle<Self>>;
        }

        #[doc = #handle_doc]
        pub type #handle_name #impl_generics = amethyst::assets::Handle<#type_name #type_generics>;
    };

    TokenStream::from(token_stream_2)
}

fn derive_append(ast: &mut DeriveInput) {
    let derives = parse_quote!(Clone, Debug, Default, Deref, DerefMut, From, PartialEq, new);

    ast.append_derives(derives);
}

fn fields_append(ast: &mut DeriveInput, component_path: &Path) {
    let component_name = &component_path
        .segments
        .last()
        .expect("Expected `Path` last segment to exist.")
        .value()
        .ident;
    let doc_string = format!("The chain of `{}` values.", component_name);
    let fields: FieldsUnnamed = parse_quote! {(#[doc = #doc_string] pub Vec<#component_path>)};

    ast.append_unnamed(fields);
}
