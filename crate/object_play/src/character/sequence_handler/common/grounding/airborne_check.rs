use game_input::ControllerInput;
use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{
        CharacterStatus, CharacterStatusUpdate, Grounding, Kinematics, ObjectStatus,
        ObjectStatusUpdate, RunCounter,
    },
};

use character::sequence_handler::SequenceHandler;

/// Returns a `JumpDescend` update if the grounding is `Airborne`.
#[derive(Debug)]
pub(crate) struct AirborneCheck;

impl SequenceHandler for AirborneCheck {
    fn update(
        _input: &ControllerInput,
        _character_status: &CharacterStatus,
        object_status: &ObjectStatus<CharacterSequenceId>,
        _kinematics: &Kinematics<f32>,
        _run_counter: RunCounter,
    ) -> Option<(
        CharacterStatusUpdate,
        ObjectStatusUpdate<CharacterSequenceId>,
    )> {
        if object_status.grounding == Grounding::Airborne {
            Some((
                CharacterStatusUpdate::default(),
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
    use game_input::ControllerInput;
    use object_model::{
        config::object::{CharacterSequenceId, SequenceState},
        entity::{
            CharacterStatus, CharacterStatusUpdate, Grounding, Kinematics, ObjectStatus,
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
                &ControllerInput::default(),
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
    fn switches_to_jump_descend_when_grounding_is_airborne() {
        assert_eq!(
            Some((
                CharacterStatusUpdate::default(),
                ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::JumpDescend),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                }
            )),
            AirborneCheck::update(
                &ControllerInput::default(),
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
