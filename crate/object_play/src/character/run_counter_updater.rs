use game_input::ControllerInput;
use object_model::{
    config::object::CharacterSequenceId,
    entity::{Grounding, Mirrored, RunCounter},
};

use character::sequence_handler::SequenceHandlerUtil;

/// Updates the `RunCounter` component for character entities.
#[derive(Debug)]
pub struct RunCounterUpdater;

impl RunCounterUpdater {
    /// Returns the updated `RunCounter` value.
    ///
    /// # Parameters
    ///
    /// * `run_counter`: Original `RunCounter` value.
    /// * `controller_input`: Controller input for this character.
    /// * `character_sequence_id`: Current character sequence ID.
    /// * `mirrored`: Whether the object is mirrored (facing left).
    /// * `grounding`: Whether the object is on the ground.
    pub fn update(
        run_counter: &RunCounter,
        controller_input: &ControllerInput,
        character_sequence_id: &CharacterSequenceId,
        mirrored: &Mirrored,
        grounding: &Grounding,
    ) -> RunCounter {
        match character_sequence_id {
            CharacterSequenceId::Stand | CharacterSequenceId::Walk => {}
            _ => return RunCounter::Unused,
        }

        if *grounding != Grounding::OnGround
            || controller_input.defend
            || controller_input.jump
            || controller_input.attack
        {
            return RunCounter::Unused;
        }

        use object_model::entity::RunCounter::*;
        if controller_input.x_axis_value == 0. {
            match *run_counter {
                Unused => *run_counter,
                Exceeded | Decrease(0) => RunCounter::Unused,
                Decrease(ticks) => Decrease(ticks - 1),
                Increase(_) => Decrease(RunCounter::RESET_TICK_COUNT),
            }
        } else {
            let same_direction =
                SequenceHandlerUtil::input_matches_direction(controller_input, *mirrored);
            match (*run_counter, same_direction) {
                (Unused, _) | (Decrease(_), false) | (Increase(_), false) => {
                    Increase(RunCounter::RESET_TICK_COUNT)
                }
                (Decrease(_), true) => Unused, // Switch to running
                (Increase(0), true) => Exceeded,
                (Increase(ticks), true) => Increase(ticks - 1),
                (Exceeded, _) => *run_counter,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use game_input::ControllerInput;
    use object_model::{
        config::object::CharacterSequenceId,
        entity::{Grounding, Mirrored, RunCounter},
    };

    use super::RunCounterUpdater;

    #[test]
    fn none_when_grounding_is_airborne_and_unused() {
        let input = ControllerInput::default();

        assert_eq!(
            RunCounter::Unused,
            RunCounterUpdater::update(
                &RunCounter::Unused,
                &input,
                &CharacterSequenceId::default(),
                &Mirrored::default(),
                &Grounding::Airborne
            )
        );
    }

    #[test]
    fn unused_when_grounding_is_airborne() {
        let input = ControllerInput::default();

        assert_eq!(
            RunCounter::Unused,
            RunCounterUpdater::update(
                &RunCounter::Increase(10),
                &input,
                &CharacterSequenceId::default(),
                &Mirrored::default(),
                &Grounding::Airborne
            )
        );
    }

    #[test]
    fn none_when_jump_is_pressed_and_unused() {
        let mut input = ControllerInput::default();
        input.jump = true;

        assert_eq!(
            RunCounter::Unused,
            RunCounterUpdater::update(
                &RunCounter::Unused,
                &input,
                &CharacterSequenceId::default(),
                &Mirrored::default(),
                &Grounding::Airborne
            )
        );
    }

    macro_rules! test_action_button {
        ($test_name:ident, $action_button:ident) => {
            #[test]
            fn $test_name() {
                let mut input = ControllerInput::default();
                input.$action_button = true;

                assert_eq!(
                    RunCounter::Unused,
                    RunCounterUpdater::update(
                        &RunCounter::Increase(10),
                        &input,
                        &CharacterSequenceId::default(),
                        &Mirrored::default(),
                        &Grounding::Airborne
                    )
                );
            }
        };
    }

    test_action_button!(unused_when_defend_is_pressed, defend);
    test_action_button!(unused_when_jump_is_pressed, jump);
    test_action_button!(unused_when_attack_is_pressed, attack);

    #[test]
    fn none_when_unused_and_no_x_input() {
        let input = ControllerInput::default();

        assert_eq!(
            RunCounter::Unused,
            RunCounterUpdater::update(
                &RunCounter::Unused,
                &input,
                &CharacterSequenceId::default(),
                &Mirrored::default(),
                &Grounding::default()
            )
        );
    }

    #[test]
    fn unused_when_counter_decrease_runs_out_and_no_x_input() {
        let input = ControllerInput::default();

        assert_eq!(
            RunCounter::Unused,
            RunCounterUpdater::update(
                &RunCounter::Decrease(0),
                &input,
                &CharacterSequenceId::default(),
                &Mirrored::default(),
                &Grounding::default()
            )
        );
    }

    #[test]
    fn decrements_run_counter_when_decrease_and_no_x_input() {
        let input = ControllerInput::default();

        assert_eq!(
            RunCounter::Decrease(0),
            RunCounterUpdater::update(
                &RunCounter::Decrease(1),
                &input,
                &CharacterSequenceId::default(),
                &Mirrored::default(),
                &Grounding::default()
            )
        );
    }

    #[test]
    fn decrease_when_increase_and_no_x_input() {
        let input = ControllerInput::new(0., 1., false, false, false, false);

        assert_eq!(
            RunCounter::Decrease(RunCounter::RESET_TICK_COUNT),
            RunCounterUpdater::update(
                &RunCounter::Increase(0),
                &input,
                &CharacterSequenceId::default(),
                &Mirrored::default(),
                &Grounding::default()
            )
        );
    }

    #[test]
    fn increase_when_unused_and_input_non_zero() {
        let x_inputs = vec![1., -1.];
        let mirrors = vec![false, true];

        x_inputs
            .into_iter()
            .zip(mirrors.into_iter())
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    RunCounter::Increase(RunCounter::RESET_TICK_COUNT),
                    RunCounterUpdater::update(
                        &RunCounter::Unused,
                        &input,
                        &CharacterSequenceId::default(),
                        &mirrored.into(),
                        &Grounding::default()
                    )
                );
            });
    }

