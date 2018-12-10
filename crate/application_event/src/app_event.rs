use amethyst::{
    core::EventReader, derive::EventReader, ecs::*, renderer::Event, shrev::EventChannel,
};
use character_selection_model::CharacterSelectionEvent;
use derivative::Derivative;
use derive_more::From;
use game_mode_selection_model::GameModeSelectionEvent;
use game_play_model::GamePlayEvent;
use map_selection_model::MapSelectionEvent;
use strum_macros::{Display, EnumDiscriminants, EnumIter, EnumString};

/// Type encompassing all state event types.
//
// === Non-Rustdoc === //
//
// Development note: Remember to update the `FromAppEvent` trait implementations when adding
// variants here.
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
    /// `game_mode_selection` events.
    GameModeSelection(GameModeSelectionEvent),
    /// `game_play` events.
    GamePlay(GamePlayEvent),
    /// `map_selection` events.
    MapSelection(MapSelectionEvent),
    /// Events sent by the winit window.
    // TODO: Pending <https://github.com/amethyst/amethyst/pull/1131>
    Window(#[derivative(PartialEq = "ignore")] Event),
}
