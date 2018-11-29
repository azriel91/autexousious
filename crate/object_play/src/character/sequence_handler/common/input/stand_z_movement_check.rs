use object_model::{
    config::object::CharacterSequenceId,
    entity::{ObjectStatusUpdate, SequenceStatus},
};

use character::sequence_handler::SequenceHandler;
use CharacterSequenceUpdateComponents;

/// Determines whether to switch to the `Walk` sequence based on Z input.
///
/// This should only be called from the Stand sequence handler.
#[derive(Debug)]
pub(crate) struct StandZMovementCheck;

impl SequenceHandler for StandZMovementCheck {
    fn update<'c>(
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> Option<ObjectStatusUpdate<CharacterSequenceId>> {
        if components.controller_input.z_axis_value != 0. {
            let sequence_id = Some(CharacterSequenceId::Walk);
            let sequence_status = Some(SequenceStatus::Begin);

            Some(ObjectStatusUpdate::new(sequence_id, sequence_status))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use game_input::ControllerInput;
    use object_model::{
        config::object::CharacterSequenceId,
        entity::{
            CharacterStatus, Grounding, Kinematics, Mirrored, ObjectStatus, ObjectStatusUpdate,
            RunCounter, SequenceStatus,
        },
    };

    use super::StandZMovementCheck;
    use character::sequence_handler::SequenceHandler;
    use CharacterSequenceUpdateComponents;

    #[test]
    fn no_change_when_no_z_input() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            None,
            StandZMovementCheck::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus::default(),
                &Kinematics::default(),
                Mirrored::default(),
                Grounding::default(),
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn walk_when_z_axis_is_non_zero() {
        let input = ControllerInput::new(0., 1., false, false, false, false);

        assert_eq!(
            Some(ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::Walk),
                sequence_status: Some(SequenceStatus::Begin),
            }),
            StandZMovementCheck::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus::default(),
                &Kinematics::default(),
                Mirrored::default(),
                Grounding::default(),
                RunCounter::default()
            ))
        );
    }
}
