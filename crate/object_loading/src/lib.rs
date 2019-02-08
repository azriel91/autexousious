#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Processes object configuration into the loaded object model.

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub use crate::{
    object::{CharacterLoader, ObjectLoader, ObjectLoaderParams},
    system::ObjectDefinitionToWrapperProcessor,
};

mod object;
mod system;
