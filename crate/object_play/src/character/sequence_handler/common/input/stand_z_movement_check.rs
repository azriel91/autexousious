use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{CharacterInput, CharacterStatus, Kinematics, ObjectStatusUpdate},
};

use character::sequence_handler::SequenceHandler;

/// Determines whether to swithc to the `Walk` or `Run` sequence based on X input.
///
/// This should only be called from the Stand or Walk sequence handlers.
#[derive(Debug)]
pub(crate) struct StandZMovementCheck;

impl SequenceHandler for StandZMovementCheck {
    fn update(
        input: &CharacterInput,
        _character_status: &CharacterStatus,
        _kinematics: &Kinematics<f32>,
    ) -> Option<ObjectStatusUpdate<CharacterSequenceId>> {
        if input.z_axis_value != 0. {
            let sequence_id = Some(CharacterSequenceId::Walk);
            let sequence_state = Some(SequenceState::Begin);
            let mirrored = None;
            let grounding = None;

            Some(ObjectStatusUpdate::new(
                sequence_id,
                sequence_state,
                mirrored,
                grounding,
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
        entity::{CharacterInput, CharacterStatus, Kinematics, ObjectStatusUpdate, RunCounter},
    };

    use super::StandZMovementCheck;
    use character::sequence_handler::SequenceHandler;

    #[test]
    fn no_change_when_no_z_input() {
        let input = CharacterInput::new(0., 0., false, false, false, false);

        assert_eq!(
            None,
            StandZMovementCheck::update(
                &input,
                &CharacterStatus::default(),
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn walk_when_z_axis_is_non_zero_and_decrements_tick_count() {
        let input = CharacterInput::new(0., 1., false, false, false, false);

        assert_eq!(
            Some(ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::Walk),
                sequence_state: Some(SequenceState::Begin),
                ..Default::default()
            }),
            StandZMovementCheck::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Decrease(10),
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );
    }
}
