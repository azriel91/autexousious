#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use amethyst::{
        ecs::{Join, ReadStorage, World, WriteStorage},
        input::{Axis as InputAxis, Button, VirtualKeyCode},
        ui::UiText,
        Error,
    };
    use application_test_support::{AssetQueries, AutexousiousApplication};
    use assets_test::CHAR_BAT_SLUG;
    use character_selection_model::CharacterSelection;
    use game_input_model::{Axis, ControlAction, ControllerConfig, InputConfig};
    use indexmap::IndexMap;
    use typename::TypeName;

    use character_selection_ui::{CharacterSelectionWidgetState, CharacterSelectionWidgetUiSystem};

    #[test]
    fn initializes_ui_when_character_selections_waiting() -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_effect(|world| world.insert(input_config()))
            .with_system_single(
                CharacterSelectionWidgetUiSystem::new(),
                CharacterSelectionWidgetUiSystem::type_name(),
                &[],
            )
            .with_assertion(|world| assert_widget_count(world, 2))
            .with_assertion(|world| assert_widget_text(world, "Press Attack To Join"))
            .run_isolated()
    }

    #[test]
    fn refreshes_ui_when_selections_select_random() -> Result<(), Error> {
        AutexousiousApplication::config_base()
            // Set up UI
            .with_resource(input_config())
            // Run this in its own dispatcher, otherwise the LoadingState hasn't had time to
            // complete.
            .with_system_single(
                CharacterSelectionWidgetUiSystem::new(),
                CharacterSelectionWidgetUiSystem::type_name(),
                &[],
            )
            .with_assertion(|world| assert_widget_count(world, 2))
            // Select character and send event
            .with_effect(|world| {
                let (mut character_selections, mut character_selection_widget_states) = world
                    .system_data::<(
                        WriteStorage<'_, CharacterSelection>,
                        WriteStorage<'_, CharacterSelectionWidgetState>,
                    )>();
                let (character_selection, character_selection_widget_state) = (
                    &mut character_selections,
                    &mut character_selection_widget_states,
                )
                    .join()
                    .next()
                    .expect("Expected entity to exist.");

                *character_selection = CharacterSelection::Random;
                *character_selection_widget_state = CharacterSelectionWidgetState::CharacterSelect;
            })
            .with_system_single(
                CharacterSelectionWidgetUiSystem::new(),
                CharacterSelectionWidgetUiSystem::type_name(),
                &[],
            )
            .with_assertion(|world| assert_widget_text(world, "◀      Random      ▶"))
            .run_isolated()
    }

    #[test]
    fn refreshes_ui_when_selections_select_id() -> Result<(), Error> {
        AutexousiousApplication::config_base()
            // Set up UI
            .with_resource(input_config())
            .with_system_single(
                CharacterSelectionWidgetUiSystem::new(),
                CharacterSelectionWidgetUiSystem::type_name(),
                &[],
            )
            .with_assertion(|world| assert_widget_count(world, 2))
            // Select character and send event
            .with_effect(|world| {
                let (mut character_selections, mut character_selection_widget_states) = world
                    .system_data::<(
                        WriteStorage<'_, CharacterSelection>,
                        WriteStorage<'_, CharacterSelectionWidgetState>,
                    )>();
                let (character_selection, character_selection_widget_state) = (
                    &mut character_selections,
                    &mut character_selection_widget_states,
                )
                    .join()
                    .next()
                    .expect("Expected entity to exist.");

                let bat_asset_id = AssetQueries::id(world, &*CHAR_BAT_SLUG);
                *character_selection = CharacterSelection::Id(bat_asset_id);

                *character_selection_widget_state = CharacterSelectionWidgetState::CharacterSelect;
            })
            .with_system_single(
                CharacterSelectionWidgetUiSystem::new(),
                CharacterSelectionWidgetUiSystem::type_name(),
                &[],
            )
            .with_assertion(|world| assert_widget_text(world, "◀     test/bat     ▶"))
            .run_isolated() // kcov-ignore
    }

    fn input_config() -> InputConfig {
        let controller_config_0 =
            controller_config([VirtualKeyCode::A, VirtualKeyCode::D, VirtualKeyCode::Key1]);
        let controller_config_1 = controller_config([
            VirtualKeyCode::Left,
            VirtualKeyCode::Right,
            VirtualKeyCode::O,
        ]);

        let mut controller_configs = IndexMap::new();
        controller_configs.insert(String::from("zero1"), controller_config_0);
        controller_configs.insert(String::from("one"), controller_config_1);
        InputConfig::new(controller_configs)
    }

    fn controller_config(keys: [VirtualKeyCode; 3]) -> ControllerConfig {
        let mut axes = HashMap::new();
        axes.insert(
            Axis::X,
            InputAxis::Emulated {
                neg: Button::Key(keys[0]),
                pos: Button::Key(keys[1]),
            },
        );
        let mut actions = HashMap::new();
        actions.insert(ControlAction::Jump, Button::Key(keys[2]));
        ControllerConfig::new(axes, actions)
    }

    fn assert_widget_count(world: &mut World, count: usize) {
        world.exec(|widgets: ReadStorage<'_, CharacterSelectionWidgetState>| {
            assert_eq!(count, widgets.join().count());
        });
    }

    fn assert_widget_text(world: &mut World, text: &str) {
        world.exec(|ui_texts: ReadStorage<'_, UiText>| {
            assert_eq!(
                text,
                ui_texts
                    .join()
                    .next()
                    .expect("Expected entity with `UiText` component to exist.")
                    .text
            );
        });
    }
}
