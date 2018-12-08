#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides the state where Map selection takes place.

#[macro_use]
extern crate derivative;
#[macro_use]
extern crate derive_new;

#[macro_use]
extern crate log;

use typename;
#[macro_use]
extern crate typename_derive;

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
