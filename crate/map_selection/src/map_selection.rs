use map_model::loaded::MapHandle;

use MapSelectionStatus;

/// Stores the selected map for the current game.
#[derive(Debug, Default, new)]
pub struct MapSelection {
    /// Status of the user selecting a map.
    ///
    /// This is not intended to be mutated outside the `MapSelectionState`. However, it may be used
    /// by test logic to set up a precondition state.
    pub status: MapSelectionStatus,
    /// Handle to the selected map.
    pub map_handle: Option<MapHandle>,
}
