use sequence_model::config::Sequence;
use sprite_model::config::SpriteFrame;
use ui_model_spi::config::UiSequenceName;

/// Plain sequence of `SpriteFrame`s.
pub type UiSequence = Sequence<UiSequenceName, SpriteFrame>;
