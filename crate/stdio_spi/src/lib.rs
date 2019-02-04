#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Types and traits to allow the `stdio_view` to control a state.

pub use crate::{
    mapper_system::MapperSystem, mapper_system_data::MapperSystemData, stdin_mapper::StdinMapper,
    stdio_error::StdioError, variant_and_tokens::VariantAndTokens,
};

mod mapper_system;
mod mapper_system_data;
mod stdin_mapper;
mod stdio_error;
mod variant_and_tokens;
