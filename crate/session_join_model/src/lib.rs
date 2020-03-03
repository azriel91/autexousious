#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Types used during session join.

pub use crate::{session_join_entity::SessionJoinEntity, session_join_event::SessionJoinEvent};

pub mod config;
pub mod play;

mod session_join_entity;
mod session_join_event;
