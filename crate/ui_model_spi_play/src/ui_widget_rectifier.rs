use amethyst::ecs::Entity;
use ui_model_spi::play::{Siblings, WidgetStatus};

use crate::UiRectifySystemData;

const FONT_COLOUR_ACTIVE: [f32; 4] = [0.9, 0.9, 1., 1.];

/// Attaches `Siblings` component and sets `WidgetStatus::Active` on the first entity.
#[derive(Debug)]
pub struct UiWidgetRectifier;

impl UiWidgetRectifier {
    /// Attaches `Siblings` component and sets `WidgetStatus::Active` on the first entity.
    pub fn rectify(
        UiRectifySystemData {
            widget_statuses,
            siblingses,
            ui_texts,
        }: &mut UiRectifySystemData,
        widget_entities: &[Entity],
    ) {
        // Set first widget to be active.
        if let Some(entity) = widget_entities.first().copied() {
            widget_statuses
                .insert(entity, WidgetStatus::Active)
                .expect("Failed to insert `WidgetStatus` component.");
            if let Some(ui_text) = ui_texts.get_mut(entity) {
                ui_text.color = FONT_COLOUR_ACTIVE;
            }
        }

        // Set previous and next siblings
        if widget_entities.len() >= 2 {
            if let Some(first_item) = widget_entities.first().copied() {
                let second = widget_entities.get(1).copied();
                siblingses
                    .insert(first_item, Siblings::new(None, second))
                    .expect("Failed to insert `Siblings` component.");
            }
            // Skip first menu item.
            //
            // `Vec#get(n)` returns `None` when out of bounds, so the logic works for the
            // last item.
            widget_entities[..]
                .iter()
                .enumerate()
                .skip(1)
                .for_each(|(index, menu_item)| {
                    let prev_item = widget_entities.get(index - 1).copied();
                    let next_item = widget_entities.get(index + 1).copied();
                    siblingses
                        .insert(*menu_item, Siblings::new(prev_item, next_item))
                        .expect("Failed to insert `Siblings` component.");
                });
        }
    }
}
