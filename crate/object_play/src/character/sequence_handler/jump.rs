use object_model::{config::object::CharacterSequenceId, entity::SequenceStatus};

use character::sequence_handler::CharacterSequenceHandler;
use CharacterSequenceUpdateComponents;

#[derive(Debug)]
pub(crate) struct Jump;

impl CharacterSequenceHandler for Jump {
    fn update<'c>(
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> Option<CharacterSequenceId> {
        if components.sequence_status == SequenceStatus::End {
            Some(CharacterSequenceId::JumpOff)
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
        entity::{CharacterStatus, Grounding, Kinematics, Mirrored, RunCounter, SequenceStatus},
    };

    use super::Jump;
    use character::sequence_handler::CharacterSequenceHandler;
    use CharacterSequenceUpdateComponents;

    #[test]
    fn no_update_when_sequence_not_ended() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            None,
            Jump::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                CharacterSequenceId::Jump,
                SequenceStatus::default(),
                &Kinematics::default(),
                Mirrored::default(),
                Grounding::default(),
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn switches_to_jump_off_when_sequence_ends() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut kinematics = Kinematics::default();
        kinematics.velocity[1] = 1.;

        assert_eq!(
            Some(CharacterSequenceId::JumpOff),
            Jump::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                CharacterSequenceId::Jump,
                SequenceStatus::End,
                &kinematics,
                Mirrored::default(),
                Grounding::default(),
                RunCounter::default()
            ))
        );
    }
}
