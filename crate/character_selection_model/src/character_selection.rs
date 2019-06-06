use std::fmt;

use amethyst::assets::Prefab;
use asset_model::loaded::SlugAndHandle;
use character_prefab::{CharacterPrefab, CharacterPrefabHandle};

/// Selected character ID or random for a particular controller.
#[derive(Clone, Debug, PartialEq)]
pub enum CharacterSelection {
    /// User has selected *Random*.
    Random(SlugAndHandle<Prefab<CharacterPrefab>>),
    /// User has selected a character.
    Id(SlugAndHandle<Prefab<CharacterPrefab>>),
}

impl CharacterSelection {
    /// Returns the character prefab handle of this `CharacterSelection`.
    pub fn handle(&self) -> &CharacterPrefabHandle {
        match self {
            CharacterSelection::Random(SlugAndHandle { ref handle, .. })
            | CharacterSelection::Id(SlugAndHandle { ref handle, .. }) => handle,
        }
    }
}

impl fmt::Display for CharacterSelection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CharacterSelection::Random(ref _slug_and_handle) => write!(f, "Random"), // TODO: i18n
            CharacterSelection::Id(SlugAndHandle { ref slug, .. }) => write!(f, "{}", slug),
        }
    }
}
