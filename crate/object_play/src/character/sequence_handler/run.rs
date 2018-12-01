use object_model::config::object::CharacterSequenceId;

use character::sequence_handler::{
    common::{grounding::AirborneCheck, input::RunStopCheck, status::AliveCheck},
    CharacterSequenceHandler,
};
use CharacterSequenceUpdateComponents;

/// Hold forward to run, release to stop running.
#[derive(Debug)]
pub(crate) struct Run;

impl CharacterSequenceHandler for Run {
    fn update<'c>(
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> Option<CharacterSequenceId> {
        [
            AliveCheck::update,
            AirborneCheck::update,
            RunStopCheck::update,
        ]
        .iter()
        .fold(None, |status_update, fn_update| {
            status_update.or_else(|| fn_update(components))
        })
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

    use super::Run;
    use character::sequence_handler::CharacterSequenceHandler;
    use CharacterSequenceUpdateComponents;

    #[test]
    fn jump_descend_when_airborne() {
        assert_eq!(
            Some(CharacterSequenceId::JumpDescend),
            Run::update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                &CharacterStatus::default(),
                &CharacterSequenceId::Run,
                &SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                &Mirrored::default(),
                &Grounding::Airborne,
                &RunCounter::default()
            ))
        );
    }

    #[test]
    fn reverts_to_run_stop_when_no_input() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceId::RunStop),
            Run::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &CharacterSequenceId::Run,
                &SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                &Mirrored(false),
                &Grounding::default(),
                &RunCounter::default()
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
                &CharacterStatus::default(),
                &CharacterSequenceId::Run,
                &SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                &Mirrored(false),
                &Grounding::default(),
                &RunCounter::default()
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
                &CharacterStatus::default(),
                &CharacterSequenceId::Run,
                &SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                &Mirrored(true),
                &Grounding::default(),
                &RunCounter::default()
            ))
        );
    }

    #[test]
    fn restarts_run_when_sequence_ended() {
        vec![(1., false), (-1., true)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    Some(CharacterSequenceId::Run),
                    Run::update(CharacterSequenceUpdateComponents::new(
                        &input,
                        &CharacterStatus::default(),
                        &CharacterSequenceId::Run,
                        &SequenceStatus::End,
                        &Position::default(),
                        &Velocity::default(),
                        &mirrored.into(),
                        &Grounding::default(),
                        &RunCounter::default()
                    ))
                );
            });
    }

    #[test]
    fn reverts_to_run_stop_when_x_axis_negative_and_non_mirrored() {
        let input = ControllerInput::new(-1., 0., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceId::RunStop),
            Run::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &CharacterSequenceId::Run,
                &SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                &Mirrored(false),
                &Grounding::default(),
                &RunCounter::default()
            ))
        );
    }

    #[test]
    fn reverts_to_run_stop_when_x_axis_positive_and_mirrored() {
        let input = ControllerInput::new(1., 0., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceId::RunStop),
            Run::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &CharacterSequenceId::Run,
                &SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                &Mirrored(true),
                &Grounding::default(),
                &RunCounter::default()
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
                &CharacterStatus::default(),
                &CharacterSequenceId::Run,
                &SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                &Mirrored(false),
                &Grounding::default(),
                &RunCounter::default()
            ))
        );

        let input = ControllerInput::new(1., -1., false, false, false, false);

        assert_eq!(
            None,
            Run::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &CharacterSequenceId::Run,
                &SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                &Mirrored(false),
                &Grounding::default(),
                &RunCounter::default()
            ))
        );
    }
}
