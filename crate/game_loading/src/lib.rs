#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides the `State` where loading of game entities takes place.
//!
//! This is split from the `game_play` crate as it allows the `application_test_support` crate to
//! depend on this crate and spawn objects for use by other crates. The `game_play` crate can then
//! depend on the `application_test_support` crate for testing its systems.

#[macro_use]
extern crate log;

pub use crate::{
    animation_runner::AnimationRunner,
    game_loading_state::GameLoadingState,
    multi_animation_runner::MultiAnimationRunner,
    spawn::{
        BodyAcs, CharacterComponentStorages, CharacterEntitySpawner, InteractionAcs,
        MapLayerComponentStorages, MapLayerEntitySpawner, MapSpawningResources,
        ObjectAnimationStorages, ObjectComponentStorages, ObjectSpawningResources, SpriteRenderAcs,
    },
};
pub(crate) use crate::{
    game_loading_bundle::GameLoadingBundle,
    game_loading_status::GameLoadingStatus,
    system::{CharacterSelectionSpawningSystem, MapSelectionSpawningSystem},
};

mod animation_runner;
mod game_loading_bundle;
mod game_loading_state;
mod game_loading_status;
mod multi_animation_runner;
mod spawn;
mod system;
