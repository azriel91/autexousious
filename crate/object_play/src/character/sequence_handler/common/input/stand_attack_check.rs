use object_model::{
    config::object::CharacterSequenceId,
    entity::{ObjectStatusUpdate, SequenceStatus},
};

use character::sequence_handler::SequenceHandler;
use CharacterSequenceUpdateComponents;

/// Determines whether to switch to the `StandAttack` sequence based on Attack input.
#[derive(Debug)]
pub(crate) struct StandAttackCheck;

impl SequenceHandler for StandAttackCheck {
    fn update<'c>(
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> Option<ObjectStatusUpdate<CharacterSequenceId>> {
        if components.controller_input.attack {
            let sequence_id = Some(CharacterSequenceId::StandAttack);
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

    use super::StandAttackCheck;
    use character::sequence_handler::SequenceHandler;
    use CharacterSequenceUpdateComponents;

    #[test]
    fn no_change_when_no_attack_input() {
        let input = ControllerInput::default();

        assert_eq!(
            None,
            StandAttackCheck::update(CharacterSequenceUpdateComponents::new(
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
    fn walk_when_attack_is_true() {
        let mut input = ControllerInput::default();
        input.attack = true;

        assert_eq!(
            Some(ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::StandAttack),
                sequence_status: Some(SequenceStatus::Begin),
            }),
            StandAttackCheck::update(CharacterSequenceUpdateComponents::new(
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
