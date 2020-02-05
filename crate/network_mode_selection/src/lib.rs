#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! State where network mode selection takes place.

pub use crate::{
    network_mode_selection_state::{
        NetworkModeSelectionState, NetworkModeSelectionStateBuilder,
        NetworkModeSelectionStateDelegate,
    },
    network_mode_selection_trans::NetworkModeSelectionTrans,
};

mod network_mode_selection_state;
mod network_mode_selection_trans;
