use amethyst::{
    core::Transform,
    ecs::{World, WriteStorage},
    renderer::transparent::Transparent,
    shred::{ResourceId, SystemData},
};
use derivative::Derivative;
use kinematic_model::config::{Position, Velocity};
use object_model::play::Mirrored;
use sequence_model::{
    config::SequenceId,
    loaded::SequenceEndTransitions,
    play::{FrameIndexClock, FrameWaitClock, SequenceStatus},
};

/// Common game object `Component` storages.
///
/// These are the storages for the components common to all game objects.
///
/// # Type Parameters:
///
/// * `SeqId`: Sequence ID of the object, such as `CharacterSequenceId`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ObjectComponentStorages<'s, SeqId>
where
    SeqId: SequenceId + 'static,
{
    /// `Transparent` components.
    #[derivative(Debug = "ignore")]
    pub transparents: WriteStorage<'s, Transparent>,
    /// `Position` components.
    #[derivative(Debug = "ignore")]
    pub positions: WriteStorage<'s, Position<f32>>,
    /// `Velocity` components.
    #[derivative(Debug = "ignore")]
    pub velocities: WriteStorage<'s, Velocity<f32>>,
    /// `Transform` components.
    #[derivative(Debug = "ignore")]
    pub transforms: WriteStorage<'s, Transform>,
    /// `Mirrored` components.
    #[derivative(Debug = "ignore")]
    pub mirroreds: WriteStorage<'s, Mirrored>,
    /// `SequenceEndTransitions` components.
    #[derivative(Debug = "ignore")]
    pub sequence_end_transitionses: WriteStorage<'s, SequenceEndTransitions<SeqId>>,
    /// `SeqId` components.
    #[derivative(Debug = "ignore")]
    pub sequence_ids: WriteStorage<'s, SeqId>,
    /// `SequenceStatus` components.
    #[derivative(Debug = "ignore")]
    pub sequence_statuses: WriteStorage<'s, SequenceStatus>,
    /// `FrameIndexClock` components.
    #[derivative(Debug = "ignore")]
    pub frame_index_clocks: WriteStorage<'s, FrameIndexClock>,
    /// `FrameWaitClock` components.
    #[derivative(Debug = "ignore")]
    pub frame_wait_clocks: WriteStorage<'s, FrameWaitClock>,
}
