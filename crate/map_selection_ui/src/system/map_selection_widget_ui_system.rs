use amethyst::{
    core::transform::Parent,
    ecs::{Entities, Join, Read, ReadExpect, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    ui::{Anchor, UiText, UiTransform},
};
use application_ui::{FontVariant, Theme};
use asset_model::{
    config::AssetType,
    loaded::{AssetIdMappings, AssetTypeMappings},
};
use derivative::Derivative;
use derive_new::new;
use game_input::{ControllerInput, InputControlled};
use game_input_model::{ControllerId, InputConfig};
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
pub struct MapSelectionWidgetUiSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct MapSelectionWidgetUiSystemData<'s> {
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `AssetIdMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_id_mappings: Read<'s, AssetIdMappings>,
    /// `AssetTypeMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_type_mappings: Read<'s, AssetTypeMappings>,
    /// `InputConfig` resource.
    #[derivative(Debug = "ignore")]
    pub input_config: Read<'s, InputConfig>,
    /// `InputControlled` components.
    #[derivative(Debug = "ignore")]
    pub input_controlleds: WriteStorage<'s, InputControlled>,
    /// `MapSelectionWidget` components.
    #[derivative(Debug = "ignore")]
    pub map_selection_widgets: WriteStorage<'s, MapSelectionWidget>,
    /// `ControllerInput` components.
    #[derivative(Debug = "ignore")]
    pub controller_inputs: WriteStorage<'s, ControllerInput>,
    /// `Theme` resource.
    #[derivative(Debug = "ignore")]
    pub theme: ReadExpect<'s, Theme>,
    /// `UiTransform` components.
    #[derivative(Debug = "ignore")]
    pub ui_transforms: WriteStorage<'s, UiTransform>,
    /// `UiText` components.
    #[derivative(Debug = "ignore")]
    pub ui_texts: WriteStorage<'s, UiText>,
    /// `Parent` components.
    #[derivative(Debug = "ignore")]
    pub parents: WriteStorage<'s, Parent>,
    /// `MapSelectionEntity` components.
    #[derivative(Debug = "ignore")]
    pub map_selection_entities: WriteStorage<'s, MapSelectionEntity>,
}

impl MapSelectionWidgetUiSystem {
    fn initialize_ui(
        &mut self,
        MapSelectionWidgetUiSystemData {
            entities,
            asset_type_mappings,
            input_config,
            input_controlleds,
            map_selection_widgets,
            controller_inputs,
            theme,
            ui_transforms,
            ui_texts,
            parents,
            map_selection_entities,
            ..
        }: &mut MapSelectionWidgetUiSystemData<'_>,
    ) {
        if map_selection_widgets.count() == 0 {
            debug!("Initializing Map Selection UI.");

            let font = theme
                .fonts
                .get(&FontVariant::Regular)
                .expect("Failed to get regular font handle.");

            let first_map_asset_id = asset_type_mappings
                .iter_ids(&AssetType::Map)
                .next()
                .copied()
                .expect("Expected at least one map to be loaded.");

            let map_selection_widget = MapSelectionWidget::new(
                WidgetState::default(),
                MapSelection::Random(Some(first_map_asset_id)),
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
        MapSelectionWidgetUiSystemData {
            asset_id_mappings,
            map_selection_widgets,
            ui_texts,
            ..
        }: &mut MapSelectionWidgetUiSystemData<'_>,
    ) {
        (map_selection_widgets, ui_texts)
            .join()
            .for_each(|(widget, ui_text)| {
                ui_text.text = {
                    let slug_string = match widget.selection {
                        MapSelection::None => String::from("None"),
                        MapSelection::Random(..) => String::from("Random"),
                        MapSelection::Id(asset_id) => {
                            let slug = asset_id_mappings
                                .slug(asset_id)
                                .expect("Expected slug to exist for map selection.");
                            format!("{}", slug)
                        }
                    };

                    match widget.state {
                        WidgetState::MapSelect => format!("◀ {:^16} ▶", slug_string),
                        WidgetState::Ready => format!("» {:^16} «", slug_string),
                    }
                }
            });
    }
}

impl<'s> System<'s> for MapSelectionWidgetUiSystem {
    type SystemData = MapSelectionWidgetUiSystemData<'s>;

    fn run(&mut self, mut map_selection_widget_ui_system_data: Self::SystemData) {
        self.initialize_ui(&mut map_selection_widget_ui_system_data);
        self.refresh_ui(&mut map_selection_widget_ui_system_data);
    }
}
