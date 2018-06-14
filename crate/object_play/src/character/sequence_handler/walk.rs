use object_model::{config::object::character::SequenceId, entity::CharacterInput};

use character::sequence_handler::SequenceHandler;

#[derive(Debug)]
pub(crate) struct Walk;

impl SequenceHandler for Walk {
    fn update(input: &CharacterInput) -> Option<SequenceId> {
        if input.x_axis_value == 0. && input.z_axis_value == 0. {
            Some(SequenceId::Stand)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use object_model::{config::object::character::SequenceId, entity::CharacterInput};

    use super::Walk;
    use character::sequence_handler::SequenceHandler;

    #[test]
    fn update_returns_none_when_x_and_z_axes_are_non_zero() {
        let input = CharacterInput::new(1., 1., false, false, false, false);

        assert_eq!(None, Walk::update(&input));
    }

    #[test]
    fn update_returns_none_when_x_axis_is_non_zero() {
        let input = CharacterInput::new(1., 0., false, false, false, false);

        assert_eq!(None, Walk::update(&input));
    }

    #[test]
    fn update_returns_none_when_z_axis_is_non_zero() {
        let input = CharacterInput::new(0., 1., false, false, false, false);

        assert_eq!(None, Walk::update(&input));
    }

    #[test]
    fn update_returns_stand_when_x_and_z_axes_are_non_zero() {
        let input = CharacterInput::new(0., 0., false, false, false, false);

        assert_eq!(Some(SequenceId::Stand), Walk::update(&input));
    }
}
