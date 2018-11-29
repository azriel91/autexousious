use object_model::{
    config::object::{CharacterSequenceId, SequenceStatus},
    entity::ObjectStatusUpdate,
};

use character::sequence_handler::SequenceHandler;
use CharacterSequenceUpdateComponents;

/// Returns a `Jump` update if jump is pressed.
#[derive(Debug)]
pub(crate) struct JumpCheck;

impl SequenceHandler for JumpCheck {
    fn update<'c>(
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> Option<ObjectStatusUpdate<CharacterSequenceId>> {
        // TODO: Don't handle action buttons in `CharacterSequenceHandler`s. Instead, each sequence
        // has default sequence update IDs for each action button, which are overridden by
        // configuration.
        if components.controller_input.jump {
            Some(ObjectStatusUpdate::new(
                Some(CharacterSequenceId::Jump),
                Some(SequenceStatus::Begin),
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
        config::object::{CharacterSequenceId, SequenceStatus},
        entity::{
            CharacterStatus, Grounding, Kinematics, Mirrored, ObjectStatus, ObjectStatusUpdate,
            RunCounter,
        },
    };

    use super::JumpCheck;
    use character::sequence_handler::SequenceHandler;
    use CharacterSequenceUpdateComponents;

    #[test]
    fn returns_none_when_jump_is_not_pressed() {
        let mut controller_input = ControllerInput::default();
        controller_input.jump = false;
        assert_eq!(
            None,
            JumpCheck::update(CharacterSequenceUpdateComponents::new(
                &controller_input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Stand,
                    ..Default::default()
                },
                &Kinematics::<f32>::default(),
                Mirrored::default(),
                Grounding::default(),
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn switches_to_jump_when_jump_is_pressed() {
        let mut controller_input = ControllerInput::default();
        controller_input.jump = true;
        assert_eq!(
            Some(ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::Jump),
                sequence_status: Some(SequenceStatus::Begin),
            }),
            JumpCheck::update(CharacterSequenceUpdateComponents::new(
                &controller_input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Stand,
                    ..Default::default()
                },
                &Kinematics::<f32>::default(),
                Mirrored::default(),
                Grounding::Airborne,
                RunCounter::default()
            ))
        );
    }
}
