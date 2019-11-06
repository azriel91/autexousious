#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{World, WorldExt},
        shrev::EventChannel,
        Error,
    };
    use amethyst_test::AmethystApplication;
    use game_play_model::{play::GamePlayEndTransitionDelayClock, GamePlayEvent};

    use game_play::{GamePlayEndTransitionDelaySystem, GAME_PLAY_END_TRANSITION_DELAY_DEFAULT};

    #[test]
    fn ticks_clock_when_no_event_sent() -> Result<(), Error> {
        run_test(
            SetupParams {
                events: vec![],
                game_play_end_transition_delay_clock:
                    GamePlayEndTransitionDelayClock::new_with_value(
                        GAME_PLAY_END_TRANSITION_DELAY_DEFAULT,
                        1,
                    ),
            },
            ExpectedParams {
                game_play_end_transition_delay_clock:
                    GamePlayEndTransitionDelayClock::new_with_value(
                        GAME_PLAY_END_TRANSITION_DELAY_DEFAULT,
                        3,
                    ),
            },
        )
    }

    #[test]
    fn restarts_clock_when_ended_event_sent() -> Result<(), Error> {
        run_test(
            SetupParams {
                events: vec![GamePlayEvent::End],
                game_play_end_transition_delay_clock:
                    GamePlayEndTransitionDelayClock::new_with_value(
                        GAME_PLAY_END_TRANSITION_DELAY_DEFAULT,
                        1,
                    ),
            },
            ExpectedParams {
                game_play_end_transition_delay_clock:
                    GamePlayEndTransitionDelayClock::new_with_value(
                        GAME_PLAY_END_TRANSITION_DELAY_DEFAULT,
                        0,
                    ),
            },
        )
    }

    #[test]
    fn ticks_clock_when_other_event_sent() -> Result<(), Error> {
        run_test(
            SetupParams {
                events: vec![GamePlayEvent::Resume],
                game_play_end_transition_delay_clock:
                    GamePlayEndTransitionDelayClock::new_with_value(
                        GAME_PLAY_END_TRANSITION_DELAY_DEFAULT,
                        1,
                    ),
            },
            ExpectedParams {
                game_play_end_transition_delay_clock:
                    GamePlayEndTransitionDelayClock::new_with_value(
                        GAME_PLAY_END_TRANSITION_DELAY_DEFAULT,
                        3,
                    ),
            },
        )
    }

    fn run_test(
        SetupParams {
            events,
            game_play_end_transition_delay_clock: game_play_end_transition_delay_clock_setup,
        }: SetupParams,
        ExpectedParams {
            game_play_end_transition_delay_clock: game_play_end_transition_delay_clock_expected,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_resource(game_play_end_transition_delay_clock_setup)
            .with_system(GamePlayEndTransitionDelaySystem::new(), "", &[])
            .with_effect(move |world| {
                send_events(world, events);
            })
            .with_assertion(move |world| {
                let game_play_end_transition_delay_clock_actual =
                    *world.read_resource::<GamePlayEndTransitionDelayClock>();

                assert_eq!(
                    game_play_end_transition_delay_clock_expected,
                    game_play_end_transition_delay_clock_actual
                );
            })
            .run()
    }

    fn send_events(world: &mut World, mut events: Vec<GamePlayEvent>) {
        let mut state_id_update_ec = world.write_resource::<EventChannel<GamePlayEvent>>();
        state_id_update_ec.iter_write(events.drain(..));
    }

    struct SetupParams {
        events: Vec<GamePlayEvent>,
        game_play_end_transition_delay_clock: GamePlayEndTransitionDelayClock,
    }

    struct ExpectedParams {
        game_play_end_transition_delay_clock: GamePlayEndTransitionDelayClock,
    }
}
