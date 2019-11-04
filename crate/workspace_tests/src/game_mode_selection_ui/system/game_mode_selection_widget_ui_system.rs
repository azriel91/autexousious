#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use amethyst::{
        ecs::{Join, ReadStorage, World, WorldExt, WriteStorage},
        input::{Axis as InputAxis, Button},
        shrev::EventChannel,
        ui::UiText,
        winit::VirtualKeyCode,
        Error,
    };
    use application_menu::{MenuItem, MenuItemWidgetState, Siblings};
    use application_test_support::AutexousiousApplication;
    use game_input_model::{Axis, ControlAction, ControllerConfig, InputConfig};
    use game_mode_selection_model::GameModeIndex;
    use state_registry::{StateId, StateIdUpdateEvent};
    use strum::IntoEnumIterator;
    use typename::TypeName;

    use game_mode_selection_ui::{
        GameModeSelectionWidgetUiSystem, FONT_COLOUR_ACTIVE, FONT_COLOUR_IDLE,
    };

    // See `assets_test/assets/test/ui/game_mode_selection/ui.yaml`.
    const GAME_MODE_MENU_ITEM_COUNT: usize = 2;

    #[test]
    fn initializes_ui_when_menu_item_widget_states_zero() -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_resource(input_config())
            .with_system(
                GameModeSelectionWidgetUiSystem::new(),
                GameModeSelectionWidgetUiSystem::type_name(),
                &[],
            )
            .with_effect(send_state_id_update_event)
            .with_assertion(|world| assert_widget_count(world, GAME_MODE_MENU_ITEM_COUNT))
            .with_assertion(|world| assert_siblings_correct(world))
            .run_isolated()
    }

    #[test]
    fn updates_idle_menu_item_colour() -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_resource(input_config())
            // Set up UI
            .with_system(
                GameModeSelectionWidgetUiSystem::new(),
                GameModeSelectionWidgetUiSystem::type_name(),
                &[],
            )
            .with_effect(send_state_id_update_event)
            .with_assertion(|world| assert_widget_count(world, GAME_MODE_MENU_ITEM_COUNT))
            // Set widget state to idle.
            .with_effect(|world| {
                let mut menu_item_widget_states =
                    world.system_data::<WriteStorage<'_, MenuItemWidgetState>>();
                let menu_item_widget_state = (&mut menu_item_widget_states)
                    .join()
                    .next()
                    .expect("Expected entity with `MenuItemWidgetState` component.");

                *menu_item_widget_state = MenuItemWidgetState::Idle;
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
                GameModeSelectionWidgetUiSystem::type_name(),
                &[],
            )
            .with_effect(send_state_id_update_event)
            .with_assertion(|world| assert_widget_count(world, GAME_MODE_MENU_ITEM_COUNT))
            // Set widget state to active.
            .with_effect(|world| {
                let mut menu_item_widget_states =
                    world.system_data::<WriteStorage<'_, MenuItemWidgetState>>();
                let menu_item_widget_state = (&mut menu_item_widget_states)
                    .join()
                    .next()
                    .expect("Expected entity with `MenuItemWidgetState` component.");

                *menu_item_widget_state = MenuItemWidgetState::Active;
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

    fn send_state_id_update_event(world: &mut World) {
        let mut state_id_update_ec = world.write_resource::<EventChannel<StateIdUpdateEvent>>();
        state_id_update_ec.single_write(StateIdUpdateEvent::new(StateId::GameModeSelection, None));
    }

    fn assert_widget_count(world: &mut World, count: usize) {
        let (menu_items, menu_item_widget_states, siblingses, ui_texts) = world.system_data::<(
            ReadStorage<'_, MenuItem<GameModeIndex>>,
            ReadStorage<'_, MenuItemWidgetState>,
            ReadStorage<'_, Siblings>,
            ReadStorage<'_, UiText>,
        )>();
        assert_eq!(
            count,
            (
                &menu_items,
                &menu_item_widget_states,
                &siblingses,
                &ui_texts
            )
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
                            Some(MenuItem::new(GameModeIndex::StartGame)).as_ref(),
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
        let (widgets, ui_texts) = world.system_data::<(
            ReadStorage<'_, MenuItemWidgetState>,
            ReadStorage<'_, UiText>,
        )>();
        let (_widget, ui_text) = (&widgets, &ui_texts)
            .join()
            .next()
            .expect("Expected entity to exist.");
        assert_eq!(colour, ui_text.color);
    }
}
