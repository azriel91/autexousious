use std::fmt;

use game_model::loaded::SlugAndHandle;
use map_model::loaded::{Map, MapHandle};

/// Selected map ID or random for a particular controller.
#[derive(Clone, Debug, PartialEq)]
pub enum MapSelection {
    /// User has selected *Random*.
    Random(SlugAndHandle<Map>),
    /// User has selected a map.
    Id(SlugAndHandle<Map>),
}

impl MapSelection {
    /// Returns the map handle of this `MapSelection`.
    pub fn handle(&self) -> &MapHandle {
        match self {
            MapSelection::Random(SlugAndHandle { ref handle, .. })
            | MapSelection::Id(SlugAndHandle { ref handle, .. }) => handle,
        }
    }
}

impl fmt::Display for MapSelection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MapSelection::Random(ref _slug_and_handle) => write!(f, "Random"), // TODO: i18n
            MapSelection::Id(SlugAndHandle { ref slug, .. }) => write!(f, "{}", slug),
        }
    }
}
