#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Types used during network join.

pub use crate::{network_join_entity::NetworkJoinEntity, network_join_event::NetworkJoinEvent};

pub mod config;
pub mod play;

mod network_join_entity;
mod network_join_event;
