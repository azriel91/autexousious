#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Types and traits to allow the `stdio_view` to control a state.

use clap;
#[macro_use]
extern crate derive_error_chain;

#[macro_use]
extern crate log;

use typename;
#[macro_use]
extern crate typename_derive;

pub use crate::{
    error_kind::{Error, ErrorKind, Result},
    mapper_system::MapperSystem,
    stdin_mapper::StdinMapper,
    variant_and_tokens::VariantAndTokens,
};

mod error_kind;
mod mapper_system;
mod stdin_mapper;
mod variant_and_tokens;
