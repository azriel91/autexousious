use object_model::{config::object::CharacterSequenceId, entity::SequenceStatus};

use character::sequence_handler::CharacterSequenceHandler;
use CharacterSequenceUpdateComponents;

#[derive(Debug)]
pub(crate) struct JumpOff;

impl CharacterSequenceHandler for JumpOff {
    fn update<'c>(
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> Option<CharacterSequenceId> {
        // Switch to jump_descend when Y axis velocity is no longer upwards.
        if components.velocity[1] <= 0. {
            Some(CharacterSequenceId::JumpDescend)
        } else if *components.sequence_status == SequenceStatus::End {
            Some(CharacterSequenceId::JumpAscend)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use game_input::ControllerInput;
    use object_model::{
        config::object::CharacterSequenceId,
        entity::{
            CharacterStatus, Grounding, Mirrored, Position, RunCounter, SequenceStatus, Velocity,
        },
    };

    use super::JumpOff;
    use character::sequence_handler::CharacterSequenceHandler;
    use CharacterSequenceUpdateComponents;

    #[test]
    fn no_update_when_sequence_not_ended() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut velocity = Velocity::default();
        velocity[1] = 1.;

        assert_eq!(
            None,
            JumpOff::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &CharacterSequenceId::JumpOff,
                &SequenceStatus::default(),
                &Position::default(),
                &velocity,
                &Mirrored::default(),
                &Grounding::default(),
                &RunCounter::default()
            ))
        );
    }

    #[test]
    fn switches_to_jump_ascend_when_sequence_ends() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut velocity = Velocity::default();
        velocity[1] = 1.;

        assert_eq!(
            Some(CharacterSequenceId::JumpAscend),
            JumpOff::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &CharacterSequenceId::JumpOff,
                &SequenceStatus::End,
                &Position::default(),
                &velocity,
                &Mirrored::default(),
                &Grounding::default(),
                &RunCounter::default()
            ))
        );
    }

    #[test]
    fn switches_to_jump_descend_when_y_velocity_is_zero_or_downwards() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut downwards_velocity = Velocity::default();
        downwards_velocity[1] = -1.;

        vec![Velocity::default(), downwards_velocity]
            .into_iter()
            .for_each(|velocity| {
                assert_eq!(
                    Some(CharacterSequenceId::JumpDescend),
                    JumpOff::update(CharacterSequenceUpdateComponents::new(
                        &input,
                        &CharacterStatus::default(),
                        &CharacterSequenceId::JumpOff,
                        &SequenceStatus::Ongoing,
                        &Position::default(),
                        &velocity,
                        &Mirrored::default(),
                        &Grounding::default(),
                        &RunCounter::default()
                    ))
                );
            });
    }
}
