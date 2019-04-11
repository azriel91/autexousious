use derive_deref::{Deref, DerefMut};
use derive_more::From;
use derive_new::new;

use crate::{
    component_sequence,
    config::SequenceId,
    loaded::{ControlTransitionLike, ControlTransitions},
};

/// Sequence of sequence transitions upon control input.
#[component_sequence(ControlTransitions<SeqId, C>)]
pub struct ControlTransitionsSequence<SeqId, C>
where
    C: ControlTransitionLike<SeqId> + Send + Sync + 'static,
    SeqId: SequenceId;
