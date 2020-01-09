#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Contains tests for all workspace crates.

#[cfg(test)]
#[macro_use]
extern crate hamcrest;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[cfg(test)]
mod application;
#[cfg(test)]
mod application_menu;
#[cfg(test)]
mod application_robot;
#[cfg(test)]
mod application_state;
#[cfg(test)]
mod application_test_support;
#[cfg(test)]
mod application_ui;
#[cfg(test)]
mod asset_gfx_gen;
#[cfg(test)]
mod asset_loading;
#[cfg(test)]
mod asset_model;
#[cfg(test)]
mod asset_play;
#[cfg(test)]
mod asset_ui_model;
#[cfg(test)]
mod audio_loading;
#[cfg(test)]
mod audio_play;
#[cfg(test)]
mod background_loading;
#[cfg(test)]
mod background_model;
#[cfg(test)]
mod camera_play;
#[cfg(test)]
mod character_loading;
#[cfg(test)]
mod character_model;
#[cfg(test)]
mod character_play;
#[cfg(test)]
mod character_prefab;
#[cfg(test)]
mod character_selection;
#[cfg(test)]
mod character_selection_stdio;
#[cfg(test)]
mod character_selection_ui;
#[cfg(test)]
mod character_selection_ui_model;
#[cfg(test)]
mod charge_play;
#[cfg(test)]
mod chase_play;
#[cfg(test)]
mod collision_audio_loading;
#[cfg(test)]
mod collision_audio_play;
#[cfg(test)]
mod collision_loading;
#[cfg(test)]
mod collision_model;
#[cfg(test)]
mod collision_play;
#[cfg(test)]
mod debug_util_amethyst;
#[cfg(test)]
mod energy_loading;
#[cfg(test)]
mod energy_model;
#[cfg(test)]
mod energy_prefab;
#[cfg(test)]
mod game_input;
#[cfg(test)]
mod game_input_model;
#[cfg(test)]
mod game_input_stdio;
#[cfg(test)]
mod game_input_ui;
#[cfg(test)]
mod game_loading;
#[cfg(test)]
mod game_mode_selection;
#[cfg(test)]
mod game_mode_selection_stdio;
#[cfg(test)]
mod game_mode_selection_ui;
#[cfg(test)]
mod game_play;
#[cfg(test)]
mod game_play_hud;
#[cfg(test)]
mod game_play_stdio;
#[cfg(test)]
mod input_reaction_loading;
#[cfg(test)]
mod input_reaction_model;
#[cfg(test)]
mod input_reaction_play;
#[cfg(test)]
mod kinematic_loading;
#[cfg(test)]
mod kinematic_model;
#[cfg(test)]
mod loading;
#[cfg(test)]
mod logic_clock;
#[cfg(test)]
mod map_loading;
#[cfg(test)]
mod map_model;
#[cfg(test)]
mod map_play;
#[cfg(test)]
mod map_selection;
#[cfg(test)]
mod map_selection_stdio;
#[cfg(test)]
mod map_selection_ui;
#[cfg(test)]
mod object_loading;
#[cfg(test)]
mod object_model;
#[cfg(test)]
mod object_play;
#[cfg(test)]
mod object_status_play;
#[cfg(test)]
mod parent_play;
#[cfg(test)]
mod sequence_loading;
#[cfg(test)]
mod sequence_play;
#[cfg(test)]
mod spawn_loading;
#[cfg(test)]
mod spawn_model;
#[cfg(test)]
mod spawn_play;
#[cfg(test)]
mod sprite_loading;
#[cfg(test)]
mod sprite_model;
#[cfg(test)]
mod sprite_play;
#[cfg(test)]
mod state_play;
#[cfg(test)]
mod stdio_command_stdio;
#[cfg(test)]
mod stdio_input;
#[cfg(test)]
mod team_model;
#[cfg(test)]
mod test_object_model;
#[cfg(test)]
mod tracker;
#[cfg(test)]
mod ui_audio_loading;
#[cfg(test)]
mod ui_loading;
#[cfg(test)]
mod ui_model;
