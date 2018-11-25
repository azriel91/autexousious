use game_input::ControllerInput;
use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{
        CharacterStatus, CharacterStatusUpdate, Kinematics, ObjectStatus, ObjectStatusUpdate,
        RunCounter,
    },
};

use character::sequence_handler::SequenceHandler;

/// Determines whether to switch to the `Walk` sequence based on Z input.
///
/// This should only be called from the Stand sequence handler.
#[derive(Debug)]
pub(crate) struct StandZMovementCheck;

impl SequenceHandler for StandZMovementCheck {
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
        if input.z_axis_value != 0. {
            let sequence_id = Some(CharacterSequenceId::Walk);
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

    use super::StandZMovementCheck;
    use character::sequence_handler::SequenceHandler;

    #[test]
    fn no_change_when_no_z_input() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            None,
            StandZMovementCheck::update(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus::default(),
                &Kinematics::default(),
                RunCounter::default()
            )
        );
    }

    #[test]
    fn walk_when_z_axis_is_non_zero() {
        let input = ControllerInput::new(0., 1., false, false, false, false);

        assert_eq!(
            Some((
                CharacterStatusUpdate::default(),
                ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Walk),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                }
            )),
            StandZMovementCheck::update(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus::default(),
                &Kinematics::default(),
                RunCounter::default()
            )
        );
    }
}
