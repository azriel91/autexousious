#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Types used during game play.

pub use crate::{
    game_play_entity::GamePlayEntity, game_play_event::GamePlayEvent,
    game_play_status::GamePlayStatus,
};

mod game_play_entity;
mod game_play_event;
mod game_play_status;

pub mod play;
