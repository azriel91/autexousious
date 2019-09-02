use crate::play::MapBoundaryEventData;

/// Indicates an entity has entered or exited the playable area.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MapBoundaryEvent {
    /// Entity entered the playable area.
    Enter(MapBoundaryEventData),
    /// Entity exited the playable area.
    Exit(MapBoundaryEventData),
}
