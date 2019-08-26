use character_model::{config::CharacterSequenceName, play::RunCounter};

use crate::{
    sequence_handler::{common::SequenceRepeat, CharacterSequenceHandler, SequenceHandlerUtil},
    CharacterSequenceUpdateComponents,
};

/// Determines whether to switch to the `Walk` or `Run` sequence based on X input.
///
/// This should only be called from the Walk sequence handler.
#[derive(Debug)]
pub(crate) struct WalkXMovementCheck;

impl CharacterSequenceHandler for WalkXMovementCheck {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        if components.controller_input.x_axis_value != 0. {
            let same_direction = SequenceHandlerUtil::input_matches_direction(
                components.controller_input,
                components.mirrored,
            );

            let sequence_id = match (components.run_counter, same_direction) {
                (RunCounter::Unused, _) | (RunCounter::Increase(_), false) => {
                    Some(CharacterSequenceName::Walk)
                }
                (RunCounter::Decrease(_), true) => Some(CharacterSequenceName::Run),
                (RunCounter::Exceeded, _)
                | (RunCounter::Decrease(_), false)
                | (RunCounter::Increase(_), true) => None,
            };

            if sequence_id.is_none() {
                SequenceRepeat::update(components)
            } else {
                sequence_id
            }
        } else {
            // The responsibility of switching to `Stand` is handled elsewhere.
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use character_model::{config::CharacterSequenceName, play::RunCounter};
    use game_input::ControllerInput;
    use kinematic_model::config::{Position, Velocity};
    use object_model::play::{Grounding, HealthPoints, Mirrored};
    use sequence_model::play::SequenceStatus;

    use super::WalkXMovementCheck;
    use crate::{sequence_handler::CharacterSequenceHandler, CharacterSequenceUpdateComponents};

    #[test]
    fn none_when_no_input() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            None,
            WalkXMovementCheck::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                &CharacterSequenceName::Walk,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored::default(),
                Grounding::default(),
                RunCounter::Increase(10)
            ))
        );
    }

    #[test]
    fn walk_when_x_axis_positive_mirror() {
        let input = ControllerInput::new(1., 0., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceName::Walk),
            WalkXMovementCheck::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                &CharacterSequenceName::Walk,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored(true),
                Grounding::default(),
                RunCounter::Increase(11)
            ))
        );
    }

    #[test]
    fn walk_when_x_axis_negative_non_mirror() {
        let input = ControllerInput::new(-1., 0., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceName::Walk),
            WalkXMovementCheck::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                &CharacterSequenceName::Walk,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored(false),
                Grounding::default(),
                RunCounter::Increase(11)
            ))
        );
    }

    #[test]
    fn restarts_walk_when_sequence_ended() {
        vec![(1., false), (-1., true)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    Some(CharacterSequenceName::Walk),
                    WalkXMovementCheck::update(CharacterSequenceUpdateComponents::new(
                        &input,
                        HealthPoints::default(),
                        &CharacterSequenceName::Walk,
                        SequenceStatus::End,
                        &Position::default(),
                        &Velocity::default(),
                        mirrored.into(),
                        Grounding::default(),
                        RunCounter::Increase(1)
                    ))
                );
            });
    }

    #[test]
    fn run_when_x_axis_positive_and_run_counter_decrease_non_mirror() {
        let input = ControllerInput::new(1., -1., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceName::Run),
            WalkXMovementCheck::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                &CharacterSequenceName::Walk,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored(false),
                Grounding::default(),
                RunCounter::Decrease(10)
            ))
        );
    }

    #[test]
    fn run_when_x_axis_negative_and_run_counter_decrease_mirror() {
        let input = ControllerInput::new(-1., -1., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceName::Run),
            WalkXMovementCheck::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                &CharacterSequenceName::Walk,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored(true),
                Grounding::default(),
                RunCounter::Decrease(10)
            ))
        );
    }
}
