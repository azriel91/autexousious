use object_model::{config::object::CharacterSequenceId, entity::SequenceStatus};

use character::sequence_handler::CharacterSequenceHandler;
use CharacterSequenceUpdateComponents;

#[derive(Debug)]
pub(crate) struct JumpAscend;

impl CharacterSequenceHandler for JumpAscend {
    fn update<'c>(
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> Option<CharacterSequenceId> {
        // Switch to jump_descend when Y axis velocity is no longer upwards.
        if components.kinematics.velocity[1] <= 0. {
            Some(CharacterSequenceId::JumpDescend)
        } else if components.sequence_status == SequenceStatus::End {
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
        entity::{CharacterStatus, Grounding, Kinematics, Mirrored, RunCounter, SequenceStatus},
    };

    use super::JumpAscend;
    use character::sequence_handler::CharacterSequenceHandler;
    use CharacterSequenceUpdateComponents;

    #[test]
    fn no_update_when_sequence_not_ended() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut kinematics = Kinematics::default();
        kinematics.velocity[1] = 1.;

        assert_eq!(
            None,
            JumpAscend::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                CharacterSequenceId::JumpAscend,
                SequenceStatus::default(),
                &kinematics,
                Mirrored::default(),
                Grounding::Airborne,
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn restarts_jump_ascend_when_sequence_ends() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut kinematics = Kinematics::default();
        kinematics.velocity[1] = 1.;

        assert_eq!(
            Some(CharacterSequenceId::JumpAscend),
            JumpAscend::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                CharacterSequenceId::JumpAscend,
                SequenceStatus::End,
                &kinematics,
                Mirrored::default(),
                Grounding::Airborne,
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn switches_to_jump_descend_when_y_velocity_is_zero_or_downwards() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut downwards_kinematics = Kinematics::default();
        downwards_kinematics.velocity[1] = -1.;

        vec![Kinematics::default(), downwards_kinematics]
            .into_iter()
            .for_each(|kinematics| {
                assert_eq!(
                    Some(CharacterSequenceId::JumpDescend),
                    JumpAscend::update(CharacterSequenceUpdateComponents::new(
                        &input,
                        &CharacterStatus::default(),
                        CharacterSequenceId::JumpAscend,
                        SequenceStatus::Ongoing,
                        &kinematics,
                        Mirrored::default(),
                        Grounding::Airborne,
                        RunCounter::default()
                    ))
                );
            });
    }
}
