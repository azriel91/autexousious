use amethyst::ecs::prelude::{Component, HashMapStorage};
use map_selection::MapSelection;

use WidgetState;

/// Component to tag entities that are map selection widgets.
#[derive(Clone, Debug, PartialEq, new)]
pub(crate) struct MapSelectionWidget {
    /// Map selection state.
    pub state: WidgetState,
    /// Selected map ID or random.
    pub selection: MapSelection,
}

impl Component for MapSelectionWidget {
    type Storage = HashMapStorage<Self>;
}
