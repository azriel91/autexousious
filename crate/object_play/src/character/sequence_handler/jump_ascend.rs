use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{CharacterInput, CharacterStatus, CharacterStatusUpdate, Kinematics},
};

use character::sequence_handler::SequenceHandler;

#[derive(Debug)]
pub(crate) struct JumpAscend;

impl SequenceHandler for JumpAscend {
    fn update(
        _character_input: &CharacterInput,
        character_status: &CharacterStatus,
        kinematics: &Kinematics<f32>,
    ) -> CharacterStatusUpdate {
        let mut update = CharacterStatusUpdate::default();
        // Switch to airborne when Y axis velocity is no longer upwards.
        if kinematics.velocity[1] <= 0. {
            update.object_status.sequence_id = Some(CharacterSequenceId::Airborne);
            update.object_status.sequence_state = Some(SequenceState::Begin);
        } else if character_status.object_status.sequence_state == SequenceState::End {
            update.object_status.sequence_id = Some(CharacterSequenceId::JumpAscend);
            update.object_status.sequence_state = Some(SequenceState::Begin);
        }

        update
    }
}

#[cfg(test)]
mod test {
    use object_model::{
        config::object::{CharacterSequenceId, SequenceState},
        entity::{
            CharacterInput, CharacterStatus, CharacterStatusUpdate, Kinematics, ObjectStatus,
            ObjectStatusUpdate, RunCounter,
        },
    };

    use super::JumpAscend;
    use character::sequence_handler::SequenceHandler;

    #[test]
    fn no_update_when_sequence_not_ended() {
        let input = CharacterInput::new(0., 0., false, false, false, false);
        let mut kinematics = Kinematics::default();
        kinematics.velocity[1] = 1.;

        assert_eq!(
            CharacterStatusUpdate::new(None, ObjectStatusUpdate::new(None, None, None)),
            JumpAscend::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Unused,
                    ObjectStatus::new(
                        CharacterSequenceId::JumpAscend,
                        SequenceState::Ongoing,
                        false
                    )
                ),
                &kinematics
            )
        );
    }

    #[test]
    fn restarts_jump_ascend_when_sequence_ends() {
        let input = CharacterInput::new(0., 0., false, false, false, false);
        let mut kinematics = Kinematics::default();
        kinematics.velocity[1] = 1.;

        assert_eq!(
            CharacterStatusUpdate::new(
                None,
                ObjectStatusUpdate::new(
                    Some(CharacterSequenceId::JumpAscend),
                    Some(SequenceState::Begin),
                    None
                )
            ),
            JumpAscend::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Unused,
                    ObjectStatus::new(CharacterSequenceId::JumpAscend, SequenceState::End, false)
                ),
                &kinematics
            )
        );
    }

    #[test]
    fn switches_to_airborne_when_y_velocity_is_zero_or_downwards() {
        let input = CharacterInput::new(0., 0., false, false, false, false);
        let mut downwards_kinematics = Kinematics::default();
        downwards_kinematics.velocity[1] = -1.;

        vec![Kinematics::default(), downwards_kinematics]
            .into_iter()
            .for_each(|kinematics| {
                assert_eq!(
                    CharacterStatusUpdate::new(
                        None,
                        ObjectStatusUpdate::new(
                            Some(CharacterSequenceId::Airborne),
                            Some(SequenceState::Begin),
                            None
                        )
                    ),
                    JumpAscend::update(
                        &input,
                        &CharacterStatus::new(
                            RunCounter::Unused,
                            ObjectStatus::new(
                                CharacterSequenceId::JumpAscend,
                                SequenceState::Ongoing,
                                false
                            )
                        ),
                        &kinematics
                    )
                );
            });
    }
}
