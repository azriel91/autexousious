#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides the `State` where loading of game entities takes place.
//!
//! This is split from the `game_play` crate as it allows the `application_test_support` crate to
//! depend on this crate and spawn objects for use by other crates. The `game_play` crate can then
//! depend on the `application_test_support` crate for testing its systems.

extern crate amethyst;
#[cfg(test)]
extern crate amethyst_test;
extern crate animation_support;
extern crate application_event;
extern crate application_state;
#[cfg(test)]
extern crate asset_loading;
#[cfg(test)]
extern crate assets_test;
extern crate character_selection_model;
#[cfg(test)]
extern crate collision_loading;
extern crate collision_model;
#[macro_use]
extern crate derivative;
#[macro_use]
extern crate derive_new;
extern crate game_input;
extern crate game_model;
#[cfg(test)]
extern crate loading;
#[macro_use]
extern crate log;
#[cfg(test)]
extern crate map_loading;
extern crate map_model;
#[cfg(test)]
extern crate map_selection;
extern crate map_selection_model;
#[cfg(test)]
extern crate object_loading;
extern crate object_model;
extern crate typename;
#[macro_use]
extern crate typename_derive;

pub use crate::animation_runner::AnimationRunner;
pub(crate) use crate::game_loading_bundle::GameLoadingBundle;
pub use crate::game_loading_state::GameLoadingState;
pub(crate) use crate::game_loading_status::GameLoadingStatus;
pub use crate::multi_animation_runner::MultiAnimationRunner;
pub use crate::spawn::{
    BodyAcs, CharacterComponentStorages, CharacterEntitySpawner, InteractionAcs,
    MapLayerComponentStorages, MapLayerEntitySpawner, MapSpawningResources,
    ObjectAnimationStorages, ObjectComponentStorages, ObjectSpawningResources, SpriteRenderAcs,
};
pub(crate) use crate::system::{CharacterSelectionSpawningSystem, MapSelectionSpawningSystem};

mod animation_runner;
mod game_loading_bundle;
mod game_loading_state;
mod game_loading_status;
mod multi_animation_runner;
mod spawn;
mod system;
