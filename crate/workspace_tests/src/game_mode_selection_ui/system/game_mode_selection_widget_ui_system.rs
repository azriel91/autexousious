#[cfg(test)]
mod tests {
    use std::{any, collections::HashMap};

    use amethyst::{
        ecs::{Join, ReadStorage, World, WriteStorage},
        input::{Axis as InputAxis, Button},
        ui::UiText,
        winit::VirtualKeyCode,
        Error,
    };
    use application_menu::MenuItem;
    use application_test_support::AutexousiousApplication;
    use game_input_model::{Axis, ControlAction, ControllerConfig, InputConfig};
    use game_mode_selection_model::GameModeIndex;
    use indexmap::IndexMap;
    use state_registry::StateId;
    use strum::IntoEnumIterator;
    use ui_model_spi::play::{Siblings, WidgetStatus};

    use game_mode_selection_ui::{
        GameModeSelectionWidgetUiSystem, FONT_COLOUR_ACTIVE, FONT_COLOUR_IDLE,
    };

    // See `assets_test/assets/test/ui/game_mode_selection/ui.yaml`.
    const GAME_MODE_MENU_ITEM_COUNT: usize = 3;

    #[test]
    fn initializes_ui_when_widget_statuses_zero() -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_resource(input_config())
            .with_system(
                GameModeSelectionWidgetUiSystem::new(),
                any::type_name::<GameModeSelectionWidgetUiSystem>(),
                &[],
            )
            .with_effect(|world| world.insert(StateId::GameModeSelection))
            .with_assertion(|world| assert_widget_count(world, GAME_MODE_MENU_ITEM_COUNT))
            .with_assertion(|world| assert_siblings_correct(world))
            .run_isolated()
    }

    #[test]
    fn updates_idle_menu_item_colour() -> Result<(), Error> {
        AutexousiousApplication::config_base()
            // Set up UI
            .with_system(
                GameModeSelectionWidgetUiSystem::new(),
                any::type_name::<GameModeSelectionWidgetUiSystem>(),
                &[],
            )
            .with_effect(|world| world.insert(StateId::GameModeSelection))
            .with_assertion(|world| assert_widget_count(world, GAME_MODE_MENU_ITEM_COUNT))
            // Set widget state to idle.
            .with_effect(|world| {
                let mut widget_statuses = world.system_data::<WriteStorage<'_, WidgetStatus>>();
                let widget_status = (&mut widget_statuses)
                    .join()
                    .next()
                    .expect("Expected entity with `WidgetStatus` component.");

                *widget_status = WidgetStatus::Idle;
            })
            .with_assertion(|world| assert_text_colour(world, FONT_COLOUR_IDLE))
            .run_isolated()
    }

    #[test]
    fn updates_active_menu_item_colour() -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_resource(input_config())
            // Set up UI
            .with_system(
                GameModeSelectionWidgetUiSystem::new(),
                any::type_name::<GameModeSelectionWidgetUiSystem>(),
                &[],
            )
            .with_effect(|world| world.insert(StateId::GameModeSelection))
            .with_assertion(|world| assert_widget_count(world, GAME_MODE_MENU_ITEM_COUNT))
            // Set widget state to active.
            .with_effect(|world| {
                let mut widget_statuses = world.system_data::<WriteStorage<'_, WidgetStatus>>();
                (&mut widget_statuses).join().for_each(|widget_status| {
                    *widget_status = WidgetStatus::Active;
                });
            })
            .with_assertion(|world| assert_text_colour(world, FONT_COLOUR_ACTIVE))
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
        let (menu_items, widget_statuses, siblingses, ui_texts) = world.system_data::<(
            ReadStorage<'_, MenuItem<GameModeIndex>>,
            ReadStorage<'_, WidgetStatus>,
            ReadStorage<'_, Siblings>,
            ReadStorage<'_, UiText>,
        )>();
        assert_eq!(
            count,
            (&menu_items, &widget_statuses, &siblingses, &ui_texts)
                .join()
                .count()
        );
    }

    fn assert_siblings_correct(world: &mut World) {
        let (menu_items, siblingses) = world.system_data::<(
            ReadStorage<'_, MenuItem<GameModeIndex>>,
            ReadStorage<'_, Siblings>,
        )>();

        GameModeIndex::iter().for_each(|index| {
            let (_menu_item, siblings) = (&menu_items, &siblingses)
                .join()
                .filter(|(menu_item, _)| menu_item.index == index)
                .next()
                .unwrap_or_else(|| panic!("Expected `MenuItem` to exist for index: {:?}.", index));

            match index {
                GameModeIndex::StartGame => {
                    assert!(siblings.previous.is_none());
                    if let Some(next) = siblings.next.as_ref() {
                        let next_menu_item = menu_items.get(*next);
                        assert_eq!(
                            Some(MenuItem::new(GameModeIndex::ControlSettings)).as_ref(),
                            next_menu_item
                        );
                    } else {
                        panic!("Expected `StartGame` to have `next` sibling.")
                    }
                }
                GameModeIndex::ControlSettings => {
                    if let Some(previous) = siblings.previous.as_ref() {
                        let previous_menu_item = menu_items.get(*previous);
                        assert_eq!(
                            Some(MenuItem::new(GameModeIndex::StartGame)).as_ref(),
                            previous_menu_item
                        );
                    } else {
                        panic!("Expected `ControlSettings` to have `previous` sibling.")
                    }
                    if let Some(next) = siblings.next.as_ref() {
                        let next_menu_item = menu_items.get(*next);
                        assert_eq!(
                            Some(MenuItem::new(GameModeIndex::Exit)).as_ref(),
                            next_menu_item
                        );
                    } else {
                        panic!("Expected `StartGame` to have `next` sibling.")
                    }
                }
                GameModeIndex::Exit => {
                    if let Some(previous) = siblings.previous.as_ref() {
                        let previous_menu_item = menu_items.get(*previous);
                        assert_eq!(
                            Some(MenuItem::new(GameModeIndex::ControlSettings)).as_ref(),
                            previous_menu_item
                        );
                    } else {
                        panic!("Expected `Exit` to have `previous` sibling.")
                    }
                    assert!(siblings.next.is_none());
                }
            }
        });
    }

    fn assert_text_colour(world: &mut World, colour: [f32; 4]) {
        let (widgets, ui_texts) =
            world.system_data::<(ReadStorage<'_, WidgetStatus>, ReadStorage<'_, UiText>)>();
        let (_widget, ui_text) = (&widgets, &ui_texts)
            .join()
            .next()
            .expect("Expected entity to exist.");
        assert_eq!(colour, ui_text.color);
    }
}
