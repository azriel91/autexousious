use amethyst::{
    core::transform::Parent,
    ecs::{Entities, Join, Read, ReadExpect, System, WriteStorage},
    ui::{Anchor, UiText, UiTransform},
};
use application_ui::{FontVariant, Theme};
use asset_model::loaded::SlugAndHandle;
use derive_new::new;
use game_input::{ControllerInput, InputControlled};
use game_input_model::{ControllerId, InputConfig};
use game_model::loaded::MapAssets;
use log::debug;
use map_selection_model::{MapSelection, MapSelectionEntity, MapSelectionEntityId};
use typename_derive::TypeName;

use crate::{MapSelectionWidget, WidgetState};

const FONT_SIZE_WIDGET: f32 = 30.;
const FONT_SIZE_HELP: f32 = 17.;
const LABEL_WIDTH: f32 = 400.;
const LABEL_HEIGHT: f32 = 75.;
const LABEL_HEIGHT_HELP: f32 = 20.;

/// System that creates and deletes `MapSelectionWidget` entities.
///
/// This is not private because consumers may use `MapSelectionWidgetUiSystem::type_name()` to
/// specify this as a dependency of another system.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct MapSelectionWidgetUiSystem;

type WidgetComponentStorages<'s> = (
    WriteStorage<'s, MapSelectionWidget>,
    WriteStorage<'s, ControllerInput>,
);

type WidgetUiResources<'s> = (
    ReadExpect<'s, Theme>,
    WriteStorage<'s, UiTransform>,
    WriteStorage<'s, UiText>,
    WriteStorage<'s, Parent>,
    WriteStorage<'s, MapSelectionEntity>,
);

type InputControlledResources<'s> = (Read<'s, InputConfig>, WriteStorage<'s, InputControlled>);

type MapSelectionWidgetUiSystemData<'s> = (
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
        (map_selection_widgets, controller_inputs): &mut WidgetComponentStorages<'_>,
        (theme, ui_transforms, ui_texts, parents, map_selection_entities): &mut WidgetUiResources<
            '_,
        >,
    ) {
        if map_selection_widgets.count() == 0 {
            debug!("Initializing Map Selection UI.");

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
                Anchor::MiddleLeft,
                -LABEL_WIDTH / 2.,
                LABEL_HEIGHT / 2.,
                1.,
                LABEL_WIDTH,
                LABEL_HEIGHT,
            );

            let ui_text = UiText::new(
                font.clone(),
                String::from(""),
                [1., 1., 1., 1.],
                FONT_SIZE_WIDGET,
            );

            entities
                .build_entity()
                .with(
                    MapSelectionEntity::new(MapSelectionEntityId),
                    map_selection_entities,
                )
                .with(map_selection_widget, map_selection_widgets)
                .with(ui_transform, ui_transforms)
                .with(ui_text, ui_texts)
                .build();

            (0..input_config.controller_configs.len()).for_each(|index| {
                let controller_id = index as ControllerId;
                entities
                    .build_entity()
                    .with(
                        MapSelectionEntity::new(MapSelectionEntityId),
                        map_selection_entities,
                    )
                    .with(InputControlled::new(controller_id), input_controlleds)
                    .with(ControllerInput::default(), controller_inputs)
                    .build();
            });

            // Instructions label
            //
            // Need to create a container to left justify everything.
            let container_height = LABEL_HEIGHT_HELP * 5.;
            let container_entity = {
                let ui_transform = UiTransform::new(
                    String::from("map_selection_instructions"),
                    Anchor::BottomMiddle,
                    Anchor::BottomMiddle,
                    0.,
                    0.,
                    1.,
                    LABEL_WIDTH,
                    container_height,
                );

                entities
                    .build_entity()
                    .with(
                        MapSelectionEntity::new(MapSelectionEntityId),
                        map_selection_entities,
                    )
                    .with(ui_transform, ui_transforms)
                    .build()
            };
            vec![
                String::from("Press `Left` / `Right` to select map. --------"),
                String::from("Press `Attack` to confirm selection. ---------"),
                String::from("Press `Attack` to move to next screen. -------"),
                String::from("Press `Jump` to go back. ---------------------"),
                String::from(""),
                String::from("See `resources/input_config.ron` for controls."),
            ]
            .into_iter()
            .enumerate()
            .for_each(|(index, string)| {
                let ui_transform = UiTransform::new(
                    format!("map_selection_instructions#{}", index),
                    Anchor::TopLeft,
                    Anchor::TopLeft,
                    0.,
                    -LABEL_HEIGHT_HELP * index as f32,
                    1.,
                    LABEL_WIDTH,
                    LABEL_HEIGHT_HELP,
                );

                let ui_text = UiText::new(font.clone(), string, [1., 1., 1., 1.], FONT_SIZE_HELP);

                let parent = Parent::new(container_entity);

                entities
                    .build_entity()
                    .with(
                        MapSelectionEntity::new(MapSelectionEntityId),
                        map_selection_entities,
                    )
                    .with(ui_transform, ui_transforms)
                    .with(ui_text, ui_texts)
                    .with(parent, parents)
                    .build();
            });
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
                ui_text.text = match widget.state {
                    WidgetState::MapSelect => {
                        format!("◀ {:^16} ▶", format!("{}", widget.selection))
                    }
                    WidgetState::Ready => format!("» {:^16} «", format!("{}", widget.selection)),
                }
            });
    }
}

impl<'s> System<'s> for MapSelectionWidgetUiSystem {
    type SystemData = MapSelectionWidgetUiSystemData<'s>;

    fn run(
        &mut self,
        (
            map_assets,
            entities,
            mut input_controlled_resources,
            mut widget_component_storages,
            mut widget_ui_resources,
        ): Self::SystemData,
    ) {
        self.initialize_ui(
            &map_assets,
            &entities,
            &mut input_controlled_resources,
            &mut widget_component_storages,
            &mut widget_ui_resources,
        );
        self.refresh_ui(&mut widget_component_storages.0, &mut widget_ui_resources.2);
    }
}

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
    use game_input_model::{Axis, ControlAction, ControllerConfig, InputConfig};
    use game_model::loaded::MapAssets;
    use map_selection_model::MapSelection;
    use typename::TypeName;

    use super::MapSelectionWidgetUiSystem;
    use crate::{MapSelectionWidget, WidgetState};

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
