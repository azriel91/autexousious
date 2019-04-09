use derive_deref::{Deref, DerefMut};
use derive_more::From;
use derive_new::new;

use crate::{component_sequence, config::SequenceId, loaded::ControlTransitions};

/// Sequence of sequence transitions upon control input.
#[component_sequence(ControlTransitions<SeqId>)]
pub struct ControlTransitionsSequence<SeqId>
where
    SeqId: SequenceId;
