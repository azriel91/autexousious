use map_model::loaded::MapHandle;

/// Event indicating a map selection.
///
/// This should contain all the options on the map selection before it is sent through. Currently
/// there are no options, so only the map handle is present.
#[derive(Debug, new)]
pub struct MapSelectionEvent {
    /// Handle to the selected map.
    pub map_handle: MapHandle,
}
