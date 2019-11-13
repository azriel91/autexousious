use amethyst::{
    core::{
        math::Vector3,
        transform::{Parent, Transform},
    },
    ecs::{Entities, Entity, Join, Read, ReadExpect, System, World, WriteStorage},
    renderer::Transparent,
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
    ui::{Anchor, UiText, UiTransform},
};
use application_menu::{MenuItem, MenuItemWidgetState, Siblings};
use application_ui::{FontVariant, Theme};
use asset_model::loaded::{AssetId, AssetIdMappings};
use background_model::loaded::AssetBackgroundLayers;
use derivative::Derivative;
use derive_new::new;
use game_input::{ControllerInput, InputControlled};
use game_input_model::{ControllerId, InputConfig};
use game_mode_selection_model::{GameModeIndex, GameModeSelectionEntity};
use kinematic_model::{config::Position, loaded::AssetPositionInits};
use log::debug;
use sequence_model::{
    loaded::SequenceId,
    play::{FrameIndexClock, FrameWaitClock},
};
use shrev_support::EventChannelExt;
use state_registry::StateIdUpdateEvent;
use state_support::StateAssetUtils;
use typename_derive::TypeName;
use ui_label_model::loaded::AssetUiLabels;
use ui_menu_item_model::loaded::AssetUiMenuItems;

/// Visible for testing.
pub const FONT_COLOUR_IDLE: [f32; 4] = [0.65, 0.65, 0.65, 1.];
/// Visible for testing.
pub const FONT_COLOUR_ACTIVE: [f32; 4] = [0.8, 0.9, 1., 1.];
const FONT_COLOUR_HELP: [f32; 4] = [1.; 4];
const FONT_SIZE_WIDGET: f32 = 30.;
const FONT_SIZE_HELP: f32 = 17.;
const LABEL_WIDTH: f32 = 400.;
const LABEL_HEIGHT: f32 = 75.;
const LABEL_HEIGHT_HELP: f32 = 20.;

/// System to manage the `GameModeSelection` UI widgets.
#[derive(Debug, Default, TypeName, new)]
pub struct GameModeSelectionWidgetUiSystem {
    /// Reader ID for the `StateIdUpdateEvent` channel.
    #[new(default)]
    state_id_update_event_rid: Option<ReaderId<StateIdUpdateEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct GameModeSelectionWidgetUiSystemData<'s> {
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `StateIdUpdateEvent` channel.
    #[derivative(Debug = "ignore")]
    pub state_id_update_ec: Read<'s, EventChannel<StateIdUpdateEvent>>,
    /// `AssetIdMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_id_mappings: Read<'s, AssetIdMappings>,
    /// `AssetBackgroundLayers` resource.
    #[derivative(Debug = "ignore")]
    pub asset_background_layers: Read<'s, AssetBackgroundLayers>,
    /// `AssetUiLabels` resource.
    #[derivative(Debug = "ignore")]
    pub asset_ui_labels: Read<'s, AssetUiLabels>,
    /// `AssetUiMenuItems<GameModeIndex>` resource.
    #[derivative(Debug = "ignore")]
    pub asset_ui_menu_items: Read<'s, AssetUiMenuItems<GameModeIndex>>,
    /// `AssetPositionInits` resource.
    #[derivative(Debug = "ignore")]
    pub asset_position_inits: Read<'s, AssetPositionInits>,
    /// `AssetId` components.
    #[derivative(Debug = "ignore")]
    pub asset_ids: WriteStorage<'s, AssetId>,
    /// `SequenceId` components.
    #[derivative(Debug = "ignore")]
    pub sequence_ids: WriteStorage<'s, SequenceId>,
    /// `Transparent` components.
    #[derivative(Debug = "ignore")]
    pub transparents: WriteStorage<'s, Transparent>,
    /// `Position<f32>` components.
    #[derivative(Debug = "ignore")]
    pub positions: WriteStorage<'s, Position<f32>>,
    /// `Transform` components.
    #[derivative(Debug = "ignore")]
    pub transforms: WriteStorage<'s, Transform>,
    /// `FrameIndexClock` components.
    #[derivative(Debug = "ignore")]
    pub frame_index_clocks: WriteStorage<'s, FrameIndexClock>,
    /// `FrameWaitClock` components.
    #[derivative(Debug = "ignore")]
    pub frame_wait_clocks: WriteStorage<'s, FrameWaitClock>,
    /// `Theme` resource.
    #[derivative(Debug = "ignore")]
    pub theme: ReadExpect<'s, Theme>,
    /// `InputConfig` resource.
    #[derivative(Debug = "ignore")]
    pub input_config: ReadExpect<'s, InputConfig>,
    /// `MenuItem` components.
    #[derivative(Debug = "ignore")]
    pub menu_items: WriteStorage<'s, MenuItem<GameModeIndex>>,
    /// `MenuItemWidgetState` components.
    #[derivative(Debug = "ignore")]
    pub menu_item_widget_states: WriteStorage<'s, MenuItemWidgetState>,
    /// `Siblings` components.
    #[derivative(Debug = "ignore")]
    pub siblingses: WriteStorage<'s, Siblings>,
    /// `InputControlled` components.
    #[derivative(Debug = "ignore")]
    pub input_controlleds: WriteStorage<'s, InputControlled>,
    /// `ControllerInput` components.
    #[derivative(Debug = "ignore")]
    pub controller_inputs: WriteStorage<'s, ControllerInput>,
    /// `UiTransform` components.
    #[derivative(Debug = "ignore")]
    pub ui_transforms: WriteStorage<'s, UiTransform>,
    /// `UiText` components.
    #[derivative(Debug = "ignore")]
    pub ui_texts: WriteStorage<'s, UiText>,
    /// `Parent` components.
    #[derivative(Debug = "ignore")]
    pub parents: WriteStorage<'s, Parent>,
    /// `GameModeSelectionEntity` components.
    #[derivative(Debug = "ignore")]
    pub game_mode_selection_entities: WriteStorage<'s, GameModeSelectionEntity>,
}

