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
    use map_selection_model::MapSelection;
    use typename::TypeName;

    use map_selection_ui::{MapSelectionWidget, MapSelectionWidgetUiSystem, WidgetState};

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
            .run()
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
                world.exec(
                    |(mut widgets, asset_type_mappings): (
                        WriteStorage<'_, MapSelectionWidget>,
                        Read<'_, AssetTypeMappings>,
                    )| {
                        let widget = (&mut widgets)
                            .join()
                            .next()
                            .expect("Expected entity with `MapSelectionWidget` component.");

                        let first_map = asset_type_mappings
                            .iter_ids(&AssetType::Map)
                            .next()
                            .copied()
                            .expect("Expected at least one map to be loaded.");

                        widget.state = WidgetState::MapSelect;
                        widget.selection = MapSelection::Random(Some(first_map));
                    },
                );
            })
            .with_system_single(
                MapSelectionWidgetUiSystem::new(),
                MapSelectionWidgetUiSystem::type_name(),
                &[],
            )
            .with_assertion(|world| assert_widget_text(world, "◀      Random      ▶"))
            .run()
    }

    fn input_config() -> InputConfig {
        let controller_config_0 =
            controller_config([VirtualKeyCode::A, VirtualKeyCode::D, VirtualKeyCode::Key1]);
        let controller_config_1 = controller_config([
            VirtualKeyCode::Left,
            VirtualKeyCode::Right,
            VirtualKeyCode::O,
        ]);

        let controller_configs = vec![controller_config_0, controller_config_1];
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
        world.exec(|widgets: ReadStorage<'_, MapSelectionWidget>| {
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
