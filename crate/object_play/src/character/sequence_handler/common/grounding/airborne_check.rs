use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{
        CharacterInput, CharacterStatus, CharacterStatusUpdate, Grounding, Kinematics,
        ObjectStatusUpdate, RunCounter,
    },
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
    ) -> Option<CharacterStatusUpdate> {
        if character_status.object_status.grounding == Grounding::Airborne {
            // TODO: Support merging updates
            let run_counter = if character_status.run_counter == RunCounter::Unused {
                None
            } else {
                Some(RunCounter::Unused)
            };
            Some(CharacterStatusUpdate::new(
                run_counter,
                ObjectStatusUpdate::new(
                    Some(CharacterSequenceId::JumpDescend),
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
            Some(CharacterStatusUpdate {
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::JumpDescend),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                },
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

    #[test]
    fn switches_run_counter_to_unused_when_grounding_is_airborne() {
        assert_eq!(
            Some(CharacterStatusUpdate {
                run_counter: Some(RunCounter::Unused),
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::JumpDescend),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                },
            }),
            AirborneCheck::update(
                &CharacterInput::default(),
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
