use character_model::config::CharacterSequenceName;

use crate::sequence_handler::{
    switch_sequence_on_end::SwitchSequenceOnEnd, CharacterSequenceHandler,
    CharacterSequenceUpdateComponents,
};

const JUMP_DESCEND_LAND: SwitchSequenceOnEnd = SwitchSequenceOnEnd(CharacterSequenceName::Stand);

#[derive(Debug)]
pub(crate) struct JumpDescendLand;

impl CharacterSequenceHandler for JumpDescendLand {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        JUMP_DESCEND_LAND.update(components.sequence_status)
    }
}

#[cfg(test)]
mod test {
    use character_model::{config::CharacterSequenceName, play::RunCounter};
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
                CharacterSequenceName::JumpDescendLand,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored::default(),
                Grounding::OnGround,
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn reverts_to_stand_when_sequence_ended() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceName::Stand),
            JumpDescendLand::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceName::JumpDescendLand,
                SequenceStatus::End,
                &Position::default(),
                &Velocity::default(),
                Mirrored::default(),
                Grounding::OnGround,
                RunCounter::default()
            ))
        );
    }
}
