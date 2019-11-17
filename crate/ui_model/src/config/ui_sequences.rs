use sequence_model::config::Sequences;
use sprite_model::config::{SpriteFrame, SpriteSequenceName};

use crate::config::UiSequence;

/// Sequences of `SpriteFrame`s.
pub type UiSequences = Sequences<UiSequence, SpriteSequenceName, SpriteFrame>;
