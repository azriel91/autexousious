use amethyst::{
    core::transform::Parent,
    ecs::{Entities, Join, Read, ReadExpect, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
    ui::{Anchor, UiText, UiTransform},
};
use application_menu::MenuItemWidgetState;
use application_ui::{FontVariant, Theme};
use asset_model::loaded::AssetIdMappings;
use derivative::Derivative;
use derive_new::new;
use game_mode_selection_model::GameModeSelectionEntity;
use log::debug;
use shrev_support::EventChannelExt;
use state_registry::{StateId, StateIdUpdateEvent};

/// Visible for testing.
pub const FONT_COLOUR_IDLE: [f32; 4] = [0.65, 0.65, 0.65, 1.];
/// Visible for testing.
pub const FONT_COLOUR_ACTIVE: [f32; 4] = [0.9, 0.9, 1., 1.];
const FONT_COLOUR_HELP: [f32; 4] = [1.; 4];
const FONT_SIZE_HELP: f32 = 17.;
const LABEL_WIDTH: f32 = 400.;
const LABEL_HEIGHT_HELP: f32 = 20.;

/// System to manage the `GameModeSelection` UI widgets.
#[derive(Debug, Default, new)]
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
    /// `Theme` resource.
    #[derivative(Debug = "ignore")]
    pub theme: ReadExpect<'s, Theme>,
    /// `MenuItemWidgetState` components.
    #[derivative(Debug = "ignore")]
    pub menu_item_widget_states: WriteStorage<'s, MenuItemWidgetState>,
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
            ui_transforms,
            ui_texts,
            parents,
            game_mode_selection_entities,
            ..
        }: &mut GameModeSelectionWidgetUiSystemData<'_>,
    ) {
        debug!("Initializing GameMode Selection UI.");

        let font = theme
            .fonts
            .get(&FontVariant::Bold)
            .expect("Failed to get regular font handle.");

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

            let ui_text = UiText::new(font.clone(), string, FONT_COLOUR_HELP, FONT_SIZE_HELP);

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

        if let Some(StateIdUpdateEvent {
            state_id: StateId::GameModeSelection,
            ..
        }) = game_mode_selection_widget_ui_system_data
            .state_id_update_ec
            .last_event(state_id_update_event_rid)
        {
            self.initialize_ui(&mut game_mode_selection_widget_ui_system_data);
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
