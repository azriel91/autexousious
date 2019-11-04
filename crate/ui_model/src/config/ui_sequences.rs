use sequence_model::config::Sequences;
use sprite_model::config::SpriteFrame;
use ui_model_spi::config::UiSequenceName;

use crate::config::UiSequence;

/// Sequences of `SpriteFrame`s.
pub type UiSequences = Sequences<UiSequence, UiSequenceName, SpriteFrame>;
