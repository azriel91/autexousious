#![recursion_limit = "128"]

//! Provides the `#[frame_component_data]` attribute to generate a newtype around `Vec<C>`.
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
//! pub struct WaitSequence(FrameComponentData<Wait>)
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
    frame_component_data_attribute_args::FrameComponentDataAttributeArgs,
    frame_component_data_impl::frame_component_data_impl,
};

mod frame_component_data_attribute_args;
mod frame_component_data_impl;

#[proc_macro_attribute]
pub fn frame_component_data(args: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let args = parse_macro_input!(args as FrameComponentDataAttributeArgs);

    frame_component_data_impl(ast, args)
}
