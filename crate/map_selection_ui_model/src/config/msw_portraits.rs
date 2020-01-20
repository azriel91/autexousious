use sequence_model::config::SequenceNameString;
use serde::{Deserialize, Serialize};
use sprite_model::config::SpriteSequenceName;

/// Portraits to use while map selection is not present.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct MswPortraits {
    /// Used when map selection is "Random".
    pub random: SequenceNameString<SpriteSequenceName>,
    /// Used when map selection is a map.
    pub select: SequenceNameString<SpriteSequenceName>,
}