    #[test]
    fn increase_when_decrease_input_different_direction() {
        vec![(1., true), (-1., false)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    RunCounter::Increase(RunCounter::RESET_TICK_COUNT),
                    RunCounterUpdater::update(
                        &RunCounter::Decrease(11),
                        &input,
                        &CharacterSequenceId::default(),
                        &mirrored.into(),
                        &Grounding::default()
                    )
                );
            });
    }

    #[test]
    fn increase_when_increase_input_different_direction() {
        vec![(1., true), (-1., false)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    RunCounter::Increase(RunCounter::RESET_TICK_COUNT),
                    RunCounterUpdater::update(
                        &RunCounter::Increase(11),
                        &input,
                        &CharacterSequenceId::default(),
                        &mirrored.into(),
                        &Grounding::default()
                    )
                );
            });
    }

    #[test]
    fn unused_when_decrease_input_same_direction() {
        vec![(1., false), (-1., true)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    RunCounter::Unused,
                    RunCounterUpdater::update(
                        &RunCounter::Decrease(11),
                        &input,
                        &CharacterSequenceId::default(),
                        &mirrored.into(),
                        &Grounding::default()
                    )
                );
            });
    }

    #[test]
    fn exceeded_when_input_positive_same_direction_and_exceeds_tick_count() {
        vec![(1., false), (-1., true)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    RunCounter::Exceeded,
                    RunCounterUpdater::update(
                        &RunCounter::Increase(0),
                        &input,
                        &CharacterSequenceId::default(),
                        &mirrored.into(),
                        &Grounding::default()
                    )
                );
            });
    }

    #[test]
    fn decrements_increase_value_when_input_same_direction() {
        vec![(1., false), (-1., true)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    RunCounter::Increase(10),
                    RunCounterUpdater::update(
                        &RunCounter::Increase(11),
                        &input,
                        &CharacterSequenceId::default(),
                        &mirrored.into(),
                        &Grounding::default()
                    )
                );
            });
    }

    #[test]
    fn none_when_exceeded_and_input_same_direction() {
        vec![(1., false), (-1., true)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    RunCounter::Exceeded,
                    RunCounterUpdater::update(
                        &RunCounter::Exceeded,
                        &input,
                        &CharacterSequenceId::default(),
                        &mirrored.into(),
                        &Grounding::default()
                    )
                );
            });
    }
}
