#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides systems that update the map during game play.

pub use crate::{
    map_animation_update_system::MapAnimationUpdateSystem, map_play_bundle::MapPlayBundle,
};

mod map_animation_update_system;
mod map_play_bundle;
