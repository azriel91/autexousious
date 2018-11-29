use object_model::{
    config::object::CharacterSequenceId,
    entity::{ObjectStatusUpdate, SequenceStatus},
};

use character::sequence_handler::CharacterSequenceHandler;
use CharacterSequenceUpdateComponents;

#[derive(Debug)]
pub(crate) struct JumpOff;

impl CharacterSequenceHandler for JumpOff {
    fn update<'c>(
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> ObjectStatusUpdate<CharacterSequenceId> {
        let mut object_status_update = ObjectStatusUpdate::default();

        // Switch to jump_descend when Y axis velocity is no longer upwards.
        if components.kinematics.velocity[1] <= 0. {
            object_status_update.sequence_id = Some(CharacterSequenceId::JumpDescend);
        } else if components.sequence_status == SequenceStatus::End {
            object_status_update.sequence_id = Some(CharacterSequenceId::JumpAscend);
        }

        object_status_update
    }
}

#[cfg(test)]
mod test {
    use game_input::ControllerInput;
    use object_model::{
        config::object::CharacterSequenceId,
        entity::{
            CharacterStatus, Grounding, Kinematics, Mirrored, ObjectStatus, ObjectStatusUpdate,
            RunCounter, SequenceStatus,
        },
    };

    use super::JumpOff;
    use character::sequence_handler::CharacterSequenceHandler;
    use CharacterSequenceUpdateComponents;

    #[test]
    fn no_update_when_sequence_not_ended() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut kinematics = Kinematics::default();
        kinematics.velocity[1] = 1.;

        assert_eq!(
            ObjectStatusUpdate::default(),
            JumpOff::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::JumpOff,
                },
                SequenceStatus::default(),
                &kinematics,
                Mirrored::default(),
                Grounding::default(),
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn switches_to_jump_ascend_when_sequence_ends() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut kinematics = Kinematics::default();
        kinematics.velocity[1] = 1.;

        assert_eq!(
            ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::JumpAscend),
            },
            JumpOff::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::JumpOff,
                },
                SequenceStatus::End,
                &kinematics,
                Mirrored::default(),
                Grounding::default(),
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
                    ObjectStatusUpdate {
                        sequence_id: Some(CharacterSequenceId::JumpDescend),
                    },
                    JumpOff::update(CharacterSequenceUpdateComponents::new(
                        &input,
                        &CharacterStatus::default(),
                        &ObjectStatus {
                            sequence_id: CharacterSequenceId::JumpOff,
                        },
                        SequenceStatus::Ongoing,
                        &kinematics,
                        Mirrored::default(),
                        Grounding::default(),
                        RunCounter::default()
                    ))
                );
            });
    }
}
