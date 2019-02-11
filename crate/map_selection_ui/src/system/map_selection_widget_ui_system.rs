use amethyst::{
    ecs::prelude::*,
    shrev::{EventChannel, ReaderId},
    ui::{Anchor, UiText, UiTransform},
};
use application_ui::{FontVariant, Theme};
use asset_model::loaded::{MapAssets, SlugAndHandle};
use derive_new::new;
use game_input::{ControllerInput, InputControlled, SharedInputControlled};
use game_input_model::{ControllerId, InputConfig};
use log::debug;
use map_selection::MapSelectionStatus;
use map_selection_model::{MapSelection, MapSelectionEvent};
use typename_derive::TypeName;

use crate::{MapSelectionWidget, WidgetState};

const FONT_SIZE: f32 = 20.;

/// System that creates and deletes `MapSelectionWidget` entities.
///
/// This is not private because consumers may use `MapSelectionWidgetUiSystem::type_name()` to
/// specify this as a dependency of another system.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct MapSelectionWidgetUiSystem {
    /// Whether the UI is initialized.
    #[new(value = "false")]
    ui_initialized: bool,
    /// Reader ID for the `MapSelectionEvent` event channel.
    ///
    /// This is used to determine to delete the UI entities, as the `MapSelectionStatus` is
    /// only updated by the `MapSelectionsSystem` which happens after this system runs.
    #[new(default)]
    reader_id: Option<ReaderId<MapSelectionEvent>>,
    /// Entities used for the UI.
    ///
    /// This includes not only the widget entity, but the entities used to receive input.
    #[new(default)]
    entities: Vec<Entity>,
}

type WidgetComponentStorages<'s> = (
    WriteStorage<'s, MapSelectionWidget>,
    WriteStorage<'s, SharedInputControlled>,
    WriteStorage<'s, ControllerInput>,
);

type WidgetUiResources<'s> = (
    ReadExpect<'s, Theme>,
    WriteStorage<'s, UiTransform>,
    WriteStorage<'s, UiText>,
);

type InputControlledResources<'s> = (Read<'s, InputConfig>, WriteStorage<'s, InputControlled>);

type MapSelectionWidgetUiSystemData<'s> = (
    Read<'s, EventChannel<MapSelectionEvent>>,
    Read<'s, MapSelectionStatus>,
    Read<'s, MapAssets>,
    Entities<'s>,
    InputControlledResources<'s>,
    WidgetComponentStorages<'s>,
    WidgetUiResources<'s>,
);

impl MapSelectionWidgetUiSystem {
    fn initialize_ui(
        &mut self,
        map_assets: &MapAssets,
        entities: &Entities<'_>,
        (input_config, input_controlleds): &mut InputControlledResources<'_>,
        (
            map_selection_widgets,
            shared_input_controlleds,
            controller_inputs
        ): &mut WidgetComponentStorages<'_>,
        (theme, ui_transforms, ui_texts): &mut WidgetUiResources<'_>,
    ) {
        if !self.ui_initialized {
            debug!("Initializing Map Selection UI.");

            self.ui_initialized = true;

            let text_w = 250.;
            let text_h = 50.;

            let font = theme
                .fonts
                .get(&FontVariant::Regular)
                .expect("Failed to get regular font handle.");

            let first_map = map_assets
                .iter()
                .next()
                .expect("Expected at least one map to be loaded.");

            let map_selection_widget = MapSelectionWidget::new(
                WidgetState::default(),
                MapSelection::Random(SlugAndHandle::from(first_map)),
            );

            let ui_transform = UiTransform::new(
                "MapSelectionWidget".to_string(),
                Anchor::Middle,
                0.,
                text_h / 2.,
                1.,
                text_w,
                text_h,
            );

            let ui_text = UiText::new(font.clone(), String::from(""), [1., 1., 1., 1.], FONT_SIZE);

            let entity = entities
                .build_entity()
                .with(map_selection_widget, map_selection_widgets)
                .with(SharedInputControlled, shared_input_controlleds)
                // Deliberately do not add `ControllerInput`:
                //
                // The first run of the `SharedControllerInputUpdateSystem` will automatically add
                // this.
                //
                // If we add the component, and someone was holding Attack from the previous UI
                // state, then the `LastTrackerSystem` will track that previously Attack wasn't
                // pressed, and the UI logic assumes "it was just pressed" and selects the map.
                // ---
                // .with(ControllerInput::default(), controller_inputs)
                .with(ui_transform, ui_transforms)
                .with(ui_text, ui_texts)
                .build();

            self.entities.push(entity);

            let controller_entities_iter =
                (0..input_config.controller_configs.len()).map(|index| {
                    let controller_id = index as ControllerId;
                    entities
                        .build_entity()
                        .with(InputControlled::new(controller_id), input_controlleds)
                        .with(ControllerInput::default(), controller_inputs)
                        .build()
                });

            self.entities.extend(controller_entities_iter);
        }
    }

    fn refresh_ui(
        &mut self,
        map_selection_widgets: &mut WriteStorage<'_, MapSelectionWidget>,
        ui_texts: &mut WriteStorage<'_, UiText>,
    ) {
        (map_selection_widgets, ui_texts)
            .join()
            .for_each(|(widget, ui_text)| {
                ui_text.text = format!("{}", widget.selection);
            });
    }

    fn terminate_ui(&mut self, entities: &Entities<'_>) {
        if self.ui_initialized {
            self.entities.drain(..).for_each(|e| {
                entities
                    .delete(e)
                    .expect("Failed to delete `MapSelectionUI` entity.")
            });

            self.ui_initialized = false;
        }
    }
}

