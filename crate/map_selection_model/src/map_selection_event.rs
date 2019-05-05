use crate::map_selection::MapSelection;

/// Event indicating a map selection.
#[derive(Clone, Debug, PartialEq)]
pub enum MapSelectionEvent {
    /// Signal to return from `MapSelectionState`.
    Return,
    /// Map has been selected.
    Select {
        /// ID of the selected map.
        map_selection: MapSelection,
    },
    /// Map has been deselected.
    Deselect,
    /// Map selection has been confirmed.
    Confirm,
}
