#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, World, WorldExt},
        shred::SystemData,
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use game_play_model::{GamePlayEvent, GamePlayStatus};
    use game_stats_model::play::{WinOutcome, WinStatus};
    use object_model::play::HealthPoints;
    use std::any;
    use team_model::play::{IndependentCounter, Team, TeamCounter};

    use game_play::{GamePlayEndDetectionSystem, GamePlayEndDetectionSystemData};

    #[test]
    fn does_not_send_game_play_end_event_when_game_play_is_not_playing() -> Result<(), Error> {
        run_test(
            SetupParams {
                game_play_status: GamePlayStatus::Ended,
                objects: vec![
                    ObjectStatus {
                        team: Team::Number(TeamCounter::new(0)),
                        liveness: Liveness::Alive,
                    },
                    ObjectStatus {
                        team: Team::Independent(IndependentCounter::new(1)),
                        liveness: Liveness::Alive,
                    },
                ],
            },
            ExpectedParams {
                game_play_status: GamePlayStatus::Playing,
                game_play_events: vec![],
                win_status: WinStatus::default(),
            },
        )
    }

    #[test]
    fn sends_game_play_end_event_when_one_alive_team_remaining() -> Result<(), Error> {
        let winning_team = Team::Number(TeamCounter::new(0));
        run_test(
            SetupParams {
                game_play_status: GamePlayStatus::Playing,
                objects: vec![
                    ObjectStatus {
                        team: winning_team,
                        liveness: Liveness::Alive,
                    },
                    ObjectStatus {
                        team: Team::Independent(IndependentCounter::new(1)),
                        liveness: Liveness::Dead,
                    },
                ],
            },
            ExpectedParams {
                game_play_status: GamePlayStatus::Ended,
                game_play_events: vec![GamePlayEvent::End],
                win_status: WinStatus::new(WinOutcome::WinLoss { winning_team }),
            },
        )
    }

    #[test]
    fn sends_game_play_end_event_when_one_alive_team_multiple_entities_remaining()
    -> Result<(), Error> {
        let winning_team = Team::Number(TeamCounter::new(0));
        run_test(
            SetupParams {
                game_play_status: GamePlayStatus::Playing,
                objects: vec![
                    ObjectStatus {
                        team: winning_team,
                        liveness: Liveness::Alive,
                    },
                    ObjectStatus {
                        team: winning_team,
                        liveness: Liveness::Alive,
                    },
                    ObjectStatus {
                        team: Team::Independent(IndependentCounter::new(1)),
                        liveness: Liveness::Dead,
                    },
                ],
            },
            ExpectedParams {
                game_play_status: GamePlayStatus::Ended,
                game_play_events: vec![GamePlayEvent::End],
                win_status: WinStatus::new(WinOutcome::WinLoss { winning_team }),
            },
        )
    }

    #[test]
    fn sends_game_play_end_event_when_no_alive_characters_remaining() -> Result<(), Error> {
        run_test(
            SetupParams {
                game_play_status: GamePlayStatus::Playing,
                objects: vec![
                    ObjectStatus {
                        team: Team::Independent(IndependentCounter::new(0)),
                        liveness: Liveness::Dead,
                    },
                    ObjectStatus {
                        team: Team::Independent(IndependentCounter::new(1)),
                        liveness: Liveness::Dead,
                    },
                ],
            },
            ExpectedParams {
                game_play_status: GamePlayStatus::Ended,
                game_play_events: vec![GamePlayEvent::End],
                win_status: WinStatus::new(WinOutcome::Draw),
            },
        )
    }

    #[test]
    fn does_not_send_game_play_end_event_when_two_alive_characters_remaining() -> Result<(), Error>
    {
        run_test(
            SetupParams {
                game_play_status: GamePlayStatus::Playing,
                objects: vec![
                    ObjectStatus {
                        team: Team::Independent(IndependentCounter::new(0)),
                        liveness: Liveness::Alive,
                    },
                    ObjectStatus {
                        team: Team::Independent(IndependentCounter::new(1)),
                        liveness: Liveness::Alive,
                    },
                ],
            },
            ExpectedParams {
                game_play_status: GamePlayStatus::Playing,
                game_play_events: vec![],
                win_status: WinStatus::default(),
            },
        )
    }

    fn run_test(
        SetupParams {
            game_play_status: game_play_status_setup,
            objects,
        }: SetupParams,
        ExpectedParams {
            game_play_status: game_play_status_expected,
            game_play_events,
            win_status: win_status_expected,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_resource(game_play_status_setup)
            .with_setup(GamePlayEndDetectionSystemData::setup)
            .with_setup(register_event_reader)
            .with_effect(move |world| {
                objects.into_iter().for_each(|object_status| {
                    let ObjectStatus { liveness, team } = object_status;

                    let health_points = match liveness {
                        Liveness::Alive => HealthPoints(100),
                        Liveness::Dead => HealthPoints(0),
                    };

                    world.create_entity().with(team).with(health_points).build();
                });
            })
            .with_system_single(
                GamePlayEndDetectionSystem::new(),
                any::type_name::<GamePlayEndDetectionSystem>(),
                &[],
            ) // kcov-ignore
            .with_assertion(move |world| {
                let game_play_status = *world.read_resource::<GamePlayStatus>();
                let win_status = *world.read_resource::<WinStatus>();

                assert_eq!(game_play_status_expected, game_play_status);
                assert_eq!(win_status_expected, win_status);
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
        game_play_status: GamePlayStatus,
        objects: Vec<ObjectStatus>,
    }

    struct ExpectedParams {
        game_play_status: GamePlayStatus,
        game_play_events: Vec<GamePlayEvent>,
        win_status: WinStatus,
    }

    struct ObjectStatus {
        liveness: Liveness,
        team: Team,
    }

    enum Liveness {
        Alive,
        Dead,
    }
}
