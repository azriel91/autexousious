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
//! use sequence_model_spi::loaded::{ComponentSequence, ComponentDataExt};
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
//! impl ComponentDataExt for #type_name {
//!     type Component = #component_path;
//!
//!     fn to_owned(component: &Self::Component) -> Self::Component {
//!         *component
//!     }
//! }
//! ```

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use crate::{
    component_sequence_attribute_args::ComponentSequenceAttributeArgs,
    component_sequence_impl::component_sequence_impl,
};

mod component_sequence_attribute_args;
mod component_sequence_impl;

#[proc_macro_attribute]
pub fn component_sequence(args: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let args = parse_macro_input!(args as ComponentSequenceAttributeArgs);

    component_sequence_impl(ast, args)
}
