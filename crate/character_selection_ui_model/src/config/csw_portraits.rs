use sequence_model::config::SequenceNameString;
use serde::{Deserialize, Serialize};
use sprite_model::config::SpriteSequenceName;

/// Portraits to use while character selection is not present.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct CswPortraits {
    /// Used when the widget is inactive.
    pub join: SequenceNameString<SpriteSequenceName>,
    /// Used when character selection is "Random".
    pub random: SequenceNameString<SpriteSequenceName>,
    /// Used when character selection is a character.
    pub select: SequenceNameString<SpriteSequenceName>,
}
