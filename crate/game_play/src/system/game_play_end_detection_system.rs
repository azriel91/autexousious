use std::collections::HashMap;

use amethyst::{
    ecs::{Join, ReadExpect, ReadStorage, System, World, Write},
    shred::{ResourceId, SystemData},
    shrev::EventChannel,
};
use derivative::Derivative;
use derive_new::new;
use game_play_model::{GamePlayEvent, GamePlayStatus};
use object_model::play::HealthPoints;
use team_model::play::Team;
use typename_derive::TypeName;

/// Detects the end of a game play round, and fires a `GamePlayEvent::End`.
///
/// In the future this will be type parameterized to specify the detection function.
#[derive(Debug, Default, TypeName, new)]
pub struct GamePlayEndDetectionSystem {
    /// Pre-allocated `HashMap` to track number of alive players.
    #[new(default)]
    team_alive_counter: HashMap<Team, u32>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct GamePlayEndDetectionSystemData<'s> {
    /// `GamePlayStatus` resource.
    #[derivative(Debug = "ignore")]
    pub game_play_status: ReadExpect<'s, GamePlayStatus>,
    /// `GamePlayEvent` channel.
    #[derivative(Debug = "ignore")]
    pub game_play_ec: Write<'s, EventChannel<GamePlayEvent>>,
    /// `Team` components.
    #[derivative(Debug = "ignore")]
    pub teams: ReadStorage<'s, Team>,
    /// `HealthPoints` components.
    #[derivative(Debug = "ignore")]
    pub health_pointses: ReadStorage<'s, HealthPoints>,
}

impl<'s> System<'s> for GamePlayEndDetectionSystem {
    type SystemData = GamePlayEndDetectionSystemData<'s>;

    fn run(
        &mut self,
        GamePlayEndDetectionSystemData {
            game_play_status,
            mut game_play_ec,
            teams,
            health_pointses,
        }: Self::SystemData,
    ) {
        if *game_play_status == GamePlayStatus::Playing {
            self.team_alive_counter.clear();

            // Game ends when there is one or less teams remaining
            (&teams, &health_pointses)
                .join()
                .for_each(|(team, health_points)| {
                    if *health_points > 0 {
                        let alive_count = self.team_alive_counter.entry(*team).or_insert(0);
                        *alive_count += 1;
                    };
                });

            if self.team_alive_counter.len() <= 1 {
                game_play_ec.single_write(GamePlayEvent::End);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use amethyst::{
        ecs::{Builder, World},
        input::StringBindings,
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use game_play_model::{GamePlayEvent, GamePlayStatus};
    use object_model::play::HealthPoints;
    use team_model::play::{IndependentCounter, Team};
    use typename::TypeName;

    use super::{GamePlayEndDetectionSystem, GamePlayEndDetectionSystemData};

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
            .with_setup(register_gpec_reader)
            .with_setup(setup)
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
            .with_setup(register_gpec_reader)
            .with_setup(setup)
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
            .with_setup(register_gpec_reader)
            .with_setup(setup)
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
            .with_setup(register_gpec_reader)
            .with_setup(setup)
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
            .with_setup(register_gpec_reader)
            .with_setup(setup)
            .with_system_single(
                GamePlayEndDetectionSystem::new(),
                GamePlayEndDetectionSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_assertion(|world| verify_game_play_events(world, &[]))
            .run()
    }

    fn register_gpec_reader(world: &mut World) {
        GamePlayEndDetectionSystemData::setup(&mut world.res);

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
