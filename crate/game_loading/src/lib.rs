#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides the `State` where loading of game entities takes place.
//!
//! This is split from the `game_play` crate as it allows the
//! `application_test_support` crate to depend on this crate and spawn objects
//! for use by other crates. The `game_play` crate can then depend on the
//! `application_test_support` crate for testing its systems.

pub use crate::{
    character_augment_status::CharacterAugmentStatus,
    game_loading_bundle::GameLoadingBundle,
    game_loading_state::GameLoadingState,
    game_loading_status::GameLoadingStatus,
    system::{
        CharacterAugmentRectifySystem, CharacterAugmentRectifySystemData,
        CharacterSelectionSpawningSystem, CharacterSelectionSpawningSystemData,
        MapSelectionSpawningSystem, MapSelectionSpawningSystemData,
    },
};

mod character_augment_status;
mod game_loading_bundle;
mod game_loading_state;
mod game_loading_status;
mod system;
