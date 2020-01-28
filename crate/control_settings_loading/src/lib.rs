#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides loading logic for control settings.

pub use crate::{
    button_to_player_index_mapper::ButtonToPlayerIndexMapper,
    control_button_to_button_mapper::ControlButtonToButtonMapper, keyboard_ui_gen::KeyboardUiGen,
};

mod button_to_player_index_mapper;
mod control_button_to_button_mapper;
mod keyboard_ui_gen;
