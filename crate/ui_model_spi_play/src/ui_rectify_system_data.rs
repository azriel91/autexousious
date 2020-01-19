use amethyst::{
    ecs::{World, WriteStorage},
    shred::{ResourceId, SystemData},
    ui::UiText,
};
use derivative::Derivative;
use ui_model_spi::play::{Siblings, WidgetStatus};

/// `UiRectifySystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct UiRectifySystemData<'s> {
    /// `WidgetStatus` components.
    #[derivative(Debug = "ignore")]
    pub widget_statuses: WriteStorage<'s, WidgetStatus>,
    /// `Siblings` components.
    #[derivative(Debug = "ignore")]
    pub siblingses: WriteStorage<'s, Siblings>,
    /// `UiText` components.
    #[derivative(Debug = "ignore")]
    pub ui_texts: WriteStorage<'s, UiText>,
}
