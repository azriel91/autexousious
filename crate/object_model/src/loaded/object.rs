use std::collections::HashMap;

use collision_model::loaded::{BodySequenceHandle, InteractionsSequenceHandle};
use derivative::Derivative;
use derive_new::new;
use sequence_model::{
    config::SequenceId,
    loaded::{ComponentSequencesHandle, SequenceEndTransitions, WaitSequenceHandle},
};
use sprite_model::loaded::SpriteRenderSequenceHandle;

/// Represents an in-game object that has been loaded.
#[derive(Clone, Derivative, PartialEq, new)]
#[derivative(Debug)]
pub struct Object<SeqId>
where
    SeqId: SequenceId,
{
    /// Handle to sequences of components that this object uses, keyed by sequence ID.
    pub component_sequences_handles: HashMap<SeqId, ComponentSequencesHandle>,
    /// Handle to `WaitSequence`s that this object uses, keyed by sequence ID.
    pub wait_sequence_handles: HashMap<SeqId, WaitSequenceHandle>,
    /// Handle to `SpriteRenderSequence`s that this object uses, keyed by sequence ID.
    pub sprite_render_sequence_handles: HashMap<SeqId, SpriteRenderSequenceHandle>,
    /// Handle to `BodySequence`s that this object uses, keyed by sequence ID.
    pub body_sequence_handles: HashMap<SeqId, BodySequenceHandle>,
    /// Handle to `InteractionsSequence`s that this object uses, keyed by sequence ID.
    pub interactions_sequence_handles: HashMap<SeqId, InteractionsSequenceHandle>,
    /// Component sequence transitions when a sequence ends.
    pub sequence_end_transitions: SequenceEndTransitions<SeqId>,
}
