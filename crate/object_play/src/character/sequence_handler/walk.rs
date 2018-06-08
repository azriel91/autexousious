use amethyst::{
    animation::{get_animation_set, AnimationCommand, AnimationControlSet, EndControl},
    ecs::prelude::*, renderer::Material,
};
use object_model::{
    config::object::character::SequenceId, entity::CharacterInput, loaded::Character,
};

use super::SequenceHandler;

#[derive(Debug)]
pub(crate) struct Walk;

impl SequenceHandler for Walk {
    fn begin<'s>(
        entity: &Entity,
        character: &Character,
        animation_control_set_storage: &mut WriteStorage<'s, AnimationControlSet<u32, Material>>,
    ) {
        let animation_handle = &character.object.animations[1].clone();

        // Start the animation
        let animation_set =
            get_animation_set::<u32, Material>(animation_control_set_storage, *entity);
        animation_set.abort(0);
        let animation_id = 1;
        animation_set.add_animation(
            animation_id,
            &animation_handle,
            EndControl::Loop(None),
            30., // Rate at which the animation plays
            AnimationCommand::Start,
        );
    }

    fn update(input: &CharacterInput) -> Option<SequenceId> {
        if input.x_axis_value == 0. && input.z_axis_value == 0. {
            Some(SequenceId::Stand)
        } else {
            None
        }
    }
}
