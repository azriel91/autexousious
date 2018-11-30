use object_model::{
    config::object::CharacterSequenceId,
    entity::{Grounding, SequenceStatus},
};

use character::sequence_handler::CharacterSequenceHandler;
use CharacterSequenceUpdateComponents;

#[derive(Debug)]
pub(crate) struct JumpDescend;

impl CharacterSequenceHandler for JumpDescend {
    fn update<'c>(
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> Option<CharacterSequenceId> {
        if components.grounding == Grounding::OnGround {
            Some(CharacterSequenceId::JumpDescendLand)
        } else if components.sequence_status == SequenceStatus::End {
            Some(CharacterSequenceId::JumpDescend)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use game_input::ControllerInput;
    use object_model::{
        config::object::CharacterSequenceId,
        entity::{CharacterStatus, Grounding, Kinematics, Mirrored, RunCounter, SequenceStatus},
    };

    use super::JumpDescend;
    use character::sequence_handler::CharacterSequenceHandler;
    use CharacterSequenceUpdateComponents;

    #[test]
    fn no_update_when_sequence_not_ended() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut kinematics = Kinematics::default();
        kinematics.velocity[1] = -1.;

        assert_eq!(
            None,
            JumpDescend::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                CharacterSequenceId::JumpDescend,
                SequenceStatus::default(),
                &kinematics,
                Mirrored::default(),
                Grounding::Airborne,
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn restarts_jump_descend_when_sequence_ends() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut kinematics = Kinematics::default();
        kinematics.velocity[1] = -1.;

        assert_eq!(
            Some(CharacterSequenceId::JumpDescend),
            JumpDescend::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                CharacterSequenceId::JumpDescend,
                SequenceStatus::End,
                &kinematics,
                Mirrored::default(),
                Grounding::Airborne,
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn jump_descend_land_when_on_ground() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut kinematics = Kinematics::default();
        kinematics.velocity[1] = -1.;

        assert_eq!(
            Some(CharacterSequenceId::JumpDescendLand),
            JumpDescend::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                CharacterSequenceId::JumpDescend,
                SequenceStatus::default(),
                &kinematics,
                Mirrored::default(),
                Grounding::OnGround,
                RunCounter::default()
            ))
        );
    }
}
