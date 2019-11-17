use amethyst::{core::EventReader, derive::EventReader, ecs::*, shrev::EventChannel};
use character_selection_model::CharacterSelectionEvent;
use control_settings_model::ControlSettingsEvent;
use derivative::Derivative;
use derive_more::From;
use game_input_model::ControlInputEvent;
use game_mode_selection_model::GameModeSelectionEvent;
use game_play_model::GamePlayEvent;
use map_selection_model::MapSelectionEvent;
use stdio_command_model::StdioCommandEvent;
use strum_macros::{Display, EnumDiscriminants, EnumIter, EnumString};
use winit::Event;

/// Type encompassing all state event types.
#[derive(Clone, Derivative, Display, EnumDiscriminants, EventReader, From)]
#[derivative(Debug, PartialEq = "feature_allow_slow_enum")]
#[strum_discriminants(
    name(AppEventVariant),
    derive(Display, EnumIter, EnumString),
    strum(serialize_all = "snake_case")
)]
#[reader(AppEventReader)]
pub enum AppEvent {
    /// `character_selection` events.
    CharacterSelection(CharacterSelectionEvent),
    /// `control_input` events.
    ///
    /// Note: This is defined in the `game_input*` crates.
    ControlInput(ControlInputEvent),
    /// `control_settings` events.
    ControlSettings(ControlSettingsEvent),
    /// `game_mode_selection` events.
    GameModeSelection(GameModeSelectionEvent),
    /// `game_play` events.
    GamePlay(GamePlayEvent),
    /// `map_selection` events.
    MapSelection(MapSelectionEvent),
    /// `stdio_command` events.
    StdioCommand(StdioCommandEvent),
    /// Events sent by the winit window.
    Window(Event),
}
