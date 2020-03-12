#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides logic for network sessions at runtime.

pub use crate::{
    session_code_generator::SessionCodeGenerator,
    system::{
        SessionMessageResponseSystem, SessionMessageResponseSystemDesc, SessionStatusNotifierSystem,
    },
};

mod session_code_generator;
mod system;
