use character_selection_model::CharacterSelectionEvent;
use map_selection_model::MapSelectionEvent;

/// Type encompassing all state event types.
//
// === Non-Rustdoc === //
//
// Development note: Remember to update the `FromAppEvent` trait implementations when adding
// variants here.
#[derive(Clone, Debug, PartialEq)]
pub enum AppEvent {
    /// `character_selection` events.
    CharacterSelection(CharacterSelectionEvent),
    /// `map_selection` events.
    MapSelection(MapSelectionEvent),
}
