use amethyst::ecs::prelude::{Component, DenseVecStorage};
use character_selection_model::CharacterSelection;
use derive_new::new;

use crate::WidgetState;

/// Component to tag entities that are character selection widgets.
#[derive(Clone, Debug, PartialEq, new)]
pub struct CharacterSelectionWidget {
    /// Character selection state.
    pub state: WidgetState,
    /// Selected character ID or random.
    pub selection: CharacterSelection,
}

impl Component for CharacterSelectionWidget {
    type Storage = DenseVecStorage<Self>;
}
