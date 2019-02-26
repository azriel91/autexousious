use amethyst::{assets::Handle, ecs::WriteStorage, renderer::SpriteRender};
use collision_model::config::{Body, Interactions};
use derivative::Derivative;
use object_model::config::object::Wait;
use shred_derive::SystemData;

/// Game object `Component` storages for components that change per sequence frame.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ObjectFrameComponentStorages<'s> {
    /// `Wait` component storage.
    #[derivative(Debug = "ignore")]
    pub waits: WriteStorage<'s, Wait>,
    /// `SpriteRender` component storage.
    #[derivative(Debug = "ignore")]
    pub sprite_renders: WriteStorage<'s, SpriteRender>,
    /// `Body` component storage.
    #[derivative(Debug = "ignore")]
    pub bodies: WriteStorage<'s, Handle<Body>>,
    /// `Interactions` component storage.
    #[derivative(Debug = "ignore")]
    pub interactionses: WriteStorage<'s, Handle<Interactions>>,
}
