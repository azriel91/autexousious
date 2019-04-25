#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides the state where Map selection takes place.

pub(crate) use crate::system::MapSelectionSystem;
pub use crate::{
    map_selection_bundle::MapSelectionBundle,
    map_selection_state::{MapSelectionState, MapSelectionStateBuilder, MapSelectionStateDelegate},
    map_selection_status::MapSelectionStatus,
};

mod map_selection_bundle;
mod map_selection_state;
mod map_selection_status;
mod system;
