#![recursion_limit = "128"]

//! Provides the `#[component_sequence]` attribute to generate a newtype around `Vec<C>`.
//!
//! # Examples
//!
//! ```rust,edition2018
//! # use amethyst::ecs::{storage::VecStorage, Component};
//! #
//! # #[derive(Clone, Copy, Debug, Default, PartialEq)]
//! # pub struct Wait;
//! #
//! # impl Component for Wait {
//! #     type Storage = VecStorage<Self>;
//! # }
//! #
//! use sequence_model_derive::component_sequence;
//!
//! #[component_sequence(Wait, copy)]
//! pub struct WaitSequence;
//! ```
//!
//! Effectively generates the following:
//!
//! ```rust,ignore
//! use amethyst::assets::Handle;
//! use asset_derive::Asset;
//! use derive_deref::{Deref, DerefMut};
//! use sequence_model_spi::loaded::{ComponentSequence, ComponentSequenceExt};
//! use typename_derive::TypeName;
//!
//! #[derive(Asset, Clone, Debug, Deref, DerefMut, PartialEq, TypeName)]
//! pub struct WaitSequence(ComponentSequence<Wait>)
//!
//! impl WaitSequence {
//!     #[doc = #fn_new_doc]
//!     pub fn new(sequence: Vec<Wait>) -> Self {
//!         WaitSequence(ComponentSequence::<Wait>::new(sequence))
//!     }
//! }
//!
//! // Manually implement `Default` because the component type may not, and the `Default` derive
//! // imposes a `Default` bound on type parameters.
//! impl Default for WaitSequence {
//!     fn default() -> Self {
//!         WaitSequence(ComponentSequence::<Wait>::new(Vec::default()))
//!     }
//! }
//! impl ComponentSequenceExt for #type_name {
//!     type Component = #component_path;
//!
//!     fn to_owned(component: &Self::Component) -> Self::Component {
//!         *component
//!     }
//! }
//! ```

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_roids::{DeriveInputDeriveExt, DeriveInputStructExt, FieldsUnnamedAppend};
use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput, FieldsUnnamed, Path};

use crate::component_sequence_attribute_args::ComponentSequenceAttributeArgs;

mod component_sequence_attribute_args;

#[proc_macro_attribute]
pub fn component_sequence(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as DeriveInput);
    let args = parse_macro_input!(args as ComponentSequenceAttributeArgs);
    let component_path = args.component_path;
    let to_owned_fn_impl = {
        if args.component_copy {
            quote! {
                fn to_owned(component: &Self::Component) -> Self::Component {
                    *component
                }
            }
        } else {
            let to_owned_fn = args
                .to_owned_fn
                .unwrap_or_else(|| parse_quote!(std::clone::Clone::clone));

            quote! {
                fn to_owned(component: &Self::Component) -> Self::Component {
                    #to_owned_fn(component)
                }
            }
        }
    };

    ast.assert_fields_unit();

    derive_append(&mut ast);
    fields_append(&mut ast, &component_path);

    let type_name = &ast.ident;
    let fn_new_doc = format!("Returns a new `{}`.", type_name);

    let token_stream_2 = quote! {
        #[derive(
            asset_derive::Asset,
            derive_deref::Deref,
            derive_deref::DerefMut,
            typename_derive::TypeName,
        )]
        #ast

        impl #type_name {
            #[doc = #fn_new_doc]
            pub fn new(sequence: Vec<#component_path>) -> Self {
                #type_name(
                    sequence_model_spi::loaded::ComponentSequence::<#component_path>::new(sequence)
                )
            }
        }

        // Manually implement `Default` because the component type may not, and the `Default` derive
        // imposes a `Default` bound on type parameters.
        impl Default for #type_name {
            fn default() -> Self {
                #type_name(
                    sequence_model_spi::loaded::ComponentSequence::<#component_path>::new(
                        Vec::default()
                    )
                )
            }
        }

        impl sequence_model_spi::loaded::ComponentSequenceExt for #type_name {
            type Component = #component_path;

            #to_owned_fn_impl
        }
    };

    TokenStream::from(token_stream_2)
}

fn derive_append(ast: &mut DeriveInput) {
    let derives = parse_quote!(Clone, Debug, PartialEq);

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
    let fields: FieldsUnnamed = parse_quote! {
        (#[doc = #doc_string] pub sequence_model_spi::loaded::ComponentSequence<#component_path>)
    };

    ast.append_unnamed(fields);
}
