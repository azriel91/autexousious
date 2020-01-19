use amethyst::{
    ecs::{Entity, Read, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
    ui::UiText,
};
use asset_model::loaded::ItemId;
use derivative::Derivative;
use derive_new::new;
use shrev_support::EventChannelExt;
use state_registry::{StateIdUpdateEvent, StateItemEntities};
use ui_model_spi::play::{Siblings, WidgetStatus};

const FONT_COLOUR_ACTIVE: [f32; 4] = [0.9, 0.9, 1., 1.];

/// Adds the `Siblings` component to `UiMenuItem` item entities.
///
/// `UiMenuItem` state item entities need to be spawned before the `Siblings` components can be
/// instantiated to point to the correct sibling entities.
#[derive(Debug, Default, new)]
pub struct StateItemUiRectifySystem {
    /// Reader ID for the `StateIdUpdateEvent` channel.
    #[new(default)]
    state_id_update_event_rid: Option<ReaderId<StateIdUpdateEvent>>,
}

/// `StateItemUiRectifySystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct StateItemUiRectifySystemData<'s> {
    /// `StateIdUpdateEvent` channel.
    #[derivative(Debug = "ignore")]
    pub state_id_update_ec: Read<'s, EventChannel<StateIdUpdateEvent>>,
    /// `StateItemEntities` resource.
    #[derivative(Debug = "ignore")]
    pub state_item_entities: Read<'s, StateItemEntities>,
    /// `ItemId` components.
    #[derivative(Debug = "ignore")]
    pub item_ids: ReadStorage<'s, ItemId>,
    /// `WidgetStatus` components.
    #[derivative(Debug = "ignore")]
    pub widget_statuses: WriteStorage<'s, WidgetStatus>,
    /// `UiText` components.
    #[derivative(Debug = "ignore")]
    pub ui_texts: WriteStorage<'s, UiText>,
    /// `Siblings` components.
    #[derivative(Debug = "ignore")]
    pub siblingses: WriteStorage<'s, Siblings>,
}

impl<'s> System<'s> for StateItemUiRectifySystem {
    type SystemData = StateItemUiRectifySystemData<'s>;

    fn run(
        &mut self,
        StateItemUiRectifySystemData {
            state_id_update_ec,
            state_item_entities,
            item_ids,
            mut widget_statuses,
            mut ui_texts,
            mut siblingses,
        }: Self::SystemData,
    ) {
        let state_id_update_event_rid = self
            .state_id_update_event_rid
            .as_mut()
            .expect("Expected `state_id_update_event_rid` field to be set.");

        if let Some(_ev) = state_id_update_ec.last_event(state_id_update_event_rid) {
            let mut menu_item_entities = state_item_entities
                .entities
                .iter()
                .filter(|entity| widget_statuses.get(**entity).is_some())
                .copied()
                .map(|entity| {
                    let item_id = item_ids
                        .get(entity)
                        .copied()
                        .expect("Expected `ItemId` component to exist.");
                    (entity, item_id)
                })
                .collect::<Vec<(Entity, ItemId)>>();

            menu_item_entities
                .sort_unstable_by(|(_, item_id_0), (_, item_id_1)| item_id_0.cmp(item_id_1));
            let menu_item_entities = menu_item_entities
                .into_iter()
                .map(|(entity, _item_id)| entity)
                .collect::<Vec<Entity>>();

            // Set first menu item to be active.
            if let Some(entity) = menu_item_entities.first().copied() {
                widget_statuses
                    .insert(entity, WidgetStatus::Active)
                    .expect("Failed to insert `WidgetStatus` component.");
                if let Some(ui_text) = ui_texts.get_mut(entity) {
                    ui_text.color = FONT_COLOUR_ACTIVE;
                }
            }

            // Set previous and next siblings
            if menu_item_entities.len() >= 2 {
                if let Some(first_item) = menu_item_entities.first().copied() {
                    let second = menu_item_entities.get(1).copied();
                    siblingses
                        .insert(first_item, Siblings::new(None, second))
                        .expect("Failed to insert `Siblings` component.");
                }
                // Skip first menu item.
                //
                // `Vec#get(n)` returns `None` when out of bounds, so the logic works for the
                // last item.
                menu_item_entities[..]
                    .iter()
                    .enumerate()
                    .skip(1)
                    .for_each(|(index, menu_item)| {
                        let prev_item = menu_item_entities.get(index - 1).copied();
                        let next_item = menu_item_entities.get(index + 1).copied();
                        siblingses
                            .insert(*menu_item, Siblings::new(prev_item, next_item))
                            .expect("Failed to insert `Siblings` component.");
                    });
            }
        }
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
