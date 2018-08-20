use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{CharacterInput, CharacterStatus, Grounding, Kinematics, ObjectStatusUpdate},
};

use character::sequence_handler::SequenceHandler;

/// Returns a `JumpDescend` update if the grounding is `Airborne`.
#[derive(Debug)]
pub(crate) struct AirborneCheck;

impl SequenceHandler for AirborneCheck {
    fn update(
        _input: &CharacterInput,
        character_status: &CharacterStatus,
        _kinematics: &Kinematics<f32>,
    ) -> Option<ObjectStatusUpdate<CharacterSequenceId>> {
        if character_status.object_status.grounding == Grounding::Airborne {
            Some(ObjectStatusUpdate::new(
                Some(CharacterSequenceId::JumpDescend),
                Some(SequenceState::Begin),
                None,
                None,
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
            CharacterInput, CharacterStatus, Grounding, Kinematics, ObjectStatus,
            ObjectStatusUpdate, RunCounter,
        },
    };

    use super::AirborneCheck;
    use character::sequence_handler::SequenceHandler;

    #[test]
    fn returns_none_when_grounding_is_on_ground() {
        assert_eq!(
            None,
            AirborneCheck::update(
                &CharacterInput::default(),
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
    fn switches_to_jump_descend_when_grounding_is_airborne() {
        assert_eq!(
            Some(ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::JumpDescend),
                sequence_state: Some(SequenceState::Begin),
                ..Default::default()
            }),
            AirborneCheck::update(
                &CharacterInput::default(),
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
