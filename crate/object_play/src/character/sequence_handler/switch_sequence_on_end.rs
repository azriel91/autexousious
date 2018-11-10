use game_input::ControllerInput;
use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{CharacterStatus, CharacterStatusUpdate, Kinematics},
};

#[derive(Debug)]
pub(crate) struct SwitchSequenceOnEnd(
    /// The sequence to switch to.
    pub CharacterSequenceId,
);

impl SwitchSequenceOnEnd {
    pub fn update(
        &self,
        _controller_input: &ControllerInput,
        character_status: &CharacterStatus,
        _kinematics: &Kinematics<f32>,
    ) -> CharacterStatusUpdate {
        let mut update = CharacterStatusUpdate::default();
        if character_status.object_status.sequence_state == SequenceState::End {
            update.object_status.sequence_id = Some(self.0);
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
            CharacterStatus, CharacterStatusUpdate, Kinematics, ObjectStatus, ObjectStatusUpdate,
        },
    };

    use super::SwitchSequenceOnEnd;

    #[test]
    fn no_update_when_sequence_not_ended() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::default(),
            SwitchSequenceOnEnd(CharacterSequenceId::Stand).update(
                &input,
                &CharacterStatus {
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Flinch0,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn reverts_to_stand_when_sequence_ended() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate {
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Stand),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                },
                ..Default::default()
            },
            SwitchSequenceOnEnd(CharacterSequenceId::Stand).update(
                &input,
                &CharacterStatus {
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Flinch0,
                        sequence_state: SequenceState::End,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );
    }
}
