use game_input::ControllerInput;
use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{CharacterStatus, CharacterStatusUpdate, Grounding, Kinematics},
};

#[derive(Debug)]
pub(crate) struct SwitchSequenceOnLand(
    /// The sequence to switch to.
    pub CharacterSequenceId,
);

impl SwitchSequenceOnLand {
    pub fn update(
        &self,
        _controller_input: &ControllerInput,
        character_status: &CharacterStatus,
        _kinematics: &Kinematics<f32>,
    ) -> CharacterStatusUpdate {
        let mut update = CharacterStatusUpdate::default();
        if character_status.object_status.grounding == Grounding::OnGround {
            update.object_status.sequence_id = Some(self.0);
            update.object_status.sequence_state = Some(SequenceState::Begin);
        } else if character_status.object_status.sequence_state == SequenceState::End {
            update.object_status.sequence_id = Some(CharacterSequenceId::FallForwardDescend);
            update.object_status.sequence_state = Some(SequenceState::Begin);
        }

        update
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

    use super::SwitchSequenceOnLand;

    #[test]
    fn no_update_when_sequence_not_ended() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut kinematics = Kinematics::default();
        kinematics.velocity[1] = -1.;

        assert_eq!(
            CharacterStatusUpdate::default(),
            SwitchSequenceOnLand(CharacterSequenceId::FallForwardLand).update(
                &input,
                &CharacterStatus {
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::FallForwardDescend,
                        grounding: Grounding::Airborne,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                &kinematics
            )
        );
    }

    #[test]
    fn restarts_jump_descend_when_sequence_ends() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut kinematics = Kinematics::default();
        kinematics.velocity[1] = -1.;

        assert_eq!(
            CharacterStatusUpdate {
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::FallForwardDescend),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                },
                ..Default::default()
            },
            SwitchSequenceOnLand(CharacterSequenceId::FallForwardLand).update(
                &input,
                &CharacterStatus {
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::FallForwardDescend,
                        sequence_state: SequenceState::End,
                        grounding: Grounding::Airborne,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                &kinematics
            )
        );
    }

    #[test]
    fn switches_to_land_when_on_ground() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut kinematics = Kinematics::default();
        kinematics.velocity[1] = -1.;

        assert_eq!(
            CharacterStatusUpdate {
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::FallForwardLand),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                },
                ..Default::default()
            },
            SwitchSequenceOnLand(CharacterSequenceId::FallForwardLand).update(
                &input,
                &CharacterStatus {
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::FallForwardDescend,
                        grounding: Grounding::OnGround,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                &kinematics
            )
        );
    }

    #[test]
    fn does_not_switch_mirror_when_pressing_opposite_direction() {
        vec![(-1., false), (1., true)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);
                let mut kinematics = Kinematics::default();
                kinematics.velocity[1] = -1.;

                assert_eq!(
                    CharacterStatusUpdate {
                        object_status: ObjectStatusUpdate {
                            mirrored: None, // Explicitly test this.
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    SwitchSequenceOnLand(CharacterSequenceId::FallForwardLand).update(
                        &input,
                        &CharacterStatus {
                            object_status: ObjectStatus {
                                sequence_id: CharacterSequenceId::FallForwardDescend,
                                grounding: Grounding::Airborne,
                                mirrored,
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        &kinematics
                    )
                );
            });
    }
}
