use object_model::config::object::CharacterSequenceId;

use character::sequence_handler::CharacterSequenceHandler;
use CharacterSequenceUpdateComponents;

/// Returns the appropriate falling sequence if HP is 0.
#[derive(Debug)]
pub(crate) struct AliveCheck;

impl CharacterSequenceHandler for AliveCheck {
    fn update<'c>(
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> Option<CharacterSequenceId> {
        if components.character_status.hp == 0 {
            Some(CharacterSequenceId::FallForwardDescend)
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
            CharacterStatus, Grounding, HealthPoints, Kinematics, Mirrored, RunCounter,
            SequenceStatus,
        },
    };

    use super::AliveCheck;
    use character::sequence_handler::CharacterSequenceHandler;
    use CharacterSequenceUpdateComponents;

    #[test]
    fn returns_none_when_hp_is_above_zero() {
        assert_eq!(
            None,
            AliveCheck::update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                &CharacterStatus::default(),
                CharacterSequenceId::Stand,
                SequenceStatus::default(),
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
            Some(CharacterSequenceId::FallForwardDescend),
            AliveCheck::update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                &CharacterStatus {
                    hp: HealthPoints(0),
                },
                CharacterSequenceId::Stand,
                SequenceStatus::default(),
                &Kinematics::<f32>::default(),
                Mirrored::default(),
                Grounding::Airborne,
                RunCounter::default()
            ))
        );
    }
}
