use amethyst::{ecs::prelude::*, shrev::EventChannel};
use derive_new::new;
use game_play_model::{GamePlayEvent, GamePlayStatus};
use object_model::entity::HealthPoints;
use typename_derive::TypeName;

/// Detects the end of a game play round, and fires a `GamePlayEvent::End`.
///
/// In the future this will be type parameterized to specify the detection function.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct GamePlayEndDetectionSystem;

type GamePlayEndDetectionSystemData<'s> = (
    ReadExpect<'s, GamePlayStatus>,
    Write<'s, EventChannel<GamePlayEvent>>,
    ReadStorage<'s, HealthPoints>,
);

impl<'s> System<'s> for GamePlayEndDetectionSystem {
    type SystemData = GamePlayEndDetectionSystemData<'s>;

    fn run(&mut self, (game_play_status, mut game_play_ec, health_pointses): Self::SystemData) {
        if *game_play_status == GamePlayStatus::Playing {
            // Game ends when there is one or less people standing
            let alive_count = health_pointses
                .join()
                .fold(0, |mut alive_count, health_points| {
                    if *health_points > 0 {
                        alive_count += 1
                    };
                    alive_count
                });

            if alive_count <= 1 {
                game_play_ec.single_write(GamePlayEvent::End);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use amethyst::{ecs::prelude::*, shrev::EventChannel};
    use amethyst_test::*;
    use game_play_model::{GamePlayEvent, GamePlayStatus};
    use object_model::entity::HealthPoints;
    use typename::TypeName;

    use super::{GamePlayEndDetectionSystem, GamePlayEndDetectionSystemData};

    #[test]
    fn does_not_send_game_play_end_event_when_game_play_is_not_playing() {
        let setup = |world: &mut World| {
            world.create_entity().with(HealthPoints(100)).build();

            // Non-live character.
            world.create_entity().with(HealthPoints(0)).build();
        };

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<String, String>()
                .with_resource(GamePlayStatus::Ended)
                .with_setup(register_gpec_reader)
                .with_setup(setup)
                .with_system_single(
                    GamePlayEndDetectionSystem::new(),
                    GamePlayEndDetectionSystem::type_name(),
                    &[]
                ) // kcov-ignore
                .with_assertion(|world| verify_game_play_events(world, &[]))
                .run()
                .is_ok()
        );
    }

    #[test]
    fn sends_game_play_end_event_when_one_alive_character_remaining() {
        let setup = |world: &mut World| {
            world.create_entity().with(HealthPoints(100)).build();

            // Non-live character.
            world.create_entity().with(HealthPoints(0)).build();
        };

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<String, String>()
                .with_resource(GamePlayStatus::Playing)
                .with_setup(register_gpec_reader)
                .with_setup(setup)
                .with_system_single(
                    GamePlayEndDetectionSystem::new(),
                    GamePlayEndDetectionSystem::type_name(),
                    &[]
                ) // kcov-ignore
                .with_assertion(|world| verify_game_play_events(world, &[&GamePlayEvent::End]))
                .run()
                .is_ok()
        );
    }

    #[test]
    fn sends_game_play_end_event_when_no_alive_characters_remaining() {
        let setup = |world: &mut World| {
            world.create_entity().with(HealthPoints(0)).build();

            // Non-live character.
            world.create_entity().with(HealthPoints(0)).build();
        };

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<String, String>()
                .with_resource(GamePlayStatus::Playing)
                .with_setup(register_gpec_reader)
                .with_setup(setup)
                .with_system_single(
                    GamePlayEndDetectionSystem::new(),
                    GamePlayEndDetectionSystem::type_name(),
                    &[]
                ) // kcov-ignore
                .with_assertion(|world| verify_game_play_events(world, &[&GamePlayEvent::End]))
                .run()
                .is_ok()
        );
    }

    #[test]
    fn does_not_send_game_play_end_event_when_two_alive_characters_remaining() {
        let setup = |world: &mut World| {
            world.create_entity().with(HealthPoints(100)).build();

            // Non-live character.
            world.create_entity().with(HealthPoints(100)).build();
        };

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<String, String>()
                .with_resource(GamePlayStatus::Playing)
                .with_setup(register_gpec_reader)
                .with_setup(setup)
                .with_system_single(
                    GamePlayEndDetectionSystem::new(),
                    GamePlayEndDetectionSystem::type_name(),
                    &[]
                ) // kcov-ignore
                .with_assertion(|world| verify_game_play_events(world, &[]))
                .run()
                .is_ok()
        );
    }

    fn register_gpec_reader(world: &mut World) {
        GamePlayEndDetectionSystemData::setup(&mut world.res);

        let reader_id = {
            let mut game_play_ec = world.write_resource::<EventChannel<GamePlayEvent>>();
            game_play_ec.register_reader()
        };
        world.add_resource(reader_id);
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
