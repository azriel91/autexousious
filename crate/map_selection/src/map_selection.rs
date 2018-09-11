use map_model::loaded::MapHandle;

/// Stores the selected map for the current game.
#[derive(Debug, Default, new)]
pub struct MapSelection {
    /// Handle to the selected map.
    pub map_handle: Option<MapHandle>,
}
