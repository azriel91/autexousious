use amethyst::{
    ecs::{World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use collision_model::loaded::{BodySequenceHandle, InteractionsSequenceHandle};
use derivative::Derivative;
use sequence_model::loaded::WaitSequenceHandle;
use spawn_model::loaded::SpawnsSequenceHandle;
use sprite_model::loaded::SpriteRenderSequenceHandle;

/// Frame component data handle storages.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct FrameComponentDataHandleStorages<'s> {
    /// `WaitSequenceHandle` components.
    #[derivative(Debug = "ignore")]
    pub wait_sequence_handles: WriteStorage<'s, WaitSequenceHandle>,
    /// `SpriteRenderSequenceHandle` components.
    #[derivative(Debug = "ignore")]
    pub sprite_render_sequence_handles: WriteStorage<'s, SpriteRenderSequenceHandle>,
    /// `BodySequenceHandle` components.
    #[derivative(Debug = "ignore")]
    pub body_sequence_handles: WriteStorage<'s, BodySequenceHandle>,
    /// `InteractionsSequenceHandle` components.
    #[derivative(Debug = "ignore")]
    pub interactions_sequence_handles: WriteStorage<'s, InteractionsSequenceHandle>,
    /// `SpawnsSequenceHandle` components.
    #[derivative(Debug = "ignore")]
    pub spawns_sequence_handles: WriteStorage<'s, SpawnsSequenceHandle>,
}
