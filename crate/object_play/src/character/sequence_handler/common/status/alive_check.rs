use object_model::{
    config::object::CharacterSequenceId,
    entity::{ObjectStatusUpdate, SequenceStatus},
};

use character::sequence_handler::SequenceHandler;
use CharacterSequenceUpdateComponents;

/// Returns the appropriate falling sequence if HP is 0.
#[derive(Debug)]
pub(crate) struct AliveCheck;

impl SequenceHandler for AliveCheck {
    fn update<'c>(
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> Option<ObjectStatusUpdate<CharacterSequenceId>> {
        if components.character_status.hp == 0 {
            Some(ObjectStatusUpdate::new(
                Some(CharacterSequenceId::FallForwardDescend),
                Some(SequenceStatus::Begin),
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
        config::object::CharacterSequenceId,
        entity::{
            CharacterStatus, Grounding, HealthPoints, Kinematics, Mirrored, ObjectStatus,
            ObjectStatusUpdate, RunCounter, SequenceStatus,
        },
    };

    use super::AliveCheck;
    use character::sequence_handler::SequenceHandler;
    use CharacterSequenceUpdateComponents;

    #[test]
    fn returns_none_when_hp_is_above_zero() {
        assert_eq!(
            None,
            AliveCheck::update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Stand,
                    ..Default::default()
                },
                &Kinematics::<f32>::default(),
                Mirrored::default(),
                Grounding::default(),
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn switches_to_fall_forward_descend_when_hp_is_zero() {
        assert_eq!(
            Some(ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::FallForwardDescend),
                sequence_status: Some(SequenceStatus::Begin),
            }),
            AliveCheck::update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                &CharacterStatus {
                    hp: HealthPoints(0),
                },
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Stand,
                    ..Default::default()
                },
                &Kinematics::<f32>::default(),
                Mirrored::default(),
                Grounding::Airborne,
                RunCounter::default()
            ))
        );
    }
}
