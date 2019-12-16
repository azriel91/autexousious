use sequence_model::config::Sequences;
use sprite_model::config::SpriteSequenceName;

use crate::config::{UiFrame, UiSequence};

/// Sequences of `UiFrame`s.
pub type UiSequences = Sequences<UiSequence, SpriteSequenceName, UiFrame>;
