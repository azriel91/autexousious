#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! State where the player joins an existing hosted game session.

pub use crate::network_join_state::{
    NetworkJoinState, NetworkJoinStateBuilder, NetworkJoinStateDelegate,
};

mod network_join_state;
