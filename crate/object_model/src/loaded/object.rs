use std::collections::HashMap;

use derivative::Derivative;
use derive_new::new;

use crate::{
    config::object::SequenceId,
    loaded::{
        AnimatedComponentAnimation, AnimatedComponentDefault, ComponentSequences,
        SequenceEndTransitions,
    },
};

/// Represents an in-game object that has been loaded.
#[derive(Clone, Derivative, PartialEq, new)]
#[derivative(Debug)]
pub struct Object<SeqId>
where
    SeqId: SequenceId,
{
    /// Handle to the default sprite sheet to use for the object.
    pub animation_defaults: Vec<AnimatedComponentDefault>,
    /// Handles to the animations that this object uses, keyed by sequence ID.
    pub animations: HashMap<SeqId, Vec<AnimatedComponentAnimation>>,
    /// Sequences of components that this object uses, keyed by sequence ID.
    pub component_sequences: HashMap<SeqId, ComponentSequences>,
    /// Component sequence transitions when a sequence ends.
    pub sequence_end_transitions: SequenceEndTransitions<SeqId>,
}
