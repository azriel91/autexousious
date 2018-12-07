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

pub use crate::map_selection_bundle::MapSelectionBundle;
pub use crate::map_selection_state::{
    MapSelectionState, MapSelectionStateBuilder, MapSelectionStateDelegate,
};
pub use crate::map_selection_status::MapSelectionStatus;
pub(crate) use crate::system::MapSelectionSystem;

mod map_selection_bundle;
mod map_selection_state;
mod map_selection_status;
mod system;
