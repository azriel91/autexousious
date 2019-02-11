#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides types to make it easier to work with Amethyst animations.

pub use crate::{
    active_handle::ActiveHandle, active_handle_channel::ActiveHandleChannel,
    active_handle_primitive::ActiveHandlePrimitive, animation_runner::AnimationRunner,
    multi_animation_runner::MultiAnimationRunner,
};

mod active_handle;
mod active_handle_channel;
mod active_handle_primitive;
mod animation_runner;
mod multi_animation_runner;
