use std::fmt;

use game_model::config::AssetSlug;

/// Selected character ID or random for a particular controller.
#[derive(Clone, Debug, Derivative, PartialEq, Eq)]
#[derivative(Default)]
pub enum CharacterSelection {
    /// User has selected *Random*.
    #[derivative(Default)]
    Random,
    /// User has selected a character.
    Id(AssetSlug),
}

impl fmt::Display for CharacterSelection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CharacterSelection::Random => write!(f, "Random"), // TODO: i18n
            CharacterSelection::Id(ref slug) => write!(f, "{}", slug),
        }
    }
}
