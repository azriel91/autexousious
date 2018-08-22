use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{
        CharacterInput, CharacterStatus, CharacterStatusUpdate, Kinematics, ObjectStatusUpdate,
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
        input: &CharacterInput,
        _character_status: &CharacterStatus,
        _kinematics: &Kinematics<f32>,
    ) -> Option<CharacterStatusUpdate> {
        if input.z_axis_value != 0. {
            let sequence_id = Some(CharacterSequenceId::Walk);
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
    use object_model::{
        config::object::{CharacterSequenceId, SequenceState},
        entity::{
            CharacterInput, CharacterStatus, CharacterStatusUpdate, Kinematics, ObjectStatusUpdate,
        },
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
    fn walk_when_z_axis_is_non_zero() {
        let input = CharacterInput::new(0., 1., false, false, false, false);

        assert_eq!(
            Some(CharacterStatusUpdate {
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Walk),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                },
                ..Default::default()
            }),
            StandZMovementCheck::update(
                &input,
                &CharacterStatus::default(),
                &Kinematics::default()
            )
        );
    }
}
