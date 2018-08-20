use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{
        CharacterInput, CharacterStatus, CharacterStatusUpdate, Kinematics, ObjectStatusUpdate,
        RunCounter,
    },
};

use character::sequence_handler::SequenceHandlerOpt;

/// Returns a `Jump` update if jump is pressed.
#[derive(Debug)]
pub(crate) struct JumpCheck;

impl SequenceHandlerOpt for JumpCheck {
    fn update(
        input: &CharacterInput,
        character_status: &CharacterStatus,
        _kinematics: &Kinematics<f32>,
    ) -> Option<CharacterStatusUpdate> {
        // TODO: Don't handle action buttons in `SequenceHandler`s. Instead, each sequence has
        // default sequence update IDs for each action button, which are overridden by
        // configuration.
        if input.jump {
            let run_counter = if character_status.run_counter == RunCounter::Unused {
                None
            } else {
                Some(RunCounter::Unused)
            };
            Some(CharacterStatusUpdate::new(
                run_counter,
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
    use object_model::{
        config::object::{CharacterSequenceId, SequenceState},
        entity::{
            CharacterInput, CharacterStatus, CharacterStatusUpdate, Grounding, Kinematics,
            ObjectStatus, ObjectStatusUpdate, RunCounter,
        },
    };

    use super::JumpCheck;
    use character::sequence_handler::SequenceHandlerOpt;

    #[test]
    fn returns_none_when_jump_is_not_pressed() {
        let mut character_input = CharacterInput::default();
        character_input.jump = false;
        assert_eq!(
            None,
            JumpCheck::update(
                &character_input,
                &CharacterStatus {
                    run_counter: RunCounter::Unused,
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Stand,
                        ..Default::default()
                    }
                },
                &Kinematics::<f32>::default()
            )
        );
    }

    #[test]
    fn switches_to_jump_when_jump_is_pressed() {
        let mut character_input = CharacterInput::default();
        character_input.jump = true;
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
                &character_input,
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

    #[test]
    fn switches_run_counter_to_unused_when_jump_is_pressed() {
        let mut character_input = CharacterInput::default();
        character_input.jump = true;
        assert_eq!(
            Some(CharacterStatusUpdate {
                run_counter: Some(RunCounter::Unused),
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Jump),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                },
            }),
            JumpCheck::update(
                &character_input,
                &CharacterStatus {
                    run_counter: RunCounter::Decrease(1),
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Stand,
                        grounding: Grounding::Airborne,
                        ..Default::default()
                    },
                },
                &Kinematics::<f32>::default()
            )
        );
    }
}
