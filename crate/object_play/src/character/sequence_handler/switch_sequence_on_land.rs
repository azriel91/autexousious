use game_input::ControllerInput;
use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{
        CharacterStatus, CharacterStatusUpdate, Grounding, Kinematics, ObjectStatus,
        ObjectStatusUpdate,
    },
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
        _character_status: &CharacterStatus,
        object_status: &ObjectStatus<CharacterSequenceId>,
        _kinematics: &Kinematics<f32>,
    ) -> (
        CharacterStatusUpdate,
        ObjectStatusUpdate<CharacterSequenceId>,
    ) {
        let character_status_update = CharacterStatusUpdate::default();
        let mut object_status_update = ObjectStatusUpdate::default();
        if object_status.grounding == Grounding::OnGround {
            object_status_update.sequence_id = Some(self.0);
            object_status_update.sequence_state = Some(SequenceState::Begin);
        } else if object_status.sequence_state == SequenceState::End {
            object_status_update.sequence_id = Some(CharacterSequenceId::FallForwardDescend);
            object_status_update.sequence_state = Some(SequenceState::Begin);
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

    use super::SwitchSequenceOnLand;

    #[test]
    fn no_update_when_sequence_not_ended() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut kinematics = Kinematics::default();
        kinematics.velocity[1] = -1.;

        assert_eq!(
            (
                CharacterStatusUpdate::default(),
                ObjectStatusUpdate::default()
            ),
            SwitchSequenceOnLand(CharacterSequenceId::FallForwardLand).update(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::FallForwardDescend,
                    grounding: Grounding::Airborne,
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
            (
                CharacterStatusUpdate::default(),
                ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::FallForwardDescend),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                }
            ),
            SwitchSequenceOnLand(CharacterSequenceId::FallForwardLand).update(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::FallForwardDescend,
                    sequence_state: SequenceState::End,
                    grounding: Grounding::Airborne,
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
            (
                CharacterStatusUpdate::default(),
                ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::FallForwardLand),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                }
            ),
            SwitchSequenceOnLand(CharacterSequenceId::FallForwardLand).update(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::FallForwardDescend,
                    grounding: Grounding::OnGround,
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
                    (
                        CharacterStatusUpdate::default(),
                        ObjectStatusUpdate {
                            mirrored: None, // Explicitly test this.
                            ..Default::default()
                        }
                    ),
                    SwitchSequenceOnLand(CharacterSequenceId::FallForwardLand).update(
                        &input,
                        &CharacterStatus::default(),
                        &ObjectStatus {
                            sequence_id: CharacterSequenceId::FallForwardDescend,
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
