use amethyst::{ecs::prelude::*, shrev::EventChannel};
use game_play_model::GamePlayEvent;
use object_model::entity::CharacterStatus;

/// Detects the end of a game play round, and fires a `GamePlayEvent::End`.
///
/// In the future this will be type parameterized to specify the detection function.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct GamePlayEndDetectionSystem;

type GamePlayEndDetectionSystemData<'s> = (
    Write<'s, EventChannel<GamePlayEvent>>,
    ReadStorage<'s, CharacterStatus>,
);

impl<'s> System<'s> for GamePlayEndDetectionSystem {
    type SystemData = GamePlayEndDetectionSystemData<'s>;

    fn run(&mut self, (mut game_play_ec, character_statuses): Self::SystemData) {
        // Game ends when there is one or less people standing
        let alive_count = character_statuses
            .join()
            .fold(0, |mut alive_count, character_status| {
                if character_status.hp > 0 {
                    alive_count += 1
                };
                alive_count
            });

        if alive_count <= 1 {
            game_play_ec.single_write(GamePlayEvent::End);
        }
    }
}

#[cfg(test)]
mod test {
    use amethyst::{ecs::prelude::*, shrev::EventChannel};
    use amethyst_test::*;
    use game_play_model::GamePlayEvent;
    use object_model::entity::{CharacterStatus, HealthPoints};
    use typename::TypeName;

    use super::{GamePlayEndDetectionSystem, GamePlayEndDetectionSystemData};

    #[test]
    fn sends_game_play_end_event_when_one_alive_character_remaining() {
        let setup = |world: &mut World| {
            world
                .create_entity()
                .with(CharacterStatus {
                    hp: HealthPoints(100),
                    ..Default::default()
                })
                .build();

            // Non-live character.
            world
                .create_entity()
                .with(CharacterStatus {
                    hp: HealthPoints(0),
                    ..Default::default()
                })
                .build();
        };

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<String, String>()
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
            world
                .create_entity()
                .with(CharacterStatus {
                    hp: HealthPoints(0),
                    ..Default::default()
                })
                .build();

            // Non-live character.
            world
                .create_entity()
                .with(CharacterStatus {
                    hp: HealthPoints(0),
                    ..Default::default()
                })
                .build();
        };

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<String, String>()
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
            world
                .create_entity()
                .with(CharacterStatus {
                    hp: HealthPoints(100),
                    ..Default::default()
                })
                .build();

            // Non-live character.
            world
                .create_entity()
                .with(CharacterStatus {
                    hp: HealthPoints(100),
                    ..Default::default()
                })
                .build();
        };

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<String, String>()
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
