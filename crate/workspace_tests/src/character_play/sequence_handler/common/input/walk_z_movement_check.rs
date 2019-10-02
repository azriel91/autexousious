#[cfg(test)]
mod tests {
    use character_model::{config::CharacterSequenceName, play::RunCounter};
    use game_input::ControllerInput;
    use kinematic_model::config::{Position, Velocity};
    use object_model::play::{Grounding, HealthPoints, Mirrored};
    use sequence_model::play::SequenceStatus;

    use character_play::{
        sequence_handler::{common::input::WalkZMovementCheck, CharacterSequenceHandler},
        CharacterSequenceUpdateComponents,
    };

    #[test]
    fn none_when_no_z_input() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            None,
            WalkZMovementCheck::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceName::Walk,
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
    fn no_change_when_z_axis_non_zero() {
        vec![1., -1.].into_iter().for_each(|z_input| {
            let input = ControllerInput::new(0., z_input, false, false, false, false);

            assert_eq!(
                None,
                WalkZMovementCheck::update(CharacterSequenceUpdateComponents::new(
                    &input,
                    HealthPoints::default(),
                    CharacterSequenceName::Walk,
                    SequenceStatus::default(),
                    &Position::default(),
                    &Velocity::default(),
                    Mirrored::default(),
                    Grounding::OnGround,
                    RunCounter::default()
                ))
            );
        });
    }

    #[test]
    fn restarts_walk_when_sequence_ended() {
        vec![1., -1.].into_iter().for_each(|z_input| {
            let input = ControllerInput::new(0., z_input, false, false, false, false);

            assert_eq!(
                Some(CharacterSequenceName::Walk),
                WalkZMovementCheck::update(CharacterSequenceUpdateComponents::new(
                    &input,
                    HealthPoints::default(),
                    CharacterSequenceName::Walk,
                    SequenceStatus::End,
                    &Position::default(),
                    &Velocity::default(),
                    Mirrored(false),
                    Grounding::OnGround,
                    RunCounter::Increase(1)
                ))
            );
        });
    }
}
