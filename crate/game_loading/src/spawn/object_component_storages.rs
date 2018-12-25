use amethyst::{
    core::Transform,
    ecs::prelude::*,
    renderer::{Flipped, SpriteRender, Transparent},
};
use collision_model::animation::{BodyFrameActiveHandle, InteractionFrameActiveHandle};
use derivative::Derivative;
use object_model::entity::{Position, Velocity};
use shred_derive::SystemData;

/// Common game object `Component` storages.
///
/// These are the storages for the components common to all game objects.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ObjectComponentStorages<'s> {
    /// `SpriteRender` component storage.
    #[derivative(Debug = "ignore")]
    pub sprite_renders: WriteStorage<'s, SpriteRender>,
    /// `Flipped` component storage.
    #[derivative(Debug = "ignore")]
    pub flippeds: WriteStorage<'s, Flipped>,
    /// `Transparent` component storage.
    #[derivative(Debug = "ignore")]
    pub transparents: WriteStorage<'s, Transparent>,
    /// `Position` component storage.
    #[derivative(Debug = "ignore")]
    pub positions: WriteStorage<'s, Position<f32>>,
    /// `Velocity` component storage.
    #[derivative(Debug = "ignore")]
    pub velocities: WriteStorage<'s, Velocity<f32>>,
    /// `Transform` component storage.
    #[derivative(Debug = "ignore")]
    pub transforms: WriteStorage<'s, Transform>,
    /// `BodyFrameActiveHandle` component storage.
    #[derivative(Debug = "ignore")]
    pub body_frame_active_handles: WriteStorage<'s, BodyFrameActiveHandle>,
    /// `InteractionFrameActiveHandle` component storage.
    #[derivative(Debug = "ignore")]
    pub interaction_frame_active_handles: WriteStorage<'s, InteractionFrameActiveHandle>,
}
