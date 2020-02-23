use amethyst::{
    ecs::{storage::DenseVecStorage, Component, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
    ui::TextEditing,
};
use asset_model::ItemComponent;
use derivative::Derivative;
use derive_new::new;
use serde::{Deserialize, Serialize};
use ui_label_model::config::{UiLabel, UiLabelSystemData};

const MAX_LENGTH_DEFAULT: usize = 100;
const SELECTED_TEXT_COLOUR_DEFAULT: [f32; 4] = [0., 0., 0., 0.8];
const SELECTED_BACKGROUND_COLOUR_DEFAULT: [f32; 4] = [1., 1., 1., 0.8];
const USE_BLOCK_CURSOR: bool = false;

/// Defines a text input field.
#[derive(Clone, Component, Debug, Derivative, Deserialize, PartialEq, Serialize, new)]
#[derivative(Default)]
#[serde(default, deny_unknown_fields)]
pub struct UiTextInput {
    /// Attributes of the rendered label text.
    #[serde(flatten)]
    pub label_attributes: UiLabel,
    /// The maximum graphemes permitted in this input field.
    #[derivative(Default(value = "MAX_LENGTH_DEFAULT"))]
    pub max_length: usize,
    /// The color of the text when highlighted.
    #[derivative(Default(value = "SELECTED_TEXT_COLOUR_DEFAULT"))]
    pub selected_text_colour: [f32; 4],
    /// The background color of the text when highlighted.
    #[derivative(Default(value = "SELECTED_BACKGROUND_COLOUR_DEFAULT"))]
    pub selected_background_colour: [f32; 4],
}

/// `UiTextInputSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct UiTextInputSystemData<'s> {
    /// `UiLabelSystemData`.
    pub ui_label_system_data: UiLabelSystemData<'s>,
    /// `TextEditing` components.
    #[derivative(Debug = "ignore")]
    pub text_editings: WriteStorage<'s, TextEditing>,
}

impl<'s> ItemComponent<'s> for UiTextInput {
    type SystemData = UiTextInputSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let UiTextInputSystemData {
            ui_label_system_data,
            text_editings,
        } = system_data;

        self.label_attributes.augment(ui_label_system_data, entity);

        if text_editings.get(entity).is_none() {
            let text_editing = TextEditing::new(
                self.max_length,
                self.selected_text_colour,
                self.selected_background_colour,
                USE_BLOCK_CURSOR,
            );
            text_editings
                .insert(entity, text_editing)
                .expect("Failed to insert `TextEditing` component.");
        }
    }
}
