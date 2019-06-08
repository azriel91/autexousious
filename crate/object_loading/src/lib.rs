#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Processes object configuration into the loaded object model.

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub use crate::{
    object_loader::ObjectLoader, object_loader_params::ObjectLoaderParams,
    system::ObjectDefinitionToWrapperProcessor,
};

mod object_loader;
mod object_loader_params;
mod system;
