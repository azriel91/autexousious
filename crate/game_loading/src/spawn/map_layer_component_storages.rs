use amethyst::{
    core::Transform,
    ecs::WriteStorage,
    renderer::{SpriteRender, Transparent},
};
use derivative::Derivative;
use logic_clock::LogicClock;
use sequence_model::{
    config::{Repeat, Wait},
    entity::{FrameIndexClock, SequenceStatus},
    loaded::ComponentSequencesHandle,
};
use shred_derive::SystemData;

/// Map layer `Component` storages.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct MapLayerComponentStorages<'s> {
    /// `Transparent` components.
    #[derivative(Debug = "ignore")]
    pub transparents: WriteStorage<'s, Transparent>,
    /// `Transform` components.
    #[derivative(Debug = "ignore")]
    pub transforms: WriteStorage<'s, Transform>,
    /// `Wait` components.
    #[derivative(Debug = "ignore")]
    pub waits: WriteStorage<'s, Wait>,
    /// `Repeat` components.
    #[derivative(Debug = "ignore")]
    pub repeats: WriteStorage<'s, Repeat>,
    /// `SequenceStatus` components.
    #[derivative(Debug = "ignore")]
    pub sequence_statuses: WriteStorage<'s, SequenceStatus>,
    /// `FrameIndexClock` components.
    #[derivative(Debug = "ignore")]
    pub frame_index_clocks: WriteStorage<'s, FrameIndexClock>,
    /// `LogicClock` components.
    #[derivative(Debug = "ignore")]
    pub logic_clocks: WriteStorage<'s, LogicClock>,
    /// `SpriteRender` components.
    #[derivative(Debug = "ignore")]
    pub sprite_renders: WriteStorage<'s, SpriteRender>,
    /// `ComponentSequencesHandle` components.
    #[derivative(Debug = "ignore")]
    pub component_sequences_handles: WriteStorage<'s, ComponentSequencesHandle>,
}
