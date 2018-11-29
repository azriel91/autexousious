use object_model::{
    config::object::CharacterSequenceId,
    entity::{ObjectStatusUpdate, SequenceStatus},
};

use CharacterSequenceUpdateComponents;

#[derive(Debug)]
pub(crate) struct SwitchSequenceOnDescend(
    /// The sequence to switch to.
    pub CharacterSequenceId,
);

impl SwitchSequenceOnDescend {
    pub fn update<'c>(
        &self,
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> ObjectStatusUpdate<CharacterSequenceId> {
        let mut object_status_update = ObjectStatusUpdate::default();
        // Switch to descend_sequence when Y axis velocity is no longer upwards.
        if components.kinematics.velocity[1] <= 0. {
            object_status_update.sequence_id = Some(self.0);
            object_status_update.sequence_status = Some(SequenceStatus::Begin);
        } else if components.object_status.sequence_status == SequenceStatus::End {
            object_status_update.sequence_id = Some(components.object_status.sequence_id);
            object_status_update.sequence_status = Some(SequenceStatus::Begin);
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

    use super::SwitchSequenceOnDescend;
    use CharacterSequenceUpdateComponents;

    #[test]
    fn no_update_when_sequence_not_ended() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut kinematics = Kinematics::default();
        kinematics.velocity[1] = 1.;

        assert_eq!(
            ObjectStatusUpdate::default(),
            SwitchSequenceOnDescend(CharacterSequenceId::FallForwardDescend).update(
                CharacterSequenceUpdateComponents::new(
                    &input,
                    &CharacterStatus::default(),
                    &ObjectStatus {
                        sequence_id: CharacterSequenceId::FallForwardAscend,
                        ..Default::default()
                    },
                    &kinematics,
                    Mirrored::default(),
                    Grounding::Airborne,
                    RunCounter::default()
                )
            )
        );
    }

    #[test]
    fn restarts_ascend_sequence_when_sequence_ends() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut kinematics = Kinematics::default();
        kinematics.velocity[1] = 1.;

        assert_eq!(
            ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::FallForwardAscend),
                sequence_status: Some(SequenceStatus::Begin),
            },
            SwitchSequenceOnDescend(CharacterSequenceId::FallForwardDescend).update(
                CharacterSequenceUpdateComponents::new(
                    &input,
                    &CharacterStatus::default(),
                    &ObjectStatus {
                        sequence_id: CharacterSequenceId::FallForwardAscend,
                        sequence_status: SequenceStatus::End,
                    },
                    &kinematics,
                    Mirrored::default(),
                    Grounding::Airborne,
                    RunCounter::default()
                )
            )
        );
    }

    #[test]
    fn switches_to_descend_sequence_when_y_velocity_is_zero_or_downwards() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut downwards_kinematics = Kinematics::default();
        downwards_kinematics.velocity[1] = -1.;

        vec![Kinematics::default(), downwards_kinematics]
            .into_iter()
            .for_each(|kinematics| {
                assert_eq!(
                    ObjectStatusUpdate {
                        sequence_id: Some(CharacterSequenceId::FallForwardDescend),
                        sequence_status: Some(SequenceStatus::Begin),
                    },
                    SwitchSequenceOnDescend(CharacterSequenceId::FallForwardDescend).update(
                        CharacterSequenceUpdateComponents::new(
                            &input,
                            &CharacterStatus::default(),
                            &ObjectStatus {
                                sequence_id: CharacterSequenceId::FallForwardAscend,
                                sequence_status: SequenceStatus::Ongoing,
                            },
                            &kinematics,
                            Mirrored::default(),
                            Grounding::Airborne,
                            RunCounter::default()
                        )
                    )
                );
            });
    }
}
