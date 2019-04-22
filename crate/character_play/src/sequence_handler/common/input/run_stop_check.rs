use character_model::config::CharacterSequenceId;

use crate::{
    sequence_handler::{common::SequenceRepeat, CharacterSequenceHandler, SequenceHandlerUtil},
    CharacterSequenceUpdateComponents,
};

/// Determines whether to switch to the `RunStop` sequence based on X input.
///
/// This should only be called from the Walk sequence handler.
#[derive(Debug)]
pub(crate) struct RunStopCheck;

impl CharacterSequenceHandler for RunStopCheck {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceId> {
        if SequenceHandlerUtil::input_matches_direction(
            components.controller_input,
            components.mirrored,
        ) {
            SequenceRepeat::update(components)
        } else {
            Some(CharacterSequenceId::RunStop)
        }
    }
}

#[cfg(test)]
mod tests {
    use character_model::{config::CharacterSequenceId, play::RunCounter};
    use game_input::ControllerInput;
    use object_model::play::{Grounding, HealthPoints, Mirrored, Position, Velocity};
    use sequence_model::play::SequenceStatus;

    use super::RunStopCheck;
    use crate::{sequence_handler::CharacterSequenceHandler, CharacterSequenceUpdateComponents};

    #[test]
    fn none_when_input_same_direction() {
        vec![(1., false), (-1., true)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    None,
                    RunStopCheck::update(CharacterSequenceUpdateComponents::new(
                        &input,
                        HealthPoints::default(),
                        CharacterSequenceId::Walk,
                        SequenceStatus::default(),
                        &Position::default(),
                        &Velocity::default(),
                        mirrored.into(),
                        Grounding::default(),
                        RunCounter::default()
                    ))
                );
            });
    }

    #[test]
    fn run_stop_when_no_x_input() {
        let input = ControllerInput::new(0., 1., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceId::RunStop),
            RunStopCheck::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceId::Walk,
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
    fn run_stop_when_input_different_direction() {
        vec![(1., true), (-1., false)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    Some(CharacterSequenceId::RunStop),
                    RunStopCheck::update(CharacterSequenceUpdateComponents::new(
                        &input,
                        HealthPoints::default(),
                        CharacterSequenceId::Walk,
                        SequenceStatus::default(),
                        &Position::default(),
                        &Velocity::default(),
                        mirrored.into(),
                        Grounding::default(),
                        RunCounter::default()
                    ))
                );
            });
    }

    #[test]
    fn restarts_run_when_sequence_ended() {
        vec![(1., false), (-1., true)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    Some(CharacterSequenceId::Run),
                    RunStopCheck::update(CharacterSequenceUpdateComponents::new(
                        &input,
                        HealthPoints::default(),
                        CharacterSequenceId::Run,
                        SequenceStatus::End,
                        &Position::default(),
                        &Velocity::default(),
                        mirrored.into(),
                        Grounding::default(),
                        RunCounter::default()
                    ))
                );
            });
    }
}
