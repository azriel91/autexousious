use object_model::{
    config::object::CharacterSequenceId,
    entity::{Grounding, SequenceStatus},
};

use CharacterSequenceUpdateComponents;

#[derive(Debug)]
pub(crate) struct SwitchSequenceOnLand(
    /// The sequence to switch to.
    pub CharacterSequenceId,
);

impl SwitchSequenceOnLand {
    pub fn update<'c>(
        &self,
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> Option<CharacterSequenceId> {
        if *components.grounding == Grounding::OnGround {
            Some(self.0)
        } else if *components.sequence_status == SequenceStatus::End {
            Some(CharacterSequenceId::FallForwardDescend)
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
        entity::{
            CharacterStatus, Grounding, Mirrored, Position, RunCounter, SequenceStatus, Velocity,
        },
    };

    use super::SwitchSequenceOnLand;
    use CharacterSequenceUpdateComponents;

    #[test]
    fn no_update_when_sequence_not_ended() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut velocity = Velocity::default();
        velocity[1] = -1.;

        assert_eq!(
            None,
            SwitchSequenceOnLand(CharacterSequenceId::FallForwardLand).update(
                CharacterSequenceUpdateComponents::new(
                    &input,
                    &CharacterStatus::default(),
                    &CharacterSequenceId::FallForwardDescend,
                    &SequenceStatus::default(),
                    &Position::default(),
                    &velocity,
                    &Mirrored::default(),
                    &Grounding::Airborne,
                    &RunCounter::default()
                )
            )
        );
    }

    #[test]
    fn restarts_jump_descend_when_sequence_ends() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut velocity = Velocity::default();
        velocity[1] = -1.;

        assert_eq!(
            Some(CharacterSequenceId::FallForwardDescend),
            SwitchSequenceOnLand(CharacterSequenceId::FallForwardLand).update(
                CharacterSequenceUpdateComponents::new(
                    &input,
                    &CharacterStatus::default(),
                    &CharacterSequenceId::FallForwardDescend,
                    &SequenceStatus::End,
                    &Position::default(),
                    &velocity,
                    &Mirrored::default(),
                    &Grounding::Airborne,
                    &RunCounter::default()
                )
            )
        );
    }

    #[test]
    fn switches_to_land_when_on_ground() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut velocity = Velocity::default();
        velocity[1] = -1.;

        assert_eq!(
            Some(CharacterSequenceId::FallForwardLand),
            SwitchSequenceOnLand(CharacterSequenceId::FallForwardLand).update(
                CharacterSequenceUpdateComponents::new(
                    &input,
                    &CharacterStatus::default(),
                    &CharacterSequenceId::FallForwardDescend,
                    &SequenceStatus::default(),
                    &Position::default(),
                    &velocity,
                    &Mirrored::default(),
                    &Grounding::OnGround,
                    &RunCounter::default()
                )
            )
        );
    }
}
