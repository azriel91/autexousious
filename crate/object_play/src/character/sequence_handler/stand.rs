use object_model::{
    config::object::CharacterSequenceId,
    entity::{CharacterInput, CharacterStatus, ObjectStatusUpdate},
};

use character::sequence_handler::SequenceHandler;

#[derive(Debug)]
pub(crate) struct Stand;

impl SequenceHandler for Stand {
    fn update(
        input: &CharacterInput,
        _character_status: &mut CharacterStatus,
    ) -> ObjectStatusUpdate<CharacterSequenceId> {
        let sequence_id = if input.x_axis_value != 0. || input.z_axis_value != 0. {
            Some(CharacterSequenceId::Walk)
        } else {
            None
        };

        let mirrored = if input.x_axis_value < 0. {
            Some(true)
        } else if input.x_axis_value > 0. {
            Some(false)
        } else {
            None
        };

        ObjectStatusUpdate::new(sequence_id, mirrored)
    }
}

#[cfg(test)]
mod test {
    use object_model::{
        config::object::CharacterSequenceId,
        entity::{CharacterInput, CharacterStatus},
    };

    use super::Stand;
    use character::sequence_handler::SequenceHandler;

    #[test]
    fn update_sequence_is_none_when_x_and_z_axes_are_zero() {
        let input = CharacterInput::new(0., 0., false, false, false, false);

        assert_eq!(
            None,
            Stand::update(&input, &mut CharacterStatus::default()).sequence_id
        );
    }

    #[test]
    fn update_sequence_is_walk_when_x_axis_is_non_zero() {
        let input = CharacterInput::new(1., 0., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceId::Walk),
            Stand::update(&input, &mut CharacterStatus::default()).sequence_id
        );
    }

    #[test]
    fn update_sequence_is_walk_when_z_axis_is_non_zero() {
        let input = CharacterInput::new(0., 1., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceId::Walk),
            Stand::update(&input, &mut CharacterStatus::default()).sequence_id
        );
    }

    #[test]
    fn update_sequence_is_walk_when_x_and_z_axes_are_non_zero() {
        let input = CharacterInput::new(1., 1., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceId::Walk),
            Stand::update(&input, &mut CharacterStatus::default()).sequence_id
        );
    }

    #[test]
    fn update_mirrored_is_none_when_x_axis_is_zero() {
        let input = CharacterInput::new(0., 0., false, false, false, false);

        assert_eq!(
            None,
            Stand::update(&input, &mut CharacterStatus::default()).mirrored
        );
    }

    #[test]
    fn update_mirrored_is_false_when_x_axis_is_above_zero() {
        let input = CharacterInput::new(1., 0., false, false, false, false);

        assert_eq!(
            Some(false),
            Stand::update(&input, &mut CharacterStatus::default()).mirrored
        );
    }

    #[test]
    fn update_mirrored_is_true_when_z_axis_is_below_zero() {
        let input = CharacterInput::new(-1., 0., false, false, false, false);

        assert_eq!(
            Some(true),
            Stand::update(&input, &mut CharacterStatus::default()).mirrored
        );
    }
}
