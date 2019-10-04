use amethyst::{
    core::transform::Parent,
    ecs::{Entities, Entity, Join, ReadExpect, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    ui::{Anchor, UiText, UiTransform},
};
use application_menu::{MenuItem, MenuItemWidgetState, Siblings};
use application_ui::{FontVariant, Theme};
use derivative::Derivative;
use derive_new::new;
use game_input::{ControllerInput, InputControlled};
use game_input_model::{ControllerId, InputConfig};
use game_mode_selection_model::{
    GameModeIndex, GameModeSelectionEntity, GameModeSelectionEntityId,
};
use heck::TitleCase;
use log::debug;
use strum::IntoEnumIterator;
use typename_derive::TypeName;

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
pub struct GameModeSelectionWidgetUiSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct GameModeSelectionWidgetUiSystemData<'s> {
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
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
        }: &mut GameModeSelectionWidgetUiSystemData<'_>,
    ) {
        if menu_item_widget_states.count() == 0 {
            debug!("Initializing GameMode Selection UI.");

            let font = theme
                .fonts
                .get(&FontVariant::Bold)
                .expect("Failed to get regular font handle.");

            let item_count = GameModeIndex::iter().len();
            let menu_items = GameModeIndex::iter()
                .enumerate()
                .map(|(order, index)| {
                    let index_id = index.to_string();
                    let ui_transform = UiTransform::new(
                        format!("menu_item_widget#{}", index_id),
                        Anchor::Middle,
                        Anchor::MiddleLeft,
                        -LABEL_WIDTH / 2.,
                        ((item_count - order) as f32 * LABEL_HEIGHT)
                            - (item_count as f32 * LABEL_HEIGHT / 2.),
                        1.,
                        LABEL_WIDTH,
                        LABEL_HEIGHT,
                    );

                    let index_text = index_id.to_title_case();
                    let ui_text =
                        UiText::new(font.clone(), index_text, FONT_COLOUR_IDLE, FONT_SIZE_WIDGET);

                    // Set first item to `Active`.
                    let menu_item_widget_state = if order == 0 {
                        MenuItemWidgetState::Active
                    } else {
                        MenuItemWidgetState::Idle
                    };

                    entities
                        .build_entity()
                        .with(
                            GameModeSelectionEntity::new(GameModeSelectionEntityId),
                            game_mode_selection_entities,
                        )
                        .with(MenuItem::new(index), menu_items)
                        .with(menu_item_widget_state, menu_item_widget_states)
                        .with(ui_transform, ui_transforms)
                        .with(ui_text, ui_texts)
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
                    .with(
                        GameModeSelectionEntity::new(GameModeSelectionEntityId),
                        game_mode_selection_entities,
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
                    .with(
                        GameModeSelectionEntity::new(GameModeSelectionEntityId),
                        game_mode_selection_entities,
                    )
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

                let ui_text = UiText::new(font.clone(), string, FONT_COLOUR_HELP, FONT_SIZE_HELP);

                let parent = Parent::new(container_entity);

                entities
                    .build_entity()
                    .with(
                        GameModeSelectionEntity::new(GameModeSelectionEntityId),
                        game_mode_selection_entities,
                    )
                    .with(ui_transform, ui_transforms)
                    .with(ui_text, ui_texts)
                    .with(parent, parents)
                    .build();
            });
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

    fn run(&mut self, mut menu_item_widget_ui_system_data: Self::SystemData) {
        self.initialize_ui(&mut menu_item_widget_ui_system_data);

        self.refresh_ui(
            &menu_item_widget_ui_system_data.menu_item_widget_states,
            &mut menu_item_widget_ui_system_data.ui_texts,
        )
    }
}
