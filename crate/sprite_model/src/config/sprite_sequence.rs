use std::convert::AsRef;

use derive_new::new;
use sequence_model::config::{Sequence, SequenceName};
use serde::{Deserialize, Serialize};

use crate::config::{SpriteFrame, SpritePosition, SpriteSequenceName};

/// Sequence of sprites in a static position.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
pub struct SpriteSequence<SeqName = SpriteSequenceName>
where
    SeqName: SequenceName,
{
    /// Position of the sprite.
    #[serde(default)]
    pub position: SpritePosition,
    /// Frames in the sequence.
    #[serde(flatten)]
    pub sequence: Sequence<SeqName, SpriteFrame>,
}

impl<SeqName> AsRef<Sequence<SeqName, SpriteFrame>> for SpriteSequence<SeqName>
where
    SeqName: SequenceName,
{
    fn as_ref(&self) -> &Sequence<SeqName, SpriteFrame> {
        &self.sequence
    }
}

impl<SeqName> AsRef<SpritePosition> for SpriteSequence<SeqName>
where
    SeqName: SequenceName,
{
    fn as_ref(&self) -> &SpritePosition {
        &self.position
    }
}
