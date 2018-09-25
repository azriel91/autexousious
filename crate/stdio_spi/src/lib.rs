#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Types and traits to allow the `stdio_view` to control a state.

extern crate amethyst;
extern crate application_event;
extern crate clap;
#[macro_use]
extern crate derive_error_chain;
#[macro_use]
extern crate derive_new;
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate structopt;
extern crate typename;
#[macro_use]
extern crate typename_derive;

pub use error_kind::{Error, ErrorKind, Result};
pub use mapper_system::MapperSystem;
pub use stdin_mapper::StdinMapper;
pub use variant_and_tokens::VariantAndTokens;

mod error_kind;
mod mapper_system;
mod stdin_mapper;
mod variant_and_tokens;
