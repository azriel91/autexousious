use object_model::{
    config::object::CharacterSequenceId,
    entity::{
        CharacterInput, CharacterStatus, CharacterStatusUpdate, ObjectStatusUpdate, RunCounter,
    },
};

use character::sequence_handler::SequenceHandler;

#[derive(Debug)]
pub(crate) struct Walk;

impl SequenceHandler for Walk {
    fn update(input: &CharacterInput, character_status: &CharacterStatus) -> CharacterStatusUpdate {
        let (run_counter, mut sequence_id, mirrored) = {
            let mirrored = character_status.object_status.mirrored;

            use object_model::entity::RunCounter::*;

            // TODO: Refactor
            if input.x_axis_value == 0. {
                let run_counter = match character_status.run_counter {
                    Unused => None,
                    Exceeded | Decrease(0) => Some(Unused),
                    Decrease(ticks) => Some(Decrease(ticks - 1)),
                    Increase(_) => Some(Decrease(RunCounter::RESET_TICK_COUNT)),
                };
                (run_counter, Some(CharacterSequenceId::Stand), None)
            } else if input.x_axis_value > 0. {
                match (character_status.run_counter, mirrored) {
                    (Unused, _) | (Decrease(_), true) | (Increase(_), true) => (
                        Some(Increase(RunCounter::RESET_TICK_COUNT)),
                        Some(CharacterSequenceId::Walk),
                        Some(false),
                    ),
                    (Decrease(_), false) => (Some(Unused), Some(CharacterSequenceId::Run), None),
                    (Increase(0), false) => (Some(Exceeded), None, None),
                    (Increase(ticks), false) => (Some(Increase(ticks - 1)), None, None),
                    (Exceeded, _) => (None, None, None),
                }
            } else {
                // input.x_axis_value < 0.
                match (character_status.run_counter, mirrored) {
                    (Unused, _) | (Decrease(_), false) | (Increase(_), false) => (
                        Some(Increase(RunCounter::RESET_TICK_COUNT)),
                        Some(CharacterSequenceId::Walk),
                        Some(true),
                    ),
                    (Decrease(_), true) => (Some(Unused), Some(CharacterSequenceId::Run), None),
                    (Increase(0), true) => (Some(Exceeded), None, None),
                    (Increase(ticks), true) => (Some(Increase(ticks - 1)), None, None),
                    (Exceeded, _) => (None, None, None),
                }
            }
        };

        // If we are about to stand, but have z axis input, then we walk instead
        if sequence_id == Some(CharacterSequenceId::Stand) && input.z_axis_value != 0. {
            sequence_id = None;
        }

        CharacterStatusUpdate::new(run_counter, ObjectStatusUpdate::new(sequence_id, mirrored))
    }
}

#[cfg(test)]
mod test {
    use object_model::{
        config::object::CharacterSequenceId,
        entity::{
            CharacterInput, CharacterStatus, CharacterStatusUpdate, ObjectStatus,
            ObjectStatusUpdate, RunCounter,
        },
    };

    use super::Walk;
    use character::sequence_handler::SequenceHandler;

