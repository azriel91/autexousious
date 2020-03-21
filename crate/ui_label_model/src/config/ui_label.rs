use amethyst::{
    core::math::Vector3,
    ecs::{
        storage::DenseVecStorage, Component, Entity, ReadExpect, ReadStorage, World, WriteStorage,
    },
    shred::{ResourceId, SystemData},
    ui::{Anchor, LineMode, UiText, UiTransform},
};
use application_ui::{FontVariant, Theme};
use asset_model::ItemComponent;
use derivative::Derivative;
use derive_new::new;
use kinematic_model::config::{Position, PositionInit};
use serde::{Deserialize, Serialize};
use ui_model_spi::config::Dimensions;

const FONT_COLOUR: [f32; 4] = [0.55, 0.55, 0.55, 1.];
const FONT_SIZE: u32 = 30;
const LABEL_WIDTH: u32 = 400;
const LABEL_HEIGHT: u32 = 75;

/// Defines text to display.
#[derive(Clone, Debug, Derivative, Deserialize, Component, PartialEq, Serialize, new)]
#[derivative(Default)]
#[serde(default, deny_unknown_fields)]
#[storage(DenseVecStorage)]
pub struct UiLabel {
    /// Position of the label relative to its parent.
    pub position: PositionInit,
    /// Text to display.
    pub text: String,
    /// Width and height of the text input. Defaults to `400x75`.
    #[derivative(Default(value = "UiLabel::dimensions_default()"))]
    pub dimensions: Dimensions,
    /// Where to align the text within the text field.
    #[derivative(Default(value = "Anchor::Middle"))]
    pub align: Anchor,
    /// Whether text should be on one line, or wrap.
    #[derivative(Default(value = "LineMode::Single"))]
    pub line_mode: LineMode,
    /// Font colour of the text.
    #[derivative(Default(value = "FONT_COLOUR"))]
    pub font_colour: [f32; 4],
    /// The height of a line of text in pixels.
    #[derivative(Default(value = "FONT_SIZE"))]
    pub font_size: u32,
}

impl UiLabel {
    fn dimensions_default() -> Dimensions {
        Dimensions {
            w: LABEL_WIDTH,
            h: LABEL_HEIGHT,
        }
    }
}

/// `UiLabelSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct UiLabelSystemData<'s> {
    /// `Theme` resource.
    #[derivative(Debug = "ignore")]
    pub theme: ReadExpect<'s, Theme>,
    /// `Position<f32>` components.
    #[derivative(Debug = "ignore")]
    pub positions: ReadStorage<'s, Position<f32>>,
    /// `UiTransform` components.
    #[derivative(Debug = "ignore")]
    pub ui_transforms: WriteStorage<'s, UiTransform>,
    /// `UiText` components.
    #[derivative(Debug = "ignore")]
    pub ui_texts: WriteStorage<'s, UiText>,
}

impl<'s> ItemComponent<'s> for UiLabel {
    type SystemData = UiLabelSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let UiLabelSystemData {
            theme,
            positions,
            ui_transforms,
            ui_texts,
        } = system_data;

        let font = theme
            .fonts
            .get(&FontVariant::Bold)
            .expect("Failed to get regular font handle.");

        let position = positions
            .get(entity)
            .copied()
            .unwrap_or_else(|| Position::from(self.position));
        let position = Into::<Vector3<f32>>::into(position);

        let mut ui_transform = UiTransform::new(
            self.text.clone(),
            Anchor::BottomLeft,
            Anchor::BottomLeft,
            position.x,
            position.y,
            position.z,
            self.dimensions.w as f32,
            self.dimensions.h as f32,
        );
        ui_transform.opaque = false;
        ui_transform.transparent_target = true;

        let index_text = self.text.clone();
        let mut ui_text = UiText::new(
            font.clone(),
            index_text,
            self.font_colour,
            self.font_size as f32,
        );
        ui_text.align = self.align;
        ui_text.line_mode = self.line_mode;

        ui_transforms
            .insert(entity, ui_transform)
            .expect("Failed to insert `UiTransform` component.");
        ui_texts
            .insert(entity, ui_text)
            .expect("Failed to insert `UiText` component.");
    }
}
