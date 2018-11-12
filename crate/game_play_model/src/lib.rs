#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Types used during game play.

extern crate amethyst;

pub use crate::{
    game_play_entity::GamePlayEntity, game_play_entity_id::GamePlayEntityId,
    game_play_event::GamePlayEvent,
};

mod game_play_entity;
mod game_play_entity_id;
mod game_play_event;
