use character_selection::CharacterSelectionEvent;
use map_selection::MapSelectionEvent;

/// Type encompassing all state event types.
//
// === Non-Rustdoc === //
//
// Development note: Remember to update the `FromAppEvent` trait implementations when adding
// variants here.
#[derive(Clone, Debug, PartialEq)]
pub enum AppEvent {
    /// `character_selection` crate events.
    CharacterSelection(CharacterSelectionEvent),
    /// `map_selection` crate events.
    MapSelection(MapSelectionEvent),
}
