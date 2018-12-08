#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides systems that update the map during game play.

use typename;
#[macro_use]
extern crate typename_derive;

pub use crate::{
    map_animation_update_system::MapAnimationUpdateSystem, map_play_bundle::MapPlayBundle,
};

mod map_animation_update_system;
mod map_play_bundle;
