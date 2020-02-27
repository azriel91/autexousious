#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! State where the player joins an existing hosted game session.

pub use crate::session_join_state::{
    SessionJoinState, SessionJoinStateBuilder, SessionJoinStateDelegate,
};

mod session_join_state;