impl<'s> System<'s> for MapSelectionWidgetUiSystem {
    type SystemData = MapSelectionWidgetUiSystemData<'s>;

    fn run(
        &mut self,
        (
            map_selection_events,
            map_selection_status,
            map_assets,
            entities,
            mut input_controlled_resources,
            mut widget_component_storages,
            mut widget_ui_resources,
        ): Self::SystemData,
    ) {
        // We need to do this because the `MapSelectionStatus` is not updated until after this
        // system has run, and so we don't actually get a chance to delete the UI entities.
        if map_selection_events
            .read(
                self.reader_id
                    .as_mut()
                    .expect("Expected to read `MapSelectionEvent`s."),
            )
            .next()
            .is_some()
        {
            self.terminate_ui(&entities);
            return;
        }

        match *map_selection_status {
            MapSelectionStatus::Pending => {
                self.initialize_ui(
                    &map_assets,
                    &entities,
                    &mut input_controlled_resources,
                    &mut widget_component_storages,
                    &mut widget_ui_resources,
                );
                self.refresh_ui(&mut widget_component_storages.0, &mut widget_ui_resources.2);
            }
            MapSelectionStatus::Confirmed => self.terminate_ui(&entities),
        };
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.reader_id = Some(
            res.fetch_mut::<EventChannel<MapSelectionEvent>>()
                .register_reader(),
        );
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use amethyst::{
        ecs::prelude::*,
        input::{Axis as InputAxis, Button},
        renderer::VirtualKeyCode,
        shrev::EventChannel,
        ui::UiText,
    };
    use application_test_support::AutexousiousApplication;
    use asset_model::loaded::{MapAssets, SlugAndHandle};
    use assets_test::ASSETS_MAP_EMPTY_SLUG;
    use game_input_model::{Axis, ControlAction, ControllerConfig, InputConfig};
    use map_selection::MapSelectionStatus;
    use map_selection_model::{MapSelection, MapSelectionEvent};
    use typename::TypeName;

    use super::MapSelectionWidgetUiSystem;
    use crate::{MapSelectionWidget, WidgetState};

    #[test]
    fn initializes_ui_when_map_selections_waiting() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::config_base(
                "initializes_ui_when_map_selections_waiting",
                false
            )
            .with_resource(input_config())
            .with_setup(|world| {
                world.add_resource(MapSelectionStatus::Pending);
            })
            .with_system_single(
                MapSelectionWidgetUiSystem::new(),
                MapSelectionWidgetUiSystem::type_name(),
                &[]
            )
            .with_assertion(|world| assert_widget_count(world, 1))
            .with_assertion(|world| assert_widget_text(world, "Random"))
            .run()
            .is_ok()
        );
    }

    #[test]
    fn refreshes_ui_when_selections_select_random() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::config_base(
                "refreshes_ui_when_selections_select_random",
                false
            )
            .with_system(
                MapSelectionWidgetUiSystem::new(),
                MapSelectionWidgetUiSystem::type_name(),
                &[]
            ) // kcov-ignore
            // Set up UI
            .with_resource(input_config())
            .with_assertion(|world| assert_widget_count(world, 1))
            // Select map and send event
            .with_effect(|world| {
                world.add_resource(MapSelectionStatus::Pending);
            })
            .with_effect(|world| {
                world.exec(
                    |(mut widgets, map_assets): (
                        WriteStorage<'_, MapSelectionWidget>,
                        Read<'_, MapAssets>,
                    )| {
                        let widget = (&mut widgets)
                            .join()
                            .next()
                            .expect("Expected entity with `MapSelectionWidget` component.");

                        let first_map = map_assets
                            .iter()
                            .next()
                            .expect("Expected at least one map to be loaded.");

                        widget.state = WidgetState::MapSelect;
                        widget.selection = MapSelection::Random(first_map.into());
                    },
                );

                let first_map = world
                    .read_resource::<MapAssets>()
                    .iter()
                    .next()
                    .expect("Expected at least one map to be loaded.")
                    .into();

                send_event(
                    world,
                    MapSelectionEvent::Select {
                        map_selection: MapSelection::Random(first_map),
                    },
                )
            })
            .with_effect(|_| {}) // Need an extra update for the event to get through.
            .with_assertion(|world| assert_widget_text(world, "Random"))
            .run()
            .is_ok()
        );
    }

    #[test]
    fn terminates_ui_when_select_event_sent() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::config_base("terminates_ui_when_confirm_event_sent", false)
                .with_system(
                    MapSelectionWidgetUiSystem::new(),
                    MapSelectionWidgetUiSystem::type_name(),
                    &[]
                ) // kcov-ignore
                // Set up UI
                .with_resource(input_config())
                .with_assertion(|world| assert_widget_count(world, 1))
                // Confirm selection and send event
                .with_effect(|world| {
                    world.add_resource(MapSelectionStatus::Confirmed);
                })
                .with_effect(|world| {
                    let empty_snh = SlugAndHandle::from((
                        &*world.read_resource::<MapAssets>(),
                        ASSETS_MAP_EMPTY_SLUG.clone(),
                    ));

                    send_event(
                        world,
                        MapSelectionEvent::Select {
                            map_selection: MapSelection::Id(empty_snh),
                        },
                    )
                })
                .with_effect(|_| {}) // Need an extra update for the event to get through.
                .with_assertion(|world| assert_widget_count(world, 0))
                .run()
                .is_ok()
        );
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

    fn send_event(world: &mut World, event: MapSelectionEvent) {
        world
            .write_resource::<EventChannel<MapSelectionEvent>>()
            .single_write(event);
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
