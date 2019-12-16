#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Processes input reaction configuration into the loaded model.

pub use crate::{
    input_reaction_loading_bundle::InputReactionLoadingBundle, irs_loader::IrsLoader,
    irs_loader_params::IrsLoaderParams,
};

mod input_reaction_loading_bundle;
mod irs_loader;
mod irs_loader_params;
