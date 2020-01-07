#[cfg(test)]
mod tests {
    use std::any;

    use amethyst::{
        ecs::{Builder, SystemData, World, WorldExt},
        input::{InputBundle, StringBindings},
        shrev::{EventChannel, ReaderId},
        window::ScreenDimensions,
        Error,
    };
    use amethyst_test::{AmethystApplication, HIDPI, SCREEN_HEIGHT, SCREEN_WIDTH};
    use game_input::ControllerInput;
    use game_play_model::{play::GamePlayEndTransitionDelayClock, GamePlayEvent, GamePlayStatus};
    use tracker::Last;

    use game_play::{GamePlayEndTransitionSystem, GamePlayEndTransitionSystemData};

    #[test]
    fn does_not_send_game_play_end_stats_event_when_game_play_transition_clock_not_complete(
    ) -> Result<(), Error> {
        run_test(
            SetupParams {
                game_play_end_transition_delay_clock:
                    GamePlayEndTransitionDelayClock::new_with_value(1, 0),
                game_play_status: GamePlayStatus::Ended,
                attack_pressed_last: false,
                attack_pressed: true,
            },
            ExpectedParams {
                game_play_status: GamePlayStatus::Ended,
                game_play_events: vec![],
            },
        )
    }

    #[test]
    fn does_not_send_game_play_end_stats_event_when_game_play_is_not_end() -> Result<(), Error> {
        run_test(
            SetupParams {
                game_play_end_transition_delay_clock:
                    GamePlayEndTransitionDelayClock::new_with_value(1, 1),
                game_play_status: GamePlayStatus::Playing,
                attack_pressed_last: false,
                attack_pressed: true,
            },
            ExpectedParams {
                game_play_status: GamePlayStatus::Playing,
                game_play_events: vec![],
            },
        )
    }

    #[test]
    fn does_not_send_game_play_end_stats_event_when_attack_is_not_pressed() -> Result<(), Error> {
        run_test(
            SetupParams {
                game_play_end_transition_delay_clock:
                    GamePlayEndTransitionDelayClock::new_with_value(1, 1),
                game_play_status: GamePlayStatus::Ended,
                attack_pressed_last: true,
                attack_pressed: false,
            },
            ExpectedParams {
                game_play_status: GamePlayStatus::Ended,
                game_play_events: vec![],
            },
        )
    }

    #[test]
    fn does_not_send_game_play_end_stats_event_when_attack_was_previously_pressed_and_is_held(
    ) -> Result<(), Error> {
        run_test(
            SetupParams {
                game_play_end_transition_delay_clock:
                    GamePlayEndTransitionDelayClock::new_with_value(1, 1),
                game_play_status: GamePlayStatus::Ended,
                attack_pressed_last: true,
                attack_pressed: true,
            },
            ExpectedParams {
                game_play_status: GamePlayStatus::Ended,
                game_play_events: vec![],
            },
        )
    }

    #[test]
    fn sends_game_play_end_stats_event_when_attack_was_not_previously_pressed_and_is_now(
    ) -> Result<(), Error> {
        run_test(
            SetupParams {
                game_play_end_transition_delay_clock:
                    GamePlayEndTransitionDelayClock::new_with_value(1, 1),
                game_play_status: GamePlayStatus::Ended,
                attack_pressed_last: false,
                attack_pressed: true,
            },
            ExpectedParams {
                game_play_status: GamePlayStatus::None,
                game_play_events: vec![GamePlayEvent::EndStats],
            },
        )
    }

    fn run_test(
        SetupParams {
            game_play_end_transition_delay_clock,
            game_play_status: game_play_status_setup,
            attack_pressed_last,
            attack_pressed,
        }: SetupParams,
        ExpectedParams {
            game_play_status: game_play_status_expected,
            game_play_events,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(InputBundle::<StringBindings>::new())
            .with_resource(game_play_end_transition_delay_clock)
            .with_resource(ScreenDimensions::new(SCREEN_WIDTH, SCREEN_HEIGHT, HIDPI))
            .with_resource(game_play_status_setup)
            .with_setup(GamePlayEndTransitionSystemData::setup)
            .with_setup(register_event_reader)
            .with_effect(move |world| {
                setup_controller_input(world, attack_pressed_last, attack_pressed)
            })
            .with_system_single(
                GamePlayEndTransitionSystem::new(),
                any::type_name::<GamePlayEndTransitionSystem>(),
                &[],
            ) // kcov-ignore
            .with_assertion(move |world| {
                let game_play_status = *world.read_resource::<GamePlayStatus>();

                assert_eq!(game_play_status_expected, game_play_status);
                assert_game_play_events(world, game_play_events);
            })
            .run()
    }

    fn register_event_reader(world: &mut World) {
        let reader_id = {
            let mut game_play_ec = world.write_resource::<EventChannel<GamePlayEvent>>();
            game_play_ec.register_reader()
        }; // kcov-ignore
        world.insert(reader_id);
    }

    fn setup_controller_input(world: &mut World, last_attack_pressed: bool, attack_pressed: bool) {
        let mut last_controller_input = ControllerInput::default();
        last_controller_input.attack = last_attack_pressed;
        let last_controller_input = Last(last_controller_input);

        let mut controller_input = ControllerInput::default();
        controller_input.attack = attack_pressed;

        world
            .create_entity()
            .with(last_controller_input)
            .with(controller_input)
            .build();
    }

    fn assert_game_play_events(world: &mut World, game_play_events_expected: Vec<GamePlayEvent>) {
        let mut reader_id = &mut world.write_resource::<ReaderId<GamePlayEvent>>();
        let game_play_ec = world.read_resource::<EventChannel<GamePlayEvent>>();

        let game_play_events_actual = game_play_ec
            .read(&mut reader_id)
            .copied()
            .collect::<Vec<GamePlayEvent>>();

        assert_eq!(game_play_events_expected, game_play_events_actual);
    }

    struct SetupParams {
        game_play_end_transition_delay_clock: GamePlayEndTransitionDelayClock,
        game_play_status: GamePlayStatus,
        attack_pressed_last: bool,
        attack_pressed: bool,
    }

    struct ExpectedParams {
        game_play_status: GamePlayStatus,
        game_play_events: Vec<GamePlayEvent>,
    }
}
