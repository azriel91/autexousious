#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Types used during session hosting.

pub use crate::{session_host_entity::SessionHostEntity, session_host_event::SessionHostEvent};

pub mod config;
pub mod play;

mod session_host_entity;
mod session_host_event;
