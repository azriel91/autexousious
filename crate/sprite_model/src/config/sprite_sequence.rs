use sequence_model::config::Sequence;

use crate::config::{SpriteFrame, SpriteSequenceName};

/// Sequence of sprites.
///
/// # Type Parameters
///
/// * `SeqName`: Type of the sequence name. Defaults to `SpriteSequenceName`.
pub type SpriteSequence<SeqName = SpriteSequenceName> = Sequence<SeqName, SpriteFrame>;
