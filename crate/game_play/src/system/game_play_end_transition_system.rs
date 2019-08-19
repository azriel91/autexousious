use amethyst::{ecs::prelude::*, shrev::EventChannel};
use derive_new::new;
use game_input::ControllerInput;
use game_play_model::{GamePlayEvent, GamePlayStatus};
use tracker::Last;
use typename_derive::TypeName;

/// Detects the end of a game play round, and fires a `GamePlayEvent::End`.
///
/// In the future this will be type parameterized to specify the detection function.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct GamePlayEndTransitionSystem;

type GamePlayEndTransitionSystemData<'s> = (
    ReadExpect<'s, GamePlayStatus>,
    ReadStorage<'s, Last<ControllerInput>>,
    ReadStorage<'s, ControllerInput>,
    Write<'s, EventChannel<GamePlayEvent>>,
);

impl<'s> System<'s> for GamePlayEndTransitionSystem {
    type SystemData = GamePlayEndTransitionSystemData<'s>;

    fn run(
        &mut self,
        (game_play_status, last_controller_inputs, controller_inputs, mut game_play_ec): Self::SystemData,
    ) {
        if *game_play_status == GamePlayStatus::Ended {
            // Transition when someone presses attack
            let should_transition = (&last_controller_inputs, &controller_inputs).join().fold(
                false,
                |should_transition, (last_controller_input, controller_input)| {
                    should_transition || (!last_controller_input.attack && controller_input.attack)
                },
            );

            if should_transition {
                game_play_ec.single_write(GamePlayEvent::EndStats);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use amethyst::{ecs::prelude::*, input::StringBindings, shrev::EventChannel};
    use amethyst_test::*;
    use game_input::ControllerInput;
    use game_play_model::{GamePlayEvent, GamePlayStatus};
    use tracker::Last;
    use typename::TypeName;

    use super::{GamePlayEndTransitionSystem, GamePlayEndTransitionSystemData};

    #[test]
    fn does_not_send_game_play_end_stats_event_when_game_play_is_not_end() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<StringBindings>()
                .with_resource(GamePlayStatus::Playing)
                .with_setup(register_gpec_reader)
                .with_setup(|world| setup_controller_input(world, false, false))
                .with_system_single(
                    GamePlayEndTransitionSystem::new(),
                    GamePlayEndTransitionSystem::type_name(),
                    &[]
                ) // kcov-ignore
                .with_assertion(|world| verify_game_play_events(world, &[]))
                .run()
                .is_ok()
        );
    }

    #[test]
    fn does_not_send_game_play_end_stats_event_when_attack_is_not_pressed() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<StringBindings>()
                .with_resource(GamePlayStatus::Ended)
                .with_setup(register_gpec_reader)
                .with_setup(|world| setup_controller_input(world, true, false))
                .with_system_single(
                    GamePlayEndTransitionSystem::new(),
                    GamePlayEndTransitionSystem::type_name(),
                    &[]
                ) // kcov-ignore
                .with_assertion(|world| verify_game_play_events(world, &[]))
                .run()
                .is_ok()
        );
    }

    #[test]
    fn does_not_send_game_play_end_stats_event_when_attack_was_previously_pressed_and_is_held() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<StringBindings>()
                .with_resource(GamePlayStatus::Ended)
                .with_setup(register_gpec_reader)
                .with_setup(|world| setup_controller_input(world, true, true))
                .with_system_single(
                    GamePlayEndTransitionSystem::new(),
                    GamePlayEndTransitionSystem::type_name(),
                    &[]
                ) // kcov-ignore
                .with_assertion(|world| verify_game_play_events(world, &[]))
                .run()
                .is_ok()
        );
    }

    #[test]
    fn sends_game_play_end_stats_event_when_attack_was_not_previously_pressed_and_is_now() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<StringBindings>()
                .with_resource(GamePlayStatus::Ended)
                .with_setup(register_gpec_reader)
                .with_setup(|world| setup_controller_input(world, false, true))
                .with_system_single(
                    GamePlayEndTransitionSystem::new(),
                    GamePlayEndTransitionSystem::type_name(),
                    &[]
                ) // kcov-ignore
                .with_assertion(|world| verify_game_play_events(world, &[&GamePlayEvent::EndStats]))
                .run()
                .is_ok()
        );
    }

    fn register_gpec_reader(world: &mut World) {
        GamePlayEndTransitionSystemData::setup(&mut world.res);

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

    fn verify_game_play_events(world: &mut World, expected: &[&GamePlayEvent]) {
        let mut reader_id = &mut world.write_resource::<ReaderId<GamePlayEvent>>();
        let game_play_ec = world.read_resource::<EventChannel<GamePlayEvent>>();

        let actual = game_play_ec
            .read(&mut reader_id)
            .collect::<Vec<&GamePlayEvent>>();

        assert_eq!(expected, &*actual);
    }
}
