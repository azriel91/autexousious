use std::fmt;

use asset_model::loaded::SlugAndHandle;
use character_model::loaded::{Character, CharacterHandle};

/// Selected character ID or random for a particular controller.
#[derive(Clone, Debug, PartialEq)]
pub enum CharacterSelection {
    /// User has selected *Random*.
    Random(SlugAndHandle<Character>),
    /// User has selected a character.
    Id(SlugAndHandle<Character>),
}

impl CharacterSelection {
    /// Returns the character handle of this `CharacterSelection`.
    pub fn handle(&self) -> &CharacterHandle {
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
