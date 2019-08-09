use character_model::config::CharacterSequenceId;

use crate::sequence_handler::{
    switch_sequence_on_end::SwitchSequenceOnEnd, CharacterSequenceHandler,
    CharacterSequenceUpdateComponents,
};

const JUMP_DESCEND_LAND: SwitchSequenceOnEnd = SwitchSequenceOnEnd(CharacterSequenceId::Stand);

#[derive(Debug)]
pub(crate) struct JumpDescendLand;

impl CharacterSequenceHandler for JumpDescendLand {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceId> {
        JUMP_DESCEND_LAND.update(components.sequence_status)
    }
}

#[cfg(test)]
mod test {
    use character_model::{config::CharacterSequenceId, play::RunCounter};
    use game_input::ControllerInput;
    use kinematic_model::config::{Position, Velocity};
    use object_model::play::{Grounding, HealthPoints, Mirrored};
    use sequence_model::play::SequenceStatus;

    use super::JumpDescendLand;
    use crate::{sequence_handler::CharacterSequenceHandler, CharacterSequenceUpdateComponents};

    #[test]
    fn no_update_when_sequence_not_ended() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            None,
            JumpDescendLand::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceId::JumpDescendLand,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored::default(),
                Grounding::default(),
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn reverts_to_stand_when_sequence_ended() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceId::Stand),
            JumpDescendLand::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceId::JumpDescendLand,
                SequenceStatus::End,
                &Position::default(),
                &Velocity::default(),
                Mirrored::default(),
                Grounding::default(),
                RunCounter::default()
            ))
        );
    }
}
