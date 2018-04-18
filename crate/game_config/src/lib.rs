#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides functionality to discover and manage game configuration.
//!
//! This crate contains the types necessary to discover game configuration from the file system; it
//! does not contain the types that represent actual configuration. Those are provided by the
//! respective configuration crates.
//!
//! For example, this crate contains the `ConfigurationIndex` type, which stores where object configuration is, but
//! does not contain `ObjectType` or `CharacterConfiguration`.

#[macro_use]
extern crate derive_more;
extern crate itertools;
#[macro_use]
extern crate log;
extern crate object_config;
#[cfg(test)]
extern crate tempfile;

pub use discovery::index_configuration;
pub use index::ConfigIndex;
pub use index::ConfigRecord;

mod config_type;
mod discovery;
mod index;
