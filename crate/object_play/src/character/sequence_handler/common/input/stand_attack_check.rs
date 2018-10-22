use game_input::ControllerInput;
use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{CharacterStatus, CharacterStatusUpdate, Kinematics, ObjectStatusUpdate},
};

use character::sequence_handler::SequenceHandler;

/// Determines whether to switch to the `StandAttack` sequence based on Attack input.
#[derive(Debug)]
pub(crate) struct StandAttackCheck;

impl SequenceHandler for StandAttackCheck {
    fn update(
        input: &ControllerInput,
        _character_status: &CharacterStatus,
        _kinematics: &Kinematics<f32>,
    ) -> Option<CharacterStatusUpdate> {
        if input.attack {
            let sequence_id = Some(CharacterSequenceId::StandAttack);
            let sequence_state = Some(SequenceState::Begin);
            let mirrored = None;
            let grounding = None;

            Some(CharacterStatusUpdate {
                object_status: ObjectStatusUpdate::new(
                    sequence_id,
                    sequence_state,
                    mirrored,
                    grounding,
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
        entity::{CharacterStatus, CharacterStatusUpdate, Kinematics, ObjectStatusUpdate},
    };

    use super::StandAttackCheck;
    use character::sequence_handler::SequenceHandler;

    #[test]
    fn no_change_when_no_attack_input() {
        let input = ControllerInput::default();

        assert_eq!(
            None,
            StandAttackCheck::update(&input, &CharacterStatus::default(), &Kinematics::default())
        );
    }

    #[test]
    fn walk_when_attack_is_true() {
        let mut input = ControllerInput::default();
        input.attack = true;

        assert_eq!(
            Some(CharacterStatusUpdate {
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::StandAttack),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                },
                ..Default::default()
            }),
            StandAttackCheck::update(&input, &CharacterStatus::default(), &Kinematics::default())
        );
    }
}
