#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides functionality to access data from a server which would normally by on the file system.
//!
//! This is required for WASM support.

pub use crate::{dir_access::DirAccess, path_access_ext::PathAccessExt};

mod dir_access;
mod path_access_ext;
