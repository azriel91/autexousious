#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Processes input reaction configuration into the loaded model.

pub use crate::{irs_loader::IrsLoader, irs_loader_params::IrsLoaderParams};

mod irs_loader;
mod irs_loader_params;
