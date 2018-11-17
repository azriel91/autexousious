use game_input::ControllerInput;
use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{
        CharacterStatus, CharacterStatusUpdate, Kinematics, ObjectStatus, ObjectStatusUpdate,
    },
};

use character::sequence_handler::{CharacterSequenceHandler, SequenceHandlerUtil};

#[derive(Debug)]
pub(crate) struct JumpAscend;

impl CharacterSequenceHandler for JumpAscend {
    fn update(
        controller_input: &ControllerInput,
        _character_status: &CharacterStatus,
        object_status: &ObjectStatus<CharacterSequenceId>,
        kinematics: &Kinematics<f32>,
    ) -> (
        CharacterStatusUpdate,
        ObjectStatusUpdate<CharacterSequenceId>,
    ) {
        let character_status_update = CharacterStatusUpdate::default();
        let mut object_status_update = ObjectStatusUpdate::default();
        // Switch to jump_descend when Y axis velocity is no longer upwards.
        if kinematics.velocity[1] <= 0. {
            object_status_update.sequence_id = Some(CharacterSequenceId::JumpDescend);
            object_status_update.sequence_state = Some(SequenceState::Begin);
        } else if object_status.sequence_state == SequenceState::End {
            object_status_update.sequence_id = Some(CharacterSequenceId::JumpAscend);
            object_status_update.sequence_state = Some(SequenceState::Begin);
        }

        // Switch direction if user is pressing the opposite way.
        if SequenceHandlerUtil::input_opposes_direction(controller_input, object_status.mirrored) {
            object_status_update.mirrored = Some(!object_status.mirrored);
        }

        (character_status_update, object_status_update)
    }
}

#[cfg(test)]
mod test {
    use game_input::ControllerInput;
    use object_model::{
        config::object::{CharacterSequenceId, SequenceState},
        entity::{
            CharacterStatus, CharacterStatusUpdate, Grounding, Kinematics, ObjectStatus,
            ObjectStatusUpdate,
        },
    };

    use super::JumpAscend;
    use character::sequence_handler::CharacterSequenceHandler;

    #[test]
    fn no_update_when_sequence_not_ended() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut kinematics = Kinematics::default();
        kinematics.velocity[1] = 1.;

        assert_eq!(
            (
                CharacterStatusUpdate::default(),
                ObjectStatusUpdate::default()
            ),
            JumpAscend::update(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::JumpAscend,
                    grounding: Grounding::Airborne,
                    ..Default::default()
                },
                &kinematics
            )
        );
    }

    #[test]
    fn restarts_jump_ascend_when_sequence_ends() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut kinematics = Kinematics::default();
        kinematics.velocity[1] = 1.;

        assert_eq!(
            (
                CharacterStatusUpdate::default(),
                ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::JumpAscend),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                }
            ),
            JumpAscend::update(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::JumpAscend,
                    sequence_state: SequenceState::End,
                    grounding: Grounding::Airborne,
                    ..Default::default()
                },
                &kinematics
            )
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
                    (
                        CharacterStatusUpdate::default(),
                        ObjectStatusUpdate {
                            sequence_id: Some(CharacterSequenceId::JumpDescend),
                            sequence_state: Some(SequenceState::Begin),
                            ..Default::default()
                        }
                    ),
                    JumpAscend::update(
                        &input,
                        &CharacterStatus::default(),
                        &ObjectStatus {
                            sequence_id: CharacterSequenceId::JumpAscend,
                            sequence_state: SequenceState::Ongoing,
                            grounding: Grounding::Airborne,
                            ..Default::default()
                        },
                        &kinematics
                    )
                );
            });
    }

    #[test]
    fn switches_mirror_when_pressing_opposite_direction() {
        vec![(-1., false), (1., true)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);
                let mut kinematics = Kinematics::default();
                kinematics.velocity[1] = 1.;

                assert_eq!(
                    (
                        CharacterStatusUpdate::default(),
                        ObjectStatusUpdate {
                            mirrored: Some(!mirrored),
                            ..Default::default()
                        }
                    ),
                    JumpAscend::update(
                        &input,
                        &CharacterStatus::default(),
                        &ObjectStatus {
                            sequence_id: CharacterSequenceId::JumpAscend,
                            grounding: Grounding::Airborne,
                            mirrored,
                            ..Default::default()
                        },
                        &kinematics
                    )
                );
            });
    }
}
