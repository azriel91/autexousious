use amethyst::{
    ecs::{
        storage::ComponentEvent, BitSet, Entities, Join, ReadStorage, ReaderId, System, World,
        Write, WriteStorage,
    },
    shred::{ResourceId, SystemData},
    shrev::EventChannel,
};
use asset_model::{loaded::ItemId, play::ItemIdEvent};
use derivative::Derivative;
use derive_new::new;
use typename_derive::TypeName;

/// Sends `ItemIdEvent`s when `ItemId` changes.
#[derive(Debug, Default, TypeName, new)]
pub struct ItemIdEventSystem {
    /// Reader ID for item ID changes.
    #[new(default)]
    item_id_rid: Option<ReaderId<ComponentEvent>>,
    /// Pre-allocated bitset to track insertions and modifications to `ItemId`s.
    #[new(default)]
    item_id_updates: BitSet,
}

/// `ItemIdEventSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ItemIdEventSystemData<'s> {
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `ItemId` components.
    #[derivative(Debug = "ignore")]
    pub item_ids: ReadStorage<'s, ItemId>,
    /// Event channel for `ItemIdEvent`s.
    #[derivative(Debug = "ignore")]
    pub item_id_ec: Write<'s, EventChannel<ItemIdEvent>>,
}

impl<'s> System<'s> for ItemIdEventSystem {
    type SystemData = ItemIdEventSystemData<'s>;

    fn run(
        &mut self,
        ItemIdEventSystemData {
            entities,
            item_ids,
            mut item_id_ec,
        }: Self::SystemData,
    ) {
        self.item_id_updates.clear();

        item_ids
            .channel()
            .read(
                self.item_id_rid
                    .as_mut()
                    .expect("Expected `item_id_rid` to be set."),
            )
            .for_each(|event| match event {
                ComponentEvent::Inserted(id) | ComponentEvent::Modified(id) => {
                    self.item_id_updates.add(*id);
                }
                ComponentEvent::Removed(_id) => {}
            });

        (&entities, &item_ids, &self.item_id_updates)
            .join()
            .for_each(|(entity, item_id, _)| {
                item_id_ec.single_write(ItemIdEvent::CreateOrUpdate {
                    entity,
                    item_id: *item_id,
                });
            });
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.item_id_rid = Some(WriteStorage::<'_, ItemId>::fetch(world).register_reader());
    }
}
