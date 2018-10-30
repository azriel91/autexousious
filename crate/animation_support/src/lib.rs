#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides types to make it easier to work with Amethyst animations.

extern crate amethyst;
#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate derive_more;
extern crate fnv;
extern crate minterpolate;
extern crate named_type;
#[macro_use]
extern crate named_type_derive;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub use active_handle::ActiveHandle;
pub use active_handle_channel::ActiveHandleChannel;
pub use active_handle_primitive::ActiveHandlePrimitive;
pub use animation_data_set::AnimationDataSet;

mod active_handle;
mod active_handle_channel;
mod active_handle_primitive;
mod animation_data_set;
