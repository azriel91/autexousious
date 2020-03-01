#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! State where the game session host chooses to begin the session.

pub use crate::session_host_state::{
    SessionHostState, SessionHostStateBuilder, SessionHostStateDelegate,
};

mod session_host_state;
