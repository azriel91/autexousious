use std::fmt;

use asset_model::config::AssetSlug;

/// Selected character ID or random for a particular controller.
#[derive(Clone, Debug, PartialEq)]
pub enum CharacterSelection {
    /// User has selected *Random*.
    Random,
    /// User has selected a character.
    Id(AssetSlug),
}

impl fmt::Display for CharacterSelection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CharacterSelection::Random => write!(f, "Random"), // TODO: i18n
            CharacterSelection::Id(ref asset_slug) => write!(f, "{}", asset_slug),
        }
    }
}
