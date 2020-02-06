use amethyst::{
    ecs::{Join, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    ui::UiText,
};
use derivative::Derivative;
use derive_new::new;
use ui_model_spi::play::WidgetStatus;

/// Visible for testing.
pub const FONT_COLOUR_IDLE: [f32; 4] = [0.55, 0.55, 0.55, 1.];
/// Visible for testing.
pub const FONT_COLOUR_ACTIVE: [f32; 4] = [1., 1., 1., 1.];

/// Updates `UiText` colour based on `WidgetStatus`.
#[derive(Debug, new)]
pub struct UiTextColourUpdateSystem;

/// `UiTextColourUpdateSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct UiTextColourUpdateSystemData<'s> {
    /// `WidgetStatus` components.
    #[derivative(Debug = "ignore")]
    pub widget_statuses: ReadStorage<'s, WidgetStatus>,
    /// `UiText` components.
    #[derivative(Debug = "ignore")]
    pub ui_texts: WriteStorage<'s, UiText>,
}

impl<'s> System<'s> for UiTextColourUpdateSystem {
    type SystemData = UiTextColourUpdateSystemData<'s>;

    fn run(
        &mut self,
        UiTextColourUpdateSystemData {
            widget_statuses,
            mut ui_texts,
        }: Self::SystemData,
    ) {
        (&widget_statuses, &mut ui_texts)
            .join()
            .for_each(|(widget_status, ui_text)| {
                ui_text.color = match widget_status {
                    WidgetStatus::Idle => FONT_COLOUR_IDLE,
                    WidgetStatus::Active => FONT_COLOUR_ACTIVE,
                }
            });
    }
}
