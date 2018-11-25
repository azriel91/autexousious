use game_input::ControllerInput;
use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{
        CharacterStatus, CharacterStatusUpdate, Kinematics, ObjectStatus, ObjectStatusUpdate,
        RunCounter,
    },
};

use character::sequence_handler::SequenceHandler;

/// Returns a `Jump` update if jump is pressed.
#[derive(Debug)]
pub(crate) struct JumpCheck;

impl SequenceHandler for JumpCheck {
    fn update(
        input: &ControllerInput,
        _character_status: &CharacterStatus,
        _object_status: &ObjectStatus<CharacterSequenceId>,
        _kinematics: &Kinematics<f32>,
        _run_counter: RunCounter,
    ) -> Option<(
        CharacterStatusUpdate,
        ObjectStatusUpdate<CharacterSequenceId>,
    )> {
        // TODO: Don't handle action buttons in `CharacterSequenceHandler`s. Instead, each sequence
        // has default sequence update IDs for each action button, which are overridden by
        // configuration.
        if input.jump {
            Some((
                CharacterStatusUpdate::default(),
                ObjectStatusUpdate::new(
                    Some(CharacterSequenceId::Jump),
                    Some(SequenceState::Begin),
                    None,
                    None,
                ),
            ))
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
            ObjectStatusUpdate, RunCounter,
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
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Stand,
                    ..Default::default()
                },
                &Kinematics::<f32>::default(),
                RunCounter::default()
            )
        );
    }

    #[test]
    fn switches_to_jump_when_jump_is_pressed() {
        let mut controller_input = ControllerInput::default();
        controller_input.jump = true;
        assert_eq!(
            Some((
                CharacterStatusUpdate::default(),
                ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Jump),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                }
            )),
            JumpCheck::update(
                &controller_input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Stand,
                    grounding: Grounding::Airborne,
                    ..Default::default()
                },
                &Kinematics::<f32>::default(),
                RunCounter::default()
            )
        );
    }
}
