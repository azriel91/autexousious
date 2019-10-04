#[cfg(test)]
mod test {
    use amethyst::{
        ecs::{Builder, SystemData, World, WorldExt},
        input::StringBindings,
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use game_input::ControllerInput;
    use game_play_model::{GamePlayEvent, GamePlayStatus};
    use tracker::Last;
    use typename::TypeName;

    use game_play::{GamePlayEndTransitionSystem, GamePlayEndTransitionSystemData};

    #[test]
    fn does_not_send_game_play_end_stats_event_when_game_play_is_not_end() -> Result<(), Error> {
        AmethystApplication::ui_base::<StringBindings>()
            .with_resource(GamePlayStatus::Playing)
            .with_effect(register_gpec_reader)
            .with_effect(|world| setup_controller_input(world, false, false))
            .with_system_single(
                GamePlayEndTransitionSystem::new(),
                GamePlayEndTransitionSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_assertion(|world| verify_game_play_events(world, &[]))
            .run()
    }

    #[test]
    fn does_not_send_game_play_end_stats_event_when_attack_is_not_pressed() -> Result<(), Error> {
        AmethystApplication::ui_base::<StringBindings>()
            .with_resource(GamePlayStatus::Ended)
            .with_effect(register_gpec_reader)
            .with_effect(|world| setup_controller_input(world, true, false))
            .with_system_single(
                GamePlayEndTransitionSystem::new(),
                GamePlayEndTransitionSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_assertion(|world| verify_game_play_events(world, &[]))
            .run()
    }

    #[test]
    fn does_not_send_game_play_end_stats_event_when_attack_was_previously_pressed_and_is_held(
    ) -> Result<(), Error> {
        AmethystApplication::ui_base::<StringBindings>()
            .with_resource(GamePlayStatus::Ended)
            .with_effect(register_gpec_reader)
            .with_effect(|world| setup_controller_input(world, true, true))
            .with_system_single(
                GamePlayEndTransitionSystem::new(),
                GamePlayEndTransitionSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_assertion(|world| verify_game_play_events(world, &[]))
            .run()
    }

    #[test]
    fn sends_game_play_end_stats_event_when_attack_was_not_previously_pressed_and_is_now(
    ) -> Result<(), Error> {
        AmethystApplication::ui_base::<StringBindings>()
            .with_resource(GamePlayStatus::Ended)
            .with_effect(register_gpec_reader)
            .with_effect(|world| setup_controller_input(world, false, true))
            .with_system_single(
                GamePlayEndTransitionSystem::new(),
                GamePlayEndTransitionSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_assertion(|world| verify_game_play_events(world, &[&GamePlayEvent::EndStats]))
            .run()
    }

    fn register_gpec_reader(world: &mut World) {
        GamePlayEndTransitionSystemData::setup(world);

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
