#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides systems that update the map during game play.

extern crate amethyst;
#[cfg(test)]
extern crate amethyst_test;
#[macro_use]
extern crate derive_new;
extern crate map_model;
extern crate typename;
#[macro_use]
extern crate typename_derive;

pub use crate::map_animation_update_system::MapAnimationUpdateSystem;
pub use crate::map_play_bundle::MapPlayBundle;

mod map_animation_update_system;
mod map_play_bundle;
