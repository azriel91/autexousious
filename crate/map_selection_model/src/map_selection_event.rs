use map_selection::MapSelection;

/// Event indicating a map selection.
#[derive(Clone, Debug, PartialEq)]
pub enum MapSelectionEvent {
    /// Map has been selected.
    Select {
        /// ID of the selected map.
        map_selection: MapSelection,
    },
}
