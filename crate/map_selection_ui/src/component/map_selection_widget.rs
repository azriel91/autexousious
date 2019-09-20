use amethyst::ecs::prelude::{Component, HashMapStorage};
use derive_new::new;
use map_selection_model::MapSelection;

use crate::WidgetState;

/// Component to tag entities that are map selection widgets.
#[derive(Clone, Copy, Debug, PartialEq, new)]
pub(crate) struct MapSelectionWidget {
    /// Map selection state.
    pub state: WidgetState,
    /// Selected map ID or random.
    pub selection: MapSelection,
}

impl Component for MapSelectionWidget {
    type Storage = HashMapStorage<Self>;
}
