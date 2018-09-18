#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides the state where Map selection takes place.

extern crate amethyst;
#[cfg(test)]
extern crate amethyst_test_support;
#[cfg(test)]
extern crate asset_loading;
#[cfg(test)]
extern crate assets_test;
#[macro_use]
extern crate derivative;
#[macro_use]
extern crate derive_new;
extern crate game_model;
#[cfg(test)]
extern crate loading;
#[macro_use]
extern crate log;
#[cfg(test)]
extern crate map_loading;
extern crate map_selection_model;
extern crate typename;
#[macro_use]
extern crate typename_derive;

pub use map_selection_bundle::MapSelectionBundle;
pub use map_selection_state::{MapSelectionState, MapSelectionStateBuilder};
pub use map_selection_status::MapSelectionStatus;
pub(crate) use system::MapSelectionSystem;

mod map_selection_bundle;
mod map_selection_state;
mod map_selection_status;
mod system;
