#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides types to make it easier to work with Amethyst animations.


#[macro_use]
extern crate derive_new;



#[macro_use]
extern crate named_type_derive;

#[macro_use]
extern crate serde_derive;

pub use crate::active_handle::ActiveHandle;
pub use crate::active_handle_channel::ActiveHandleChannel;
pub use crate::active_handle_primitive::ActiveHandlePrimitive;

mod active_handle;
mod active_handle_channel;
mod active_handle_primitive;
