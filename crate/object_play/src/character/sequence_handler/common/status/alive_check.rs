use game_input::ControllerInput;
use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{
        CharacterStatus, CharacterStatusUpdate, Kinematics, ObjectStatus, ObjectStatusUpdate,
    },
};

use character::sequence_handler::SequenceHandler;

/// Returns the appropriate falling sequence if HP is 0.
#[derive(Debug)]
pub(crate) struct AliveCheck;

impl SequenceHandler for AliveCheck {
    fn update(
        _input: &ControllerInput,
        character_status: &CharacterStatus,
        _object_status: &ObjectStatus<CharacterSequenceId>,
        _kinematics: &Kinematics<f32>,
    ) -> Option<(
        CharacterStatusUpdate,
        ObjectStatusUpdate<CharacterSequenceId>,
    )> {
        if character_status.hp == 0 {
            Some((
                CharacterStatusUpdate::default(),
                ObjectStatusUpdate::new(
                    Some(CharacterSequenceId::FallForwardDescend),
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
            CharacterStatus, CharacterStatusUpdate, Grounding, HealthPoints, Kinematics,
            ObjectStatus, ObjectStatusUpdate,
        },
    };

    use super::AliveCheck;
    use character::sequence_handler::SequenceHandler;

    #[test]
    fn returns_none_when_hp_is_above_zero() {
        assert_eq!(
            None,
            AliveCheck::update(
                &ControllerInput::default(),
                &CharacterStatus {
                    hp: HealthPoints(100),
                    ..Default::default()
                },
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Stand,
                    ..Default::default()
                },
                &Kinematics::<f32>::default()
            )
        );
    }

    #[test]
    fn switches_to_fall_forward_descend_when_hp_is_zero() {
        assert_eq!(
            Some((
                CharacterStatusUpdate::default(),
                ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::FallForwardDescend),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                }
            )),
            AliveCheck::update(
                &ControllerInput::default(),
                &CharacterStatus {
                    hp: HealthPoints(0),
                    ..Default::default()
                },
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Stand,
                    grounding: Grounding::Airborne,
                    ..Default::default()
                },
                &Kinematics::<f32>::default()
            )
        );
    }
}
