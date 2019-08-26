use character_model::config::CharacterSequenceName;

use crate::{
    sequence_handler::{
        common::{
            grounding::AirborneCheck,
            input::{StandXMovementCheck, StandZMovementCheck},
            status::AliveCheck,
            SequenceRepeat,
        },
        CharacterSequenceHandler,
    },
    CharacterSequenceUpdateComponents,
};

#[derive(Debug)]
pub(crate) struct Stand;

impl CharacterSequenceHandler for Stand {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        use character_model::play::RunCounter::*;
        match components.run_counter {
            Exceeded | Increase(_) => panic!(
                "Invalid run_counter state during `Stand` sequence: `{:?}`",
                components.run_counter
            ),
            _ => {}
        };

        [
            AliveCheck::update,
            AirborneCheck::update,
            StandXMovementCheck::update,
            StandZMovementCheck::update,
            SequenceRepeat::update,
        ]
        .iter()
        .fold(None, |status_update, fn_update| {
            status_update.or_else(|| fn_update(components))
        })
    }
}

#[cfg(test)]
mod test {
    use character_model::{config::CharacterSequenceName, play::RunCounter};
    use game_input::ControllerInput;
    use kinematic_model::config::{Position, Velocity};
    use object_model::play::{Grounding, HealthPoints, Mirrored};
    use sequence_model::play::SequenceStatus;

    use super::Stand;
    use crate::{sequence_handler::CharacterSequenceHandler, CharacterSequenceUpdateComponents};

    #[test]
    fn no_change_when_no_input() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            None,
            Stand::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                &CharacterSequenceName::Stand,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored(true),
                Grounding::OnGround,
                RunCounter::Unused
            ))
        );
    }

    #[test]
    fn restarts_stand_when_no_input_and_sequence_end() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceName::Stand),
            Stand::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                &CharacterSequenceName::Stand,
                SequenceStatus::End,
                &Position::default(),
                &Velocity::default(),
                Mirrored::default(),
                Grounding::default(),
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn switches_to_jump_descend_when_airborne() {
        let input = ControllerInput::new(1., 0., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceName::JumpDescend),
            Stand::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                &CharacterSequenceName::Stand,
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
    #[should_panic(expected = "Invalid run_counter state")]
    fn panics_when_run_counter_exceeded() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        Stand::update(CharacterSequenceUpdateComponents::new(
            &input,
            HealthPoints::default(),
            &CharacterSequenceName::default(),
            SequenceStatus::default(),
            &Position::default(),
            &Velocity::default(),
            Mirrored::default(),
            Grounding::default(),
            RunCounter::Exceeded,
        ));
    } // kcov-ignore

    #[test]
    #[should_panic(expected = "Invalid run_counter state")]
    fn panics_when_run_counter_increase() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        Stand::update(CharacterSequenceUpdateComponents::new(
            &input,
            HealthPoints::default(),
            &CharacterSequenceName::default(),
            SequenceStatus::default(),
            &Position::default(),
            &Velocity::default(),
            Mirrored::default(),
            Grounding::default(),
            RunCounter::Increase(10),
        ));
    } // kcov-ignore

    #[test]
    fn walk_when_x_axis_is_positive_mirrored() {
        let input = ControllerInput::new(1., 0., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceName::Walk),
            Stand::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                &CharacterSequenceName::Stand,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored(true),
                Grounding::default(),
                RunCounter::default()
            ))
        );

        // Already facing right
        assert_eq!(
            Some(CharacterSequenceName::Walk),
            Stand::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                &CharacterSequenceName::Stand,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored(false),
                Grounding::default(),
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn walk_when_x_axis_is_negative_non_mirrored() {
        let input = ControllerInput::new(-1., 0., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceName::Walk),
            Stand::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                &CharacterSequenceName::Stand,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored(false),
                Grounding::default(),
                RunCounter::default()
            ))
        );

        // Already facing left
        assert_eq!(
            Some(CharacterSequenceName::Walk),
            Stand::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                &CharacterSequenceName::Stand,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored(true),
                Grounding::default(),
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn walk_when_x_and_z_axes_are_non_zero() {
        let input = ControllerInput::new(1., 1., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceName::Walk),
            Stand::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                &CharacterSequenceName::Stand,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored(false),
                Grounding::default(),
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn run_when_run_counter_decrease_x_input_same_direction() {
        vec![(1., false), (-1., true)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    Some(CharacterSequenceName::Run),
                    Stand::update(CharacterSequenceUpdateComponents::new(
                        &input,
                        HealthPoints::default(),
                        &CharacterSequenceName::Stand,
                        SequenceStatus::default(),
                        &Position::default(),
                        &Velocity::default(),
                        mirrored.into(),
                        Grounding::default(),
                        RunCounter::Decrease(10)
                    ))
                );
            });
    }

    #[test]
    fn walk_when_run_counter_decrease_x_input_different_direction() {
        vec![(1., true), (-1., false)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    Some(CharacterSequenceName::Walk),
                    Stand::update(CharacterSequenceUpdateComponents::new(
                        &input,
                        HealthPoints::default(),
                        &CharacterSequenceName::Stand,
                        SequenceStatus::default(),
                        &Position::default(),
                        &Velocity::default(),
                        mirrored.into(),
                        Grounding::default(),
                        RunCounter::Decrease(10)
                    ))
                );
            });
    }
}
