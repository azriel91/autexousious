use game_input::ControllerInput;
use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{
        CharacterStatus, CharacterStatusUpdate, Kinematics, ObjectStatus, ObjectStatusUpdate,
        RunCounter,
    },
};

use character::sequence_handler::SequenceHandler;

/// Determines whether to switch to the `StandAttack` sequence based on Attack input.
#[derive(Debug)]
pub(crate) struct StandAttackCheck;

impl SequenceHandler for StandAttackCheck {
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
        if input.attack {
            let sequence_id = Some(CharacterSequenceId::StandAttack);
            let sequence_state = Some(SequenceState::Begin);
            let mirrored = None;
            let grounding = None;

            Some((
                CharacterStatusUpdate::default(),
                ObjectStatusUpdate::new(sequence_id, sequence_state, mirrored, grounding),
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
            CharacterStatus, CharacterStatusUpdate, Kinematics, ObjectStatus, ObjectStatusUpdate,
            RunCounter,
        },
    };

    use super::StandAttackCheck;
    use character::sequence_handler::SequenceHandler;

    #[test]
    fn no_change_when_no_attack_input() {
        let input = ControllerInput::default();

        assert_eq!(
            None,
            StandAttackCheck::update(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus::default(),
                &Kinematics::default(),
                RunCounter::default()
            )
        );
    }

    #[test]
    fn walk_when_attack_is_true() {
        let mut input = ControllerInput::default();
        input.attack = true;

        assert_eq!(
            Some((
                CharacterStatusUpdate::default(),
                ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::StandAttack),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                }
            )),
            StandAttackCheck::update(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus::default(),
                &Kinematics::default(),
                RunCounter::default()
            )
        );
    }
}
