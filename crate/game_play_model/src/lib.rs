#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Types used during game play.

pub use crate::{
    game_play_entity::GamePlayEntity, game_play_event::GamePlayEvent,
    game_play_event_args::GamePlayEventArgs, game_play_status::GamePlayStatus,
};

pub mod play;

mod game_play_entity;
mod game_play_event;
mod game_play_event_args;
mod game_play_status;
