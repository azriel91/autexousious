use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{Grounding, ObjectStatusUpdate},
};

use character::sequence_handler::{CharacterSequenceHandler, SequenceHandlerUtil};
use CharacterSequenceUpdateComponents;

#[derive(Debug)]
pub(crate) struct JumpDescend;

impl CharacterSequenceHandler for JumpDescend {
    fn update<'c>(
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> ObjectStatusUpdate<CharacterSequenceId> {
        let mut object_status_update = ObjectStatusUpdate::default();
        if components.object_status.grounding == Grounding::OnGround {
            object_status_update.sequence_id = Some(CharacterSequenceId::JumpDescendLand);
            object_status_update.sequence_state = Some(SequenceState::Begin);
        } else if components.object_status.sequence_state == SequenceState::End {
            object_status_update.sequence_id = Some(CharacterSequenceId::JumpDescend);
            object_status_update.sequence_state = Some(SequenceState::Begin);
        }

        // Switch direction if user is pressing the opposite way.
        if SequenceHandlerUtil::input_opposes_direction(
            components.controller_input,
            components.object_status.mirrored,
        ) {
            object_status_update.mirrored = Some(!components.object_status.mirrored);
        }

        object_status_update
    }
}

#[cfg(test)]
mod test {
    use game_input::ControllerInput;
    use object_model::{
        config::object::{CharacterSequenceId, SequenceState},
        entity::{
            CharacterStatus, Grounding, Kinematics, Mirrored, ObjectStatus, ObjectStatusUpdate,
            RunCounter,
        },
    };

    use super::JumpDescend;
    use character::sequence_handler::CharacterSequenceHandler;
    use CharacterSequenceUpdateComponents;

    #[test]
    fn no_update_when_sequence_not_ended() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut kinematics = Kinematics::default();
        kinematics.velocity[1] = -1.;

        assert_eq!(
            ObjectStatusUpdate::default(),
            JumpDescend::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::JumpDescend,
                    grounding: Grounding::Airborne,
                    ..Default::default()
                },
                &kinematics,
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn restarts_jump_descend_when_sequence_ends() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut kinematics = Kinematics::default();
        kinematics.velocity[1] = -1.;

        assert_eq!(
            ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::JumpDescend),
                sequence_state: Some(SequenceState::Begin),
                ..Default::default()
            },
            JumpDescend::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::JumpDescend,
                    sequence_state: SequenceState::End,
                    grounding: Grounding::Airborne,
                    ..Default::default()
                },
                &kinematics,
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn jump_descend_land_when_on_ground() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut kinematics = Kinematics::default();
        kinematics.velocity[1] = -1.;

        assert_eq!(
            ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::JumpDescendLand),
                sequence_state: Some(SequenceState::Begin),
                ..Default::default()
            },
            JumpDescend::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::JumpDescend,
                    grounding: Grounding::OnGround,
                    ..Default::default()
                },
                &kinematics,
                RunCounter::default()
            ))
        );
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
                    ObjectStatusUpdate {
                        mirrored: Some(Mirrored(!mirrored)),
                        ..Default::default()
                    },
                    JumpDescend::update(CharacterSequenceUpdateComponents::new(
                        &input,
                        &CharacterStatus::default(),
                        &ObjectStatus {
                            sequence_id: CharacterSequenceId::JumpDescend,
                            grounding: Grounding::Airborne,
                            mirrored: mirrored.into(),
                            ..Default::default()
                        },
                        &kinematics,
                        RunCounter::default()
                    ))
                );
            });
    }
}
