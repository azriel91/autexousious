#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides logic for the network play hosting process.

pub use crate::system::{
    SessionHostRequestSystem, SessionHostRequestSystemDesc, SessionHostResponseSystem,
    SessionHostResponseSystemDesc,
};

mod system;
