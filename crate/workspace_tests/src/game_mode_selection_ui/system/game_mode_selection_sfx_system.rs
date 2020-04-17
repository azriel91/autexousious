#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{World, WorldExt},
        shrev::EventChannel,
        Error,
    };
    use application_test_support::AutexousiousApplication;
    use game_mode_selection_model::{GameModeIndex, GameModeSelectionEvent};
    use menu_model::MenuEvent;

    use game_mode_selection_ui::GameModeSelectionSfxSystem;

    #[test]
    fn plays_sound_on_select_event() -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_system(GameModeSelectionSfxSystem::new(), "", &[])
            .with_effect(|world| {
                let event = MenuEvent::Select(GameModeIndex::StartGame);
                send_event(world, event);
            })
            .with_assertion(|_world| {
                // TODO: assert sound was played.
            })
            .run_winit_loop()
    }

    fn send_event(world: &mut World, event: GameModeSelectionEvent) {
        let mut ec = world.write_resource::<EventChannel<GameModeSelectionEvent>>();
        ec.single_write(event)
    } // kcov-ignore
}
