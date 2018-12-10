use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::MapBounds;

/// Base information of the map.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
pub struct MapHeader {
    /// Name of the map, shown to players.
    pub name: String,
    /// Boundary of the playable area of the map.
    pub bounds: MapBounds,
}
