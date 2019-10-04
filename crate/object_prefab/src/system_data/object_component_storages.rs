use amethyst::{
    core::Transform,
    ecs::{World, WriteStorage},
    renderer::transparent::Transparent,
    shred::{ResourceId, SystemData},
};
use asset_model::loaded::AssetId;
use derivative::Derivative;
use kinematic_model::config::{Position, Velocity};
use object_model::play::{Grounding, Mirrored};
use sequence_model::{
    loaded::{SequenceEndTransitions, SequenceId},
    play::{FrameIndexClock, FrameWaitClock, SequenceStatus},
};

/// Common game object `Component` storages.
///
/// These are the storages for the components common to all game objects.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ObjectComponentStorages<'s> {
    /// `AssetId` components.
    #[derivative(Debug = "ignore")]
    pub asset_ids: WriteStorage<'s, AssetId>,
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
    /// `Grounding` component storage.
    #[derivative(Debug = "ignore")]
    pub groundings: WriteStorage<'s, Grounding>,
    /// `SequenceEndTransitions` components.
    #[derivative(Debug = "ignore")]
    pub sequence_end_transitionses: WriteStorage<'s, SequenceEndTransitions>,
    /// `SequenceId` components.
    #[derivative(Debug = "ignore")]
    pub sequence_ids: WriteStorage<'s, SequenceId>,
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
