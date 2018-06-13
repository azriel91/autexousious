use object_model::{config::object::character::SequenceId, entity::CharacterInput};

use character::sequence_handler::SequenceHandler;

#[derive(Debug)]
pub(crate) struct Stand;

impl SequenceHandler for Stand {
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
