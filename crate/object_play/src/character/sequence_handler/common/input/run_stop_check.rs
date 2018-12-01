use object_model::config::object::CharacterSequenceId;

use character::sequence_handler::{
    common::SequenceRepeat, CharacterSequenceHandler, SequenceHandlerUtil,
};
use CharacterSequenceUpdateComponents;

/// Determines whether to switch to the `RunStop` sequence based on X input.
///
/// This should only be called from the Walk sequence handler.
#[derive(Debug)]
pub(crate) struct RunStopCheck;

impl CharacterSequenceHandler for RunStopCheck {
    fn update<'c>(
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> Option<CharacterSequenceId> {
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
    use game_input::ControllerInput;
    use object_model::{
        config::object::CharacterSequenceId,
        entity::{
            CharacterStatus, Grounding, Mirrored, Position, RunCounter, SequenceStatus, Velocity,
        },
    };

    use super::RunStopCheck;
    use character::sequence_handler::CharacterSequenceHandler;
    use CharacterSequenceUpdateComponents;

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
                        &CharacterStatus::default(),
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
                &CharacterStatus::default(),
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
                        &CharacterStatus::default(),
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
                        &CharacterStatus::default(),
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
