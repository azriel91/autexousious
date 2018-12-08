use derive_new::new;

use crate::config::MapBounds;

/// Base information of the map.
#[derive(Clone, Debug, Deserialize, PartialEq, new)]
pub struct MapHeader {
    /// Name of the map, shown to players.
    pub name: String,
    /// Boundary of the playable area of the map.
    pub bounds: MapBounds,
}
