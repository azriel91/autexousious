use amethyst::{animation::AnimationControlSet, ecs::WriteStorage, renderer::SpriteRender};
use collision_model::animation::{BodyFrameActiveHandle, InteractionFrameActiveHandle};
use derivative::Derivative;
use object_model::config::object::SequenceId;
use shred_derive::SystemData;

/// Shorthand for `AnimationControlSet<SeqId, SpriteRender>`.
pub type SpriteRenderAcs<SeqId> = AnimationControlSet<SeqId, SpriteRender>;
/// Shorthand for `AnimationControlSet<SeqId, BodyFrameActiveHandle>`.
pub type BodyAcs<SeqId> = AnimationControlSet<SeqId, BodyFrameActiveHandle>;
/// Shorthand for `AnimationControlSet<SeqId, InteractionFrameActiveHandle>`.
pub type InteractionAcs<SeqId> = AnimationControlSet<SeqId, InteractionFrameActiveHandle>;

/// Common game object `Animation` storages.
///
/// These are the storages for the animation control sets common to all game objects.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ObjectAnimationStorages<'s, SeqId>
where
    SeqId: SequenceId + 'static,
{
    /// `SpriteRender` animation control set storage.
    #[derivative(Debug = "ignore")]
    pub sprite_render_acses: WriteStorage<'s, SpriteRenderAcs<SeqId>>,
    /// `Body` animation control set storage.
    #[derivative(Debug = "ignore")]
    pub body_acses: WriteStorage<'s, BodyAcs<SeqId>>,
    /// `Interaction` animation control set storage.
    #[derivative(Debug = "ignore")]
    pub interaction_acses: WriteStorage<'s, InteractionAcs<SeqId>>,
}
