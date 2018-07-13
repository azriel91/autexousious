use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{CharacterInput, CharacterStatus, CharacterStatusUpdate, Kinematics},
};

use character::sequence_handler::SequenceHandler;

#[derive(Debug)]
pub(crate) struct AirborneLand;

impl SequenceHandler for AirborneLand {
    fn update(
        _character_input: &CharacterInput,
        character_status: &CharacterStatus,
        _kinematics: &Kinematics<f32>,
    ) -> CharacterStatusUpdate {
        let mut update = CharacterStatusUpdate::default();
        if character_status.object_status.sequence_state == SequenceState::End {
            update.object_status.sequence_id = Some(CharacterSequenceId::Stand);
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

    use super::AirborneLand;
    use character::sequence_handler::SequenceHandler;

    #[test]
    fn no_update_when_sequence_not_ended() {
        let input = CharacterInput::new(0., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(None, ObjectStatusUpdate::new(None, None, None)),
            AirborneLand::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Unused,
                    ObjectStatus::new(
                        CharacterSequenceId::AirborneLand,
                        SequenceState::Ongoing,
                        false
                    )
                ),
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn reverts_to_stand_when_sequence_ended() {
        let input = CharacterInput::new(0., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(
                None,
                ObjectStatusUpdate::new(
                    Some(CharacterSequenceId::Stand),
                    Some(SequenceState::Begin),
                    None
                )
            ),
            AirborneLand::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Unused,
                    ObjectStatus::new(CharacterSequenceId::AirborneLand, SequenceState::End, false)
                ),
                &Kinematics::default()
            )
        );
    }
}
