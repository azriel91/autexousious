use sequence_model_spi::loaded::ComponentFrames;

use crate::loaded::ControlTransitions;

/// Sequence of sequence transitions upon control input.
pub type ControlTransitionsSequence<SeqId> = ComponentFrames<ControlTransitions<SeqId>>;
