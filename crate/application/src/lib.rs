#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//!
//!

#[macro_use]
extern crate derive_error_chain;
extern crate error_chain;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
#[cfg(test)]
extern crate tempdir;
#[cfg(test)]
extern crate tempfile;

pub mod config;
