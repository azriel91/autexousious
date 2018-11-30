use object_model::{config::object::CharacterSequenceId, entity::SequenceStatus};

use CharacterSequenceUpdateComponents;

#[derive(Debug)]
pub(crate) struct SwitchSequenceOnDescend(
    /// The sequence to switch to.
    pub CharacterSequenceId,
);

impl SwitchSequenceOnDescend {
    pub fn update<'c>(
        &self,
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> Option<CharacterSequenceId> {
        // Switch to descend_sequence when Y axis velocity is no longer upwards.
        if components.kinematics.velocity[1] <= 0. {
            Some(self.0)
        } else if components.sequence_status == SequenceStatus::End {
            Some(components.character_sequence_id)
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

    use super::SwitchSequenceOnDescend;
    use CharacterSequenceUpdateComponents;

    #[test]
    fn no_update_when_sequence_not_ended() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut kinematics = Kinematics::default();
        kinematics.velocity[1] = 1.;

        assert_eq!(
            None,
            SwitchSequenceOnDescend(CharacterSequenceId::FallForwardDescend).update(
                CharacterSequenceUpdateComponents::new(
                    &input,
                    &CharacterStatus::default(),
                    CharacterSequenceId::FallForwardAscend,
                    SequenceStatus::default(),
                    &kinematics,
                    Mirrored::default(),
                    Grounding::Airborne,
                    RunCounter::default()
                )
            )
        );
    }

    #[test]
    fn restarts_ascend_sequence_when_sequence_ends() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut kinematics = Kinematics::default();
        kinematics.velocity[1] = 1.;

        assert_eq!(
            Some(CharacterSequenceId::FallForwardAscend),
            SwitchSequenceOnDescend(CharacterSequenceId::FallForwardDescend).update(
                CharacterSequenceUpdateComponents::new(
                    &input,
                    &CharacterStatus::default(),
                    CharacterSequenceId::FallForwardAscend,
                    SequenceStatus::End,
                    &kinematics,
                    Mirrored::default(),
                    Grounding::Airborne,
                    RunCounter::default()
                )
            )
        );
    }

    #[test]
    fn switches_to_descend_sequence_when_y_velocity_is_zero_or_downwards() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut downwards_kinematics = Kinematics::default();
        downwards_kinematics.velocity[1] = -1.;

        vec![Kinematics::default(), downwards_kinematics]
            .into_iter()
            .for_each(|kinematics| {
                assert_eq!(
                    Some(CharacterSequenceId::FallForwardDescend),
                    SwitchSequenceOnDescend(CharacterSequenceId::FallForwardDescend).update(
                        CharacterSequenceUpdateComponents::new(
                            &input,
                            &CharacterStatus::default(),
                            CharacterSequenceId::FallForwardAscend,
                            SequenceStatus::Ongoing,
                            &kinematics,
                            Mirrored::default(),
                            Grounding::Airborne,
                            RunCounter::default()
                        )
                    )
                );
            });
    }
}
