use map_model::loaded::MapHandle;

use SelectionStatus;

/// Stores the selected map for the current game.
#[derive(Debug, Default)]
pub struct MapSelection {
    /// Status of the user selecting a map.
    pub(crate) selection_status: SelectionStatus,
    /// Handle to the selected map.
    pub map_handle: Option<MapHandle>,
}
