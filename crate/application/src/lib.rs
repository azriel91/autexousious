#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//!
//!

#[macro_use]
extern crate derive_error_chain;
#[macro_use]
extern crate error_chain;
#[cfg(test)]
extern crate tempdir;

pub mod config;
