#[cfg(test)]
mod test {
    use character_model::{config::CharacterSequenceName, play::RunCounter};
    use game_input_model::play::ControllerInput;
    use kinematic_model::config::{Position, Velocity};
    use mirrored_model::play::Mirrored;
    use object_model::play::{Grounding, HealthPoints};
    use sequence_model::play::SequenceStatus;

    use character_play::{
        sequence_handler::{CharacterSequenceHandler, Run},
        CharacterSequenceUpdateComponents,
    };

    #[test]
    fn jump_descend_when_airborne() {
        assert_eq!(
            Some(CharacterSequenceName::JumpDescend),
            Run::update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                HealthPoints::default(),
                CharacterSequenceName::Run,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored::default(),
                Grounding::Airborne,
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn keeps_running_when_x_axis_positive_and_non_mirrored() {
        let input = ControllerInput::new(1., 0., false, false, false, false);

        assert_eq!(
            None,
            Run::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceName::Run,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored(false),
                Grounding::OnGround,
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn keeps_running_when_x_axis_negative_and_mirrored() {
        let input = ControllerInput::new(-1., 0., false, false, false, false);

        assert_eq!(
            None,
            Run::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceName::Run,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored(true),
                Grounding::OnGround,
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn keeps_running_when_x_axis_positive_z_axis_non_zero_and_non_mirrored() {
        let input = ControllerInput::new(1., 1., false, false, false, false);

        assert_eq!(
            None,
            Run::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceName::Run,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored(false),
                Grounding::OnGround,
                RunCounter::default()
            ))
        );

        let input = ControllerInput::new(1., -1., false, false, false, false);

        assert_eq!(
            None,
            Run::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceName::Run,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored(false),
                Grounding::OnGround,
                RunCounter::default()
            ))
        );
    }
}
