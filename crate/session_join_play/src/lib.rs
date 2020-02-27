#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides logic for the network play join process.

pub use crate::system::{
    SessionJoinAcceptedSystem, SessionJoinAcceptedSystemDesc, SessionJoinNetListenerSystem,
    SessionJoinNetListenerSystemDesc, SessionJoinRequestSystem, SessionJoinRequestSystemDesc,
};

mod system;
