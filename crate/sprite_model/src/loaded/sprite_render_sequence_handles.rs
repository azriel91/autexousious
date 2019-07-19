use sequence_model::config::SequenceId;
use sequence_model_derive::sequence_component_data;

use crate::loaded::SpriteRenderSequenceHandle;

/// Map of `SpriteRenderSequenceHandle`s, keyed by Sequence ID.
#[sequence_component_data(SeqId, SpriteRenderSequenceHandle)]
pub struct SpriteRenderSequenceHandles<SeqId>
where
    SeqId: SequenceId;
