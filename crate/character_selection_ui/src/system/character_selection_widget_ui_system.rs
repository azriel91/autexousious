use amethyst::{
    core::transform::Parent,
    ecs::{Entities, Join, Read, ReadExpect, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    ui::{Anchor, UiText, UiTransform},
};
use application_ui::{FontVariant, Theme};
use asset_model::loaded::AssetIdMappings;
use character_selection_model::{
    CharacterSelection, CharacterSelectionEntity, CharacterSelectionEntityId,
};
use derivative::Derivative;
use derive_new::new;
use game_input::{ControllerInput, InputControlled};
use game_input_model::{ControllerId, InputConfig};
use log::debug;
use typename_derive::TypeName;

use crate::{CharacterSelectionWidget, WidgetState};

const FONT_SIZE_WIDGET: f32 = 30.;
const FONT_SIZE_HELP: f32 = 17.;
const LABEL_WIDTH: f32 = 400.;
const LABEL_HEIGHT: f32 = 75.;
const LABEL_HEIGHT_HELP: f32 = 20.;

/// System that creates and deletes `CharacterSelectionWidget` entities.
///
/// This is not private because consumers may use `CharacterSelectionWidgetUiSystem::type_name()` to
/// specify this as a dependency of another system.
#[derive(Debug, Default, TypeName, new)]
pub struct CharacterSelectionWidgetUiSystem;

/// `CharacterSelectionWidgetUiSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CharacterSelectionWidgetUiSystemData<'s> {
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `AssetIdMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_id_mappings: Read<'s, AssetIdMappings>,
    /// `InputConfig` resource.
    #[derivative(Debug = "ignore")]
    pub input_config: Read<'s, InputConfig>,
    /// `InputControlled` components.
    #[derivative(Debug = "ignore")]
    pub input_controlleds: WriteStorage<'s, InputControlled>,
    /// `CharacterSelectionWidget` components.
    #[derivative(Debug = "ignore")]
    pub character_selection_widgets: WriteStorage<'s, CharacterSelectionWidget>,
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
    /// `CharacterSelectionEntity` components.
    #[derivative(Debug = "ignore")]
    pub character_selection_entities: WriteStorage<'s, CharacterSelectionEntity>,
}

impl CharacterSelectionWidgetUiSystem {
    fn initialize_ui(
        &mut self,
        CharacterSelectionWidgetUiSystemData {
            entities,
            input_config,
            input_controlleds,
            character_selection_widgets,
            controller_inputs,
            theme,
            ui_transforms,
            ui_texts,
            parents,
            character_selection_entities,
            ..
        }: &mut CharacterSelectionWidgetUiSystemData<'_>,
    ) {
        if character_selection_widgets.count() == 0 {
            debug!("Initializing Character Selection UI.");

            let controller_count = input_config.controller_configs.len();

            let font = theme
                .fonts
                .get(&FontVariant::Regular)
                .expect("Failed to get regular font handle.");

            (0..controller_count).for_each(|index| {
                let controller_id = index as ControllerId;

                let character_selection_widget = CharacterSelectionWidget::new(
                    WidgetState::default(),
                    CharacterSelection::Random,
                );

                let ui_transform = UiTransform::new(
                    format!("character_selection_widget#{}", controller_id),
                    Anchor::Middle,
                    Anchor::MiddleLeft,
                    -LABEL_WIDTH / 2.,
                    ((controller_count - index) as f32 * LABEL_HEIGHT)
                        - (controller_count as f32 * LABEL_HEIGHT / 2.),
                    1.,
                    LABEL_WIDTH,
                    LABEL_HEIGHT,
                );

                let ui_text = UiText::new(
                    font.clone(),
                    "Press Attack To Join".to_string(),
                    [1., 1., 1., 1.],
                    FONT_SIZE_WIDGET,
                );

                entities
                    .build_entity()
                    .with(
                        CharacterSelectionEntity::new(CharacterSelectionEntityId),
                        character_selection_entities,
                    )
                    .with(character_selection_widget, character_selection_widgets)
                    .with(InputControlled::new(controller_id), input_controlleds)
                    .with(ControllerInput::default(), controller_inputs)
                    .with(ui_transform, ui_transforms)
                    .with(ui_text, ui_texts)
                    .build();
            });

            // Instructions label
            //
            // Need to create a container to left justify everything.
            let container_height = LABEL_HEIGHT_HELP * 5.;
            let container_entity = {
                let ui_transform = UiTransform::new(
                    String::from("character_selection_instructions"),
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
                        CharacterSelectionEntity::new(CharacterSelectionEntityId),
                        character_selection_entities,
                    )
                    .with(ui_transform, ui_transforms)
                    .build()
            };
            vec![
                String::from("Press `Attack` to join. ----------------------"),
                String::from("Press `Left` / `Right` to select character. --"),
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
                    format!("character_selection_instructions#{}", index),
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
                        CharacterSelectionEntity::new(CharacterSelectionEntityId),
                        character_selection_entities,
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
        CharacterSelectionWidgetUiSystemData {
            asset_id_mappings,
            character_selection_widgets,
            ui_texts,
            ..
        }: &mut CharacterSelectionWidgetUiSystemData<'_>,
    ) {
        (character_selection_widgets, ui_texts)
            .join()
            .for_each(|(widget, ui_text)| {
                let slug_string = match widget.selection {
                    CharacterSelection::Random => String::from("Random"),
                    CharacterSelection::Id(asset_id) => {
                        let slug = asset_id_mappings
                            .slug(asset_id)
                            .expect("Expected slug to exist for character selection.");
                        format!("{}", slug)
                    }
                };

                ui_text.text = match widget.state {
                    WidgetState::Inactive => "Press Attack To Join".to_string(),
                    WidgetState::CharacterSelect => format!("◀ {:^16} ▶", slug_string),
                    WidgetState::Ready => format!("» {:^16} «", slug_string),
                }
            });
    }
}

impl<'s> System<'s> for CharacterSelectionWidgetUiSystem {
    type SystemData = CharacterSelectionWidgetUiSystemData<'s>;

    fn run(&mut self, mut character_selection_widget_ui_system_data: Self::SystemData) {
        self.initialize_ui(&mut character_selection_widget_ui_system_data);
        self.refresh_ui(&mut character_selection_widget_ui_system_data);
    }
}
