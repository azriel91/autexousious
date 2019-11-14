use std::convert::AsRef;

use derive_new::new;
use kinematic_model::config::PositionInit;
use sequence_model::config::SequenceNameString;
use serde::{Deserialize, Serialize};
use sprite_model::config::SpriteSequenceName;

/// Defines a sprite sequence to display.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct UiSpriteLabel {
    /// Position of the label relative to its parent.
    #[serde(default)]
    pub position: PositionInit,
    /// `SequenceNameString` that the `UiSpriteLabel` should begin with.
    pub sequence: SequenceNameString<SpriteSequenceName>,
}

impl AsRef<UiSpriteLabel> for UiSpriteLabel {
    fn as_ref(&self) -> &UiSpriteLabel {
        self
    }
}
