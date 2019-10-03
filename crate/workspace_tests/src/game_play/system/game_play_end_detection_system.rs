#[cfg(test)]
mod test {
    use amethyst::{
        ecs::{Builder, World, WorldExt},
        input::StringBindings,
        shred::SystemData,
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use game_play_model::{GamePlayEvent, GamePlayStatus};
    use object_model::play::HealthPoints;
    use team_model::play::{IndependentCounter, Team};
    use typename::TypeName;

    use game_play::{GamePlayEndDetectionSystem, GamePlayEndDetectionSystemData};

    #[test]
    fn does_not_send_game_play_end_event_when_game_play_is_not_playing() -> Result<(), Error> {
        let setup = |world: &mut World| {
            world
                .create_entity()
                .with(Team::Independent(IndependentCounter::new(0)))
                .with(HealthPoints(100))
                .build();

            // Non-live character.
            world
                .create_entity()
                .with(Team::Independent(IndependentCounter::new(1)))
                .with(HealthPoints(0))
                .build();
        };

        AmethystApplication::ui_base::<StringBindings>()
            .with_resource(GamePlayStatus::Ended)
            .with_effect(register_gpec_reader)
            .with_effect(setup)
            .with_system_single(
                GamePlayEndDetectionSystem::new(),
                GamePlayEndDetectionSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_assertion(|world| verify_game_play_events(world, &[]))
            .run()
    }

    #[test]
    fn sends_game_play_end_event_when_one_alive_team_remaining() -> Result<(), Error> {
        let setup = |world: &mut World| {
            world
                .create_entity()
                .with(Team::Independent(IndependentCounter::new(0)))
                .with(HealthPoints(100))
                .build();

            // Non-live character.
            world
                .create_entity()
                .with(Team::Independent(IndependentCounter::new(1)))
                .with(HealthPoints(0))
                .build();
        };

        AmethystApplication::ui_base::<StringBindings>()
            .with_resource(GamePlayStatus::Playing)
            .with_effect(register_gpec_reader)
            .with_effect(setup)
            .with_system_single(
                GamePlayEndDetectionSystem::new(),
                GamePlayEndDetectionSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_assertion(|world| verify_game_play_events(world, &[&GamePlayEvent::End]))
            .run()
    }

    #[test]
    fn sends_game_play_end_event_when_one_alive_team_multiple_entities_remaining(
    ) -> Result<(), Error> {
        let setup = |world: &mut World| {
            world
                .create_entity()
                .with(Team::Independent(IndependentCounter::new(0)))
                .with(HealthPoints(100))
                .build();
            world
                .create_entity()
                .with(Team::Independent(IndependentCounter::new(0)))
                .with(HealthPoints(100))
                .build();

            // Non-live character.
            world
                .create_entity()
                .with(Team::Independent(IndependentCounter::new(1)))
                .with(HealthPoints(0))
                .build();
        };

        AmethystApplication::ui_base::<StringBindings>()
            .with_resource(GamePlayStatus::Playing)
            .with_effect(register_gpec_reader)
            .with_effect(setup)
            .with_system_single(
                GamePlayEndDetectionSystem::new(),
                GamePlayEndDetectionSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_assertion(|world| verify_game_play_events(world, &[&GamePlayEvent::End]))
            .run()
    }

    #[test]
    fn sends_game_play_end_event_when_no_alive_characters_remaining() -> Result<(), Error> {
        let setup = |world: &mut World| {
            world
                .create_entity()
                .with(Team::Independent(IndependentCounter::new(0)))
                .with(HealthPoints(0))
                .build();

            // Non-live character.
            world
                .create_entity()
                .with(Team::Independent(IndependentCounter::new(1)))
                .with(HealthPoints(0))
                .build();
        };

        AmethystApplication::ui_base::<StringBindings>()
            .with_resource(GamePlayStatus::Playing)
            .with_effect(register_gpec_reader)
            .with_effect(setup)
            .with_system_single(
                GamePlayEndDetectionSystem::new(),
                GamePlayEndDetectionSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_assertion(|world| verify_game_play_events(world, &[&GamePlayEvent::End]))
            .run()
    }

    #[test]
    fn does_not_send_game_play_end_event_when_two_alive_characters_remaining() -> Result<(), Error>
    {
        let setup = |world: &mut World| {
            world
                .create_entity()
                .with(Team::Independent(IndependentCounter::new(0)))
                .with(HealthPoints(100))
                .build();

            // Non-live character.
            world
                .create_entity()
                .with(Team::Independent(IndependentCounter::new(1)))
                .with(HealthPoints(100))
                .build();
        };

        AmethystApplication::ui_base::<StringBindings>()
            .with_resource(GamePlayStatus::Playing)
            .with_effect(register_gpec_reader)
            .with_effect(setup)
            .with_system_single(
                GamePlayEndDetectionSystem::new(),
                GamePlayEndDetectionSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_assertion(|world| verify_game_play_events(world, &[]))
            .run()
    }

    fn register_gpec_reader(world: &mut World) {
        GamePlayEndDetectionSystemData::setup(world);

        let reader_id = {
            let mut game_play_ec = world.write_resource::<EventChannel<GamePlayEvent>>();
            game_play_ec.register_reader()
        }; // kcov-ignore
        world.insert(reader_id);
    }

    fn verify_game_play_events(world: &mut World, expected: &[&GamePlayEvent]) {
        let mut reader_id = &mut world.write_resource::<ReaderId<GamePlayEvent>>();
        let game_play_ec = world.read_resource::<EventChannel<GamePlayEvent>>();

        let actual = game_play_ec
            .read(&mut reader_id)
            .collect::<Vec<&GamePlayEvent>>();

        assert_eq!(expected, &*actual);
    }
}
