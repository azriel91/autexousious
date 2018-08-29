use game_input::ControllerInput;
use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{CharacterStatus, CharacterStatusUpdate, Kinematics, ObjectStatusUpdate},
};

use character::sequence_handler::SequenceHandler;

/// Returns a `Jump` update if jump is pressed.
#[derive(Debug)]
pub(crate) struct JumpCheck;

impl SequenceHandler for JumpCheck {
    fn update(
        input: &ControllerInput,
        _character_status: &CharacterStatus,
        _kinematics: &Kinematics<f32>,
    ) -> Option<CharacterStatusUpdate> {
        // TODO: Don't handle action buttons in `CharacterSequenceHandler`s. Instead, each sequence
        // has default sequence update IDs for each action button, which are overridden by
        // configuration.
        if input.jump {
            Some(CharacterStatusUpdate {
                object_status: ObjectStatusUpdate::new(
                    Some(CharacterSequenceId::Jump),
                    Some(SequenceState::Begin),
                    None,
                    None,
                ),
                ..Default::default()
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use game_input::ControllerInput;
    use object_model::{
        config::object::{CharacterSequenceId, SequenceState},
        entity::{
            CharacterStatus, CharacterStatusUpdate, Grounding, Kinematics, ObjectStatus,
            ObjectStatusUpdate,
        },
    };

    use super::JumpCheck;
    use character::sequence_handler::SequenceHandler;

    #[test]
    fn returns_none_when_jump_is_not_pressed() {
        let mut controller_input = ControllerInput::default();
        controller_input.jump = false;
        assert_eq!(
            None,
            JumpCheck::update(
                &controller_input,
                &CharacterStatus {
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Stand,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                &Kinematics::<f32>::default()
            )
        );
    }

    #[test]
    fn switches_to_jump_when_jump_is_pressed() {
        let mut controller_input = ControllerInput::default();
        controller_input.jump = true;
        assert_eq!(
            Some(CharacterStatusUpdate {
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Jump),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                },
                ..Default::default()
            }),
            JumpCheck::update(
                &controller_input,
                &CharacterStatus {
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Stand,
                        grounding: Grounding::Airborne,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                &Kinematics::<f32>::default()
            )
        );
    }
}