    #[test]
    fn reverts_to_stand_when_no_input() {
        let input = CharacterInput::new(0., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(
                Some(RunCounter::Decrease(RunCounter::RESET_TICK_COUNT)),
                ObjectStatusUpdate::new(Some(CharacterSequenceId::Stand), None)
            ),
            Walk::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Increase(10),
                    ObjectStatus::new(CharacterSequenceId::Walk, false)
                )
            )
        );
    }

    #[test]
    fn reverts_to_stand_with_run_counter_unused_when_no_input_and_run_counter_exceeded() {
        let input = CharacterInput::new(0., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(
                Some(RunCounter::Unused),
                ObjectStatusUpdate::new(Some(CharacterSequenceId::Stand), None)
            ),
            Walk::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Exceeded,
                    ObjectStatus::new(CharacterSequenceId::Walk, false)
                )
            )
        );
    }

    #[test]
    fn decrements_run_counter_when_x_axis_positive_non_mirror() {
        let input = CharacterInput::new(1., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(
                Some(RunCounter::Increase(10)),
                ObjectStatusUpdate::new(None, None)
            ),
            Walk::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Increase(11),
                    ObjectStatus::new(CharacterSequenceId::Walk, false)
                )
            )
        );
    }

    #[test]
    fn run_counter_exceeded_when_x_axis_positive_non_mirror_and_exceeds_tick_count() {
        let input = CharacterInput::new(1., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(
                Some(RunCounter::Exceeded),
                ObjectStatusUpdate::new(None, None)
            ),
            Walk::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Increase(0),
                    ObjectStatus::new(CharacterSequenceId::Walk, false)
                )
            )
        );
    }

    #[test]
    fn decrements_run_counter_when_x_axis_negative_mirror() {
        let input = CharacterInput::new(-1., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(
                Some(RunCounter::Increase(10)),
                ObjectStatusUpdate::new(None, None)
            ),
            Walk::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Increase(11),
                    ObjectStatus::new(CharacterSequenceId::Walk, true)
                )
            )
        );
    }

    #[test]
    fn run_counter_exceeded_when_x_axis_negative_mirror_and_exceeds_tick_count() {
        let input = CharacterInput::new(-1., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(
                Some(RunCounter::Exceeded),
                ObjectStatusUpdate::new(None, None)
            ),
            Walk::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Increase(0),
                    ObjectStatus::new(CharacterSequenceId::Walk, true)
                )
            )
        );
    }

    #[test]
    fn run_counter_decrease_when_x_axis_zero_z_axis_positive_and_run_counter_increase() {
        let input = CharacterInput::new(0., 1., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(
                Some(RunCounter::Decrease(RunCounter::RESET_TICK_COUNT)),
                ObjectStatusUpdate::new(None, None)
            ),
            Walk::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Increase(0),
                    ObjectStatus::new(CharacterSequenceId::Walk, false)
                )
            )
        );
    }

    #[test]
    fn walk_non_mirror_when_x_axis_positive_mirror() {
        let input = CharacterInput::new(1., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(
                Some(RunCounter::Increase(RunCounter::RESET_TICK_COUNT)),
                ObjectStatusUpdate::new(Some(CharacterSequenceId::Walk), Some(false))
            ),
            Walk::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Increase(11),
                    ObjectStatus::new(CharacterSequenceId::Walk, true)
                )
            )
        );
    }

    #[test]
    fn walk_mirror_when_x_axis_negative_non_mirror() {
        let input = CharacterInput::new(-1., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(
                Some(RunCounter::Increase(RunCounter::RESET_TICK_COUNT)),
                ObjectStatusUpdate::new(Some(CharacterSequenceId::Walk), Some(true))
            ),
            Walk::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Increase(11),
                    ObjectStatus::new(CharacterSequenceId::Walk, false)
                )
            )
        );
    }

    #[test]
    fn walk_when_z_axis_non_zero() {
        let input = CharacterInput::new(0., 1., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(
                Some(RunCounter::Decrease(RunCounter::RESET_TICK_COUNT)),
                ObjectStatusUpdate::new(None, None)
            ),
            Walk::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Increase(0),
                    ObjectStatus::new(CharacterSequenceId::Walk, false)
                )
            )
        );

        let input = CharacterInput::new(0., -1., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(
                Some(RunCounter::Decrease(RunCounter::RESET_TICK_COUNT)),
                ObjectStatusUpdate::new(None, None)
            ),
            Walk::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Increase(0),
                    ObjectStatus::new(CharacterSequenceId::Walk, false)
                )
            )
        );
    }

    #[test]
    fn run_when_x_axis_positive_and_run_counter_decrease_non_mirror() {
        let input = CharacterInput::new(1., -1., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(
                Some(RunCounter::Unused),
                ObjectStatusUpdate::new(Some(CharacterSequenceId::Run), None)
            ),
            Walk::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Decrease(10),
                    ObjectStatus::new(CharacterSequenceId::Walk, false)
                )
            )
        );
    }

    #[test]
    fn run_when_x_axis_negative_and_run_counter_decrease_mirror() {
        let input = CharacterInput::new(-1., -1., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(
                Some(RunCounter::Unused),
                ObjectStatusUpdate::new(Some(CharacterSequenceId::Run), None)
            ),
            Walk::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Decrease(10),
                    ObjectStatus::new(CharacterSequenceId::Walk, true)
                )
            )
        );
    }
}
