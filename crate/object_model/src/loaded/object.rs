use std::collections::HashMap;

use derivative::Derivative;
use derive_new::new;
use sequence_model::loaded::ComponentSequencesHandle;

use crate::{config::object::SequenceId, loaded::SequenceEndTransitions};

/// Represents an in-game object that has been loaded.
#[derive(Clone, Derivative, PartialEq, new)]
#[derivative(Debug)]
pub struct Object<SeqId>
where
    SeqId: SequenceId,
{
    /// Handle to sequences of components that this object uses, keyed by sequence ID.
    pub component_sequences_handles: HashMap<SeqId, ComponentSequencesHandle>,
    /// Component sequence transitions when a sequence ends.
    pub sequence_end_transitions: SequenceEndTransitions<SeqId>,
}
