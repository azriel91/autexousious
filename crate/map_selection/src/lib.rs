#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides the state where Map selection takes place.

#[macro_use]
extern crate log;

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
