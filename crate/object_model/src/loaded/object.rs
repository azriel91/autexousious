use collision_model::loaded::{BodySequenceHandles, InteractionsSequenceHandles};
use derivative::Derivative;
use derive_new::new;
use sequence_model::{
    config::SequenceId,
    loaded::{SequenceEndTransitions, WaitSequenceHandles},
};
use spawn_model::loaded::SpawnsSequenceHandles;
use sprite_model::loaded::SpriteRenderSequenceHandles;

/// Represents an in-game object that has been loaded.
#[derive(Clone, Derivative, PartialEq, new)]
#[derivative(Debug)]
pub struct Object<SeqId>
where
    SeqId: SequenceId,
{
    /// Handles to `WaitSequence`s that this object uses, keyed by sequence ID.
    pub wait_sequence_handles: WaitSequenceHandles<SeqId>,
    /// Handles to `SpriteRenderSequence`s that this object uses, keyed by sequence ID.
    pub sprite_render_sequence_handles: SpriteRenderSequenceHandles<SeqId>,
    /// Handles to `BodySequence`s that this object uses, keyed by sequence ID.
    pub body_sequence_handles: BodySequenceHandles<SeqId>,
    /// Handles to `InteractionsSequence`s that this object uses, keyed by sequence ID.
    pub interactions_sequence_handles: InteractionsSequenceHandles<SeqId>,
    /// Handles to `SpawnsSequence`s that this object uses, keyed by sequence ID.
    pub spawns_sequence_handles: SpawnsSequenceHandles<SeqId>,
    /// Sequence transition when a sequence ends, keyed by sequence ID.
    pub sequence_end_transitions: SequenceEndTransitions<SeqId>,
}
