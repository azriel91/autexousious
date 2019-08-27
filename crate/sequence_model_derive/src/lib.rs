#![recursion_limit = "128"]

//! Provides the `#[frame_component_data]` attribute to generate a newtype around `Vec<C>`.
//!
//! # Examples
//!
//! ## Frame Component Data
//!
//! ```rust,edition2018
//! # use amethyst::ecs::{storage::VecStorage, Component};
//! # use specs_derive::Component;
//! #
//! # #[derive(Clone, Component, Copy, Debug, Default, PartialEq)]
//! # #[storage(VecStorage)]
//! # pub struct Wait;
//! #
//! use sequence_model_derive::frame_component_data;
//!
//! #[frame_component_data(Wait, copy)]
//! pub struct WaitSequence;
//! ```
//!
//! Effectively generates the following:
//!
//! ```rust,ignore
//! use amethyst::assets::Handle;
//! use asset_derive::Asset;
//! use derive_deref::{Deref, DerefMut};
//! use sequence_model_spi::loaded::{FrameComponentData, ComponentDataExt};
//! use typename_derive::TypeName;
//!
//! #[derive(Asset, Clone, Debug, Deref, DerefMut, PartialEq, TypeName)]
//! pub struct WaitSequence(pub FrameComponentData<Wait>);
//!
//! impl WaitSequence {
//!     #[doc = #fn_new_doc]
//!     pub fn new(sequence: Vec<Wait>) -> Self {
//!         WaitSequence(FrameComponentData::<Wait>::new(sequence))
//!     }
//! }
//!
//! // Manually implement `Default` because the component type may not, and the `Default` derive
//! // imposes a `Default` bound on type parameters.
//! impl Default for WaitSequence {
//!     fn default() -> Self {
//!         WaitSequence(FrameComponentData::<Wait>::new(Vec::default()))
//!     }
//! }
//!
//! impl ComponentDataExt for #type_name {
//!     type Component = #component_path;
//!
//!     fn to_owned(component: &Self::Component) -> Self::Component {
//!         *component
//!     }
//! }
//! ```
//!
//! ## Sequence Component Data
//!
//! ```rust,edition2018
//! # use amethyst::ecs::{storage::VecStorage, Component};
//! # use derivative::Derivative;
//! # use sequence_model_core::config::SequenceName;
//! # use serde::{Deserialize, Serialize};
//! # use specs_derive::Component;
//! # use strum_macros::{Display, EnumString, IntoStaticStr};
//! # use typename_derive::TypeName;
//! #
//! # #[derive(Clone, Component, Copy, Debug, Default, PartialEq)]
//! # #[storage(VecStorage)]
//! # pub struct SequenceEndTransition;
//! #
//! # #[derive(
//! #     Clone,
//! #     Copy,
//! #     Debug,
//! #     Derivative,
//! #     Deserialize,
//! #     Display,
//! #     EnumString,
//! #     IntoStaticStr,
//! #     PartialEq,
//! #     Eq,
//! #     Hash,
//! #     Serialize,
//! #     TypeName,
//! # )]
//! # #[derivative(Default)]
//! # #[serde(rename_all = "snake_case")]
//! # #[strum(serialize_all = "snake_case")]
//! # pub enum MagicSequenceId {
//! #     #[derivative(Default)]
//! #     Boo,
//! # }
//! # impl SequenceName for MagicSequenceId {}
//! #
//! use sequence_model_derive::sequence_component_data;
//!
//! #[sequence_component_data(SequenceEndTransition, copy)]
//! pub struct SequenceEndTransitions;
//! ```
//!
//! Effectively generates the following:
//!
//! ```rust,ignore
//! use std::collections::HashMap;
//!
//! use amethyst::assets::Handle;
//! use asset_derive::Asset;
//! use derive_deref::{Deref, DerefMut};
//! use sequence_model_spi::loaded::{SequenceComponentData, ComponentDataExt};
//! use typename_derive::TypeName;
//!
//! #[derive(Asset, Clone, Debug, Deref, DerefMut, PartialEq, TypeName)]
//! pub struct SequenceEndTransitions(
//!     pub SequenceComponentData<SequenceEndTransition>
//! );
//!
//! impl SequenceEndTransitions {
//!     #[doc = #fn_new_doc]
//!     pub fn new(sequence: Vec<SequenceEndTransition>) -> Self {
//!         SequenceEndTransitions(
//!             SequenceComponentData::<SequenceEndTransition>::new(sequence)
//!         )
//!     }
//! }
//!
//! // Manually implement `Default` because the component type may not, and the `Default` derive
//! // imposes a `Default` bound on type parameters.
//! impl Default for SequenceEndTransitions {
//!     fn default() -> Self {
//!         SequenceEndTransitions(
//!             SequenceComponentData::<SequenceEndTransition>::new(
//!                 Vec::default()
//!             )
//!         )
//!     }
//! }
//!
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
    component_data_attribute_args::ComponentDataAttributeArgs,
    frame_component_data_impl::frame_component_data_impl,
    sequence_component_data_impl::sequence_component_data_impl, to_owned_fn_impl::to_owned_fn_impl,
};

mod component_data_attribute_args;
mod frame_component_data_impl;
mod sequence_component_data_impl;
mod to_owned_fn_impl;

#[proc_macro_attribute]
pub fn frame_component_data(args: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let args = parse_macro_input!(args as ComponentDataAttributeArgs);

    frame_component_data_impl(ast, args)
}

#[proc_macro_attribute]
pub fn sequence_component_data(args: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let args = parse_macro_input!(args as ComponentDataAttributeArgs);

    sequence_component_data_impl(ast, args)
}
