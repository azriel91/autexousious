use audio_model::loaded::SourceSequenceHandles;
use collision_model::loaded::{BodySequenceHandles, InteractionsSequenceHandles};
use derivative::Derivative;
use derive_new::new;
use kinematic_model::loaded::ObjectAccelerationSequenceHandles;
use sequence_model::loaded::{SequenceEndTransitions, WaitSequenceHandles};
use spawn_model::loaded::SpawnsSequenceHandles;
use sprite_model::loaded::SpriteRenderSequenceHandles;

/// Represents an in-game object that has been loaded.
#[allow(clippy::too_many_arguments)]
#[derive(Clone, Derivative, PartialEq, new)]
#[derivative(Debug)]
pub struct Object {
    /// Handles to `WaitSequence`s that this object uses.
    pub wait_sequence_handles: WaitSequenceHandles,
    /// Handles to `SourceSequence`s that this object uses.
    pub source_sequence_handles: SourceSequenceHandles,
    /// Handles to `ObjectAccelerationSequence`s that this object uses.
    pub object_acceleration_sequence_handles: ObjectAccelerationSequenceHandles,
    /// Handles to `SpriteRenderSequence`s that this object uses.
    pub sprite_render_sequence_handles: SpriteRenderSequenceHandles,
    /// Handles to `BodySequence`s that this object uses.
    pub body_sequence_handles: BodySequenceHandles,
    /// Handles to `InteractionsSequence`s that this object uses.
    pub interactions_sequence_handles: InteractionsSequenceHandles,
    /// Handles to `SpawnsSequence`s that this object uses.
    pub spawns_sequence_handles: SpawnsSequenceHandles,
    /// Sequence transition when a sequence ends.
    pub sequence_end_transitions: SequenceEndTransitions,
}
