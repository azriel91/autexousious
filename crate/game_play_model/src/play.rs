//! Contains data types used at runtime.

pub use self::{
    game_play_end_transition_delay_clock::GamePlayEndTransitionDelayClock,
    game_play_status_entity::GamePlayStatusEntity,
};

mod game_play_end_transition_delay_clock;
mod game_play_status_entity;