impl GameModeSelectionWidgetUiSystem {
    fn initialize_ui(
        &mut self,
        GameModeSelectionWidgetUiSystemData {
            entities,
            asset_background_layers,
            asset_ui_labels,
            asset_ui_menu_items,
            asset_position_inits,
            asset_ids,
            sequence_ids,
            transparents,
            positions,
            transforms,
            frame_index_clocks,
            frame_wait_clocks,
            theme,
            input_config,
            menu_items,
            menu_item_widget_states,
            siblingses,
            input_controlleds,
            controller_inputs,
            ui_transforms,
            ui_texts,
            parents,
            game_mode_selection_entities,
            ..
        }: &mut GameModeSelectionWidgetUiSystemData<'_>,
        asset_id: AssetId,
    ) {
        if menu_item_widget_states.count() == 0 {
            debug!("Initializing GameMode Selection UI.");

            let font = theme
                .fonts
                .get(&FontVariant::Bold)
                .expect("Failed to get regular font handle.");

            let ui_menu_items = asset_ui_menu_items.get(asset_id);
            let ui_labels = asset_ui_labels.get(asset_id);
            let position_inits = asset_position_inits.get(asset_id);

            // Hack: We need to correctly map the item components together.
            let position_inits_to_skip = asset_background_layers
                .get(asset_id)
                .map(|background_layers| background_layers.len())
                .unwrap_or(0);
            // End Hack

            if let (Some(ui_menu_items), Some(ui_labels), Some(position_inits)) =
                (ui_menu_items, ui_labels, position_inits)
            {
                let menu_items = ui_menu_items
                    .iter()
                    .enumerate()
                    .zip(ui_labels.iter())
                    .zip(position_inits.iter().skip(position_inits_to_skip).copied())
                    .map(|(((order, ui_menu_item), ui_label), position_init)| {
                        let index = ui_menu_item.index;
                        let sequence_id = ui_menu_item.sequence_id;
                        let position_combined = position_init + ui_label.position;

                        let translation = Into::<Vector3<f32>>::into(position_combined);
                        let position = Position::from(translation);
                        let mut transform = Transform::default();
                        transform.set_translation(translation);

                        let ui_transform = UiTransform::new(
                            index.to_string(),
                            Anchor::BottomLeft,
                            Anchor::BottomLeft,
                            position.x,
                            position.y,
                            position.z,
                            LABEL_WIDTH,
                            LABEL_HEIGHT,
                        );

                        let index_text = ui_label.text.clone();
                        let ui_text = UiText::new(
                            font.clone(),
                            index_text,
                            FONT_COLOUR_IDLE,
                            FONT_SIZE_WIDGET,
                        );

                        // Set first item to `Active`.
                        let menu_item_widget_state = if order == 0 {
                            MenuItemWidgetState::Active
                        } else {
                            MenuItemWidgetState::Idle
                        };

                        entities
                            .build_entity()
                            .with(GameModeSelectionEntity, game_mode_selection_entities)
                            .with(MenuItem::new(index), menu_items)
                            .with(menu_item_widget_state, menu_item_widget_states)
                            .with(ui_transform, ui_transforms)
                            .with(ui_text, ui_texts)
                            .with(sequence_id, sequence_ids)
                            .with(asset_id, asset_ids)
                            .with(Transparent, transparents)
                            .with(position, positions)
                            .with(transform, transforms)
                            .with(FrameIndexClock::new(1), frame_index_clocks)
                            .with(FrameWaitClock::new(1), frame_wait_clocks)
                            .build()
                    })
                    .collect::<Vec<Entity>>();

                // Set previous and next siblings
                if menu_items.len() >= 2 {
                    if let Some(first_item) = menu_items.first() {
                        let second = menu_items.get(1).cloned();
                        siblingses
                            .insert(*first_item, Siblings::new(None, second))
                            .expect("Failed to insert `Siblings` component.");
                    }
                    // Skip first menu item.
                    //
                    // `Vec#get(n)` returns `None` when out of bounds, so the logic works for the last
                    // item.
                    menu_items[..]
                        .iter()
                        .enumerate()
                        .skip(1)
                        .for_each(|(index, menu_item)| {
                            let prev_item = menu_items.get(index - 1).cloned();
                            let next_item = menu_items.get(index + 1).cloned();
                            siblingses
                                .insert(*menu_item, Siblings::new(prev_item, next_item))
                                .expect("Failed to insert `Siblings` component.");
                        });
                }

                (0..input_config.controller_configs.len()).for_each(|index| {
                    let controller_id = index as ControllerId;
                    entities
                        .build_entity()
                        .with(GameModeSelectionEntity, game_mode_selection_entities)
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
                        String::from("game_mode_selection_instructions"),
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
                        .with(GameModeSelectionEntity, game_mode_selection_entities)
                        .with(ui_transform, ui_transforms)
                        .build()
                };
                vec![
                    String::from("Press `Up` / `Down` to select game mode. -----"),
                    String::from("Press `Attack` to confirm selection. ---------"),
                    String::from(""),
                    String::from("See `resources/input_config.ron` for controls."),
                ]
                .into_iter()
                .enumerate()
                .for_each(|(index, string)| {
                    let ui_transform = UiTransform::new(
                        format!("game_mode_selection_instructions#{}", index),
                        Anchor::TopLeft,
                        Anchor::TopLeft,
                        0.,
                        -LABEL_HEIGHT_HELP * index as f32,
                        1.,
                        LABEL_WIDTH,
                        LABEL_HEIGHT_HELP,
                    );

                    let ui_text =
                        UiText::new(font.clone(), string, FONT_COLOUR_HELP, FONT_SIZE_HELP);

                    let parent = Parent::new(container_entity);

                    entities
                        .build_entity()
                        .with(GameModeSelectionEntity, game_mode_selection_entities)
                        .with(ui_transform, ui_transforms)
                        .with(ui_text, ui_texts)
                        .with(parent, parents)
                        .build();
                });
            }
        }
    }

    fn refresh_ui(
        &self,
        menu_item_widget_states: &WriteStorage<'_, MenuItemWidgetState>,
        ui_texts: &mut WriteStorage<'_, UiText>,
    ) {
        (menu_item_widget_states, ui_texts)
            .join()
            .for_each(|(menu_item_widget_state, ui_text)| {
                ui_text.color = match menu_item_widget_state {
                    MenuItemWidgetState::Idle => FONT_COLOUR_IDLE,
                    MenuItemWidgetState::Active => FONT_COLOUR_ACTIVE,
                }
            });
    }
}

impl<'s> System<'s> for GameModeSelectionWidgetUiSystem {
    type SystemData = GameModeSelectionWidgetUiSystemData<'s>;

    fn run(&mut self, mut game_mode_selection_widget_ui_system_data: Self::SystemData) {
        let state_id_update_event_rid = self
            .state_id_update_event_rid
            .as_mut()
            .expect("Expected `state_id_update_event_rid` field to be set.");

        if let Some(ev) = game_mode_selection_widget_ui_system_data
            .state_id_update_ec
            .last_event(state_id_update_event_rid)
        {
            let asset_id = StateAssetUtils::asset_id(
                &game_mode_selection_widget_ui_system_data.asset_id_mappings,
                ev.state_id,
            );
            if let Some(asset_id) = asset_id {
                self.initialize_ui(&mut game_mode_selection_widget_ui_system_data, asset_id);
            }
        }

        self.refresh_ui(
            &game_mode_selection_widget_ui_system_data.menu_item_widget_states,
            &mut game_mode_selection_widget_ui_system_data.ui_texts,
        )
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.state_id_update_event_rid = Some(
            world
                .fetch_mut::<EventChannel<StateIdUpdateEvent>>()
                .register_reader(),
        );
    }
}
