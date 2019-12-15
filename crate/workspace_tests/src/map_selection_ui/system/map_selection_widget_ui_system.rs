#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use amethyst::{
        ecs::{Join, Read, ReadStorage, World, WriteStorage},
        input::{Axis as InputAxis, Button, VirtualKeyCode},
        ui::UiText,
        Error,
    };
    use application_test_support::AutexousiousApplication;
    use asset_model::{config::AssetType, loaded::AssetTypeMappings};
    use game_input_model::{Axis, ControlAction, ControllerConfig, InputConfig};
    use indexmap::IndexMap;
    use map_selection_model::MapSelection;
    use typename::TypeName;

    use map_selection_ui::{MapSelectionWidgetState, MapSelectionWidgetUiSystem};

    #[test]
    fn initializes_ui_when_map_selections_waiting() -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_resource(input_config())
            .with_system_single(
                MapSelectionWidgetUiSystem::new(),
                MapSelectionWidgetUiSystem::type_name(),
                &[],
            )
            .with_assertion(|world| assert_widget_count(world, 1))
            .with_assertion(|world| assert_widget_text(world, "◀      Random      ▶"))
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
                MapSelectionWidgetUiSystem::new(),
                MapSelectionWidgetUiSystem::type_name(),
                &[],
            )
            .with_assertion(|world| assert_widget_count(world, 1))
            // Select map and send event
            .with_effect(|world| {
                let (mut map_selections, mut map_selection_widget_states, asset_type_mappings) =
                    world.system_data::<(
                        WriteStorage<'_, MapSelection>,
                        WriteStorage<'_, MapSelectionWidgetState>,
                        Read<'_, AssetTypeMappings>,
                    )>();

                let (map_selection, map_selection_widget_state) =
                    (&mut map_selections, &mut map_selection_widget_states)
                        .join()
                        .next()
                        .expect("Expected entity to exist.");

                let first_map = asset_type_mappings
                    .iter_ids(&AssetType::Map)
                    .next()
                    .copied()
                    .expect("Expected at least one map to be loaded.");

                *map_selection = MapSelection::Random(Some(first_map));
                *map_selection_widget_state = MapSelectionWidgetState::MapSelect;
            })
            .with_system_single(
                MapSelectionWidgetUiSystem::new(),
                MapSelectionWidgetUiSystem::type_name(),
                &[],
            )
            .with_assertion(|world| assert_widget_text(world, "◀      Random      ▶"))
            .run_isolated()
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
        world.exec(|widgets: ReadStorage<'_, MapSelectionWidgetState>| {
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
