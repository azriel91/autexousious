use amethyst::{animation::AnimationControlSet, ecs::prelude::*, renderer::Material};
use object_model::{
    config::object::character::SequenceId, entity::CharacterInput, loaded::Character,
};

pub(super) use self::stand::Stand;
pub(super) use self::walk::Walk;

mod stand;
mod walk;

/// Traits that every sequence should define for its transition behaviour.
pub(super) trait SequenceHandler {
    /// Executes behaviour when this sequence is first transitioned into.
    ///
    /// This includes starting animations, sounds, etcetera.
    fn begin<'s>(
        entity: &Entity,
        character: &Character,
        animation_control_set_storage: &mut WriteStorage<'s, AnimationControlSet<u32, Material>>,
    );

    /// Updates behaviour in response to input.
    fn update(input: &CharacterInput) -> Option<SequenceId>;
}
