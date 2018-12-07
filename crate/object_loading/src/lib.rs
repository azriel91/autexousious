#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Processes object configuration into the loaded object model.







#[macro_use]
extern crate derive_new;


#[macro_use]
extern crate log;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;





pub use crate::object::{CharacterLoader, ObjectLoader};
pub use crate::object_loading_bundle::ObjectLoadingBundle;

mod object;
mod object_loading_bundle;
