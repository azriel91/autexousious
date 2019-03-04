use amethyst::{
    core::Transform,
    ecs::WriteStorage,
    renderer::{SpriteRender, Transparent},
};
use derivative::Derivative;
use sequence_model::loaded::ComponentSequencesHandle;
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
    /// `SpriteRender` components.
    #[derivative(Debug = "ignore")]
    pub sprite_renders: WriteStorage<'s, SpriteRender>,
    /// `ComponentSequencesHandle` components.
    #[derivative(Debug = "ignore")]
    pub component_sequences_handles: WriteStorage<'s, ComponentSequencesHandle>,
}
