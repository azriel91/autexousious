#![deny(missing_docs)] // kcov-ignore
#![deny(missing_debug_implementations)]

//! Behaviour logic for object types.

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub use crate::{
    character_sequence_update_components::CharacterSequenceUpdateComponents,
    character_sequence_updater::CharacterSequenceUpdater, mirrored_updater::MirroredUpdater,
    run_counter_updater::RunCounterUpdater, system::CharacterCtsHandleUpdateSystem,
};

mod character_sequence_update_components;
mod character_sequence_updater;
mod mirrored_updater;
mod run_counter_updater;
pub(crate) mod sequence_handler;
mod system;
