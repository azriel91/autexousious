use amethyst::{
    animation::{get_animation_set, AnimationCommand, AnimationControlSet, EndControl},
    ecs::prelude::*,
    renderer::Material,
};
use object_model::{
    config::object::character::SequenceId, entity::CharacterInput, loaded::Character,
};

use character::sequence_handler::SequenceHandler;

#[derive(Debug)]
pub(crate) struct Stand;

impl SequenceHandler for Stand {
    fn begin<'s>(
        entity: &Entity,
        character: &Character,
        animation_control_set_storage: &mut WriteStorage<'s, AnimationControlSet<u32, Material>>,
    ) {
        let animation_handle = &character.object.animations[0].clone();

        // Start the animation
        let animation_set =
            get_animation_set::<u32, Material>(animation_control_set_storage, *entity);
        animation_set.abort(1);
        let animation_id = 0;
        animation_set.add_animation(
            animation_id,
            &animation_handle,
            EndControl::Loop(None),
            30., // Rate at which the animation plays
            AnimationCommand::Start,
        );
    }

    fn update(input: &CharacterInput) -> Option<SequenceId> {
        if input.x_axis_value != 0. || input.z_axis_value != 0. {
            Some(SequenceId::Walk)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use object_model::{config::object::character::SequenceId, entity::CharacterInput};

    use super::Stand;
    use character::sequence_handler::SequenceHandler;

    #[test]
    fn update_returns_none_when_x_and_z_axes_are_zero() {
        let input = CharacterInput::new(0., 0., false, false, false, false);

        assert_eq!(None, Stand::update(&input));
    }

    #[test]
    fn update_returns_walk_when_x_axis_is_non_zero() {
        let input = CharacterInput::new(1., 0., false, false, false, false);

        assert_eq!(Some(SequenceId::Walk), Stand::update(&input));
    }

    #[test]
    fn update_returns_walk_when_z_axis_is_non_zero() {
        let input = CharacterInput::new(0., 1., false, false, false, false);

        assert_eq!(Some(SequenceId::Walk), Stand::update(&input));
    }

    #[test]
    fn update_returns_walk_when_x_and_z_axes_are_non_zero() {
        let input = CharacterInput::new(1., 1., false, false, false, false);

        assert_eq!(Some(SequenceId::Walk), Stand::update(&input));
    }
}
