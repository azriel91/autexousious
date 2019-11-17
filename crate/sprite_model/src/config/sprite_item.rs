use std::convert::AsRef;

use derive_new::new;
use kinematic_model::config::PositionInit;
use sequence_model::config::SequenceName;
use serde::{Deserialize, Serialize};

use crate::config::{SpriteSequence, SpriteSequenceName};

/// Sequence of sprites in a static position.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
pub struct SpriteItem<SeqName = SpriteSequenceName>
where
    SeqName: SequenceName,
{
    /// Position of the sprite.
    #[serde(default)]
    pub position: PositionInit,
    /// Frames in the sequence.
    #[serde(flatten)]
    pub sequence: SpriteSequence<SeqName>,
}

impl<SeqName> AsRef<SpriteSequence<SeqName>> for SpriteItem<SeqName>
where
    SeqName: SequenceName,
{
    fn as_ref(&self) -> &SpriteSequence<SeqName> {
        &self.sequence
    }
}

impl<SeqName> AsRef<PositionInit> for SpriteItem<SeqName>
where
    SeqName: SequenceName,
{
    fn as_ref(&self) -> &PositionInit {
        &self.position
    }
}
