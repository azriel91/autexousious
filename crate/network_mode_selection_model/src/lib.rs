#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Types used during network mode selection.

pub use crate::{
    network_mode_index::NetworkModeIndex,
    network_mode_selection_entity::NetworkModeSelectionEntity,
    network_mode_selection_event::NetworkModeSelectionEvent,
    network_mode_selection_event_args::NetworkModeSelectionEventArgs,
};

mod network_mode_index;
mod network_mode_selection_entity;
mod network_mode_selection_event;
mod network_mode_selection_event_args;
