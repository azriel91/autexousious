use std::{fmt::Debug, marker::PhantomData};

use amethyst::{
    ecs::{Read, System, World, WorldExt},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use asset_model::{
    play::{AssetWorld, ItemIdEvent},
    ItemComponent,
};
use derivative::Derivative;
use derive_new::new;
use typename_derive::TypeName;

/// Augments an entity with components based on the `ItemId` attached to it.
///
/// # Type Parameters
///
/// * `IC`: Type of item component data, e.g. `WaitSequenceHandles`.
#[derive(Debug, Default, TypeName, new)]
pub struct ItemComponentComponentAugmentSystem<'s, IC>
where
    IC: ItemComponent<'s> + Debug,
{
    /// Reader ID for the `ItemIdEvent` event channel.
    #[new(default)]
    item_id_rid: Option<ReaderId<ItemIdEvent>>,
    /// Marker.
    phantom_data: PhantomData<&'s IC>,
}

/// `ItemComponentComponentAugmentSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ItemComponentComponentAugmentSystemData<'s, IC>
where
    IC: ItemComponent<'s> + Debug,
{
    /// Event channel for `ItemIdEvent`s.
    #[derivative(Debug = "ignore")]
    pub item_id_ec: Read<'s, EventChannel<ItemIdEvent>>,
    /// `AssetWorld` resource.
    #[derivative(Debug = "ignore")]
    pub asset_world: Read<'s, AssetWorld>,
    /// Resources and storages to augment the entity.
    #[derivative(Debug = "ignore")]
    pub item_component_system_data: <IC as ItemComponent<'s>>::SystemData,
}

impl<'s, IC> System<'s> for ItemComponentComponentAugmentSystem<'s, IC>
where
    IC: ItemComponent<'s> + Debug,
{
    type SystemData = ItemComponentComponentAugmentSystemData<'s, IC>;

    fn run(
        &mut self,
        ItemComponentComponentAugmentSystemData {
            item_id_ec,
            asset_world,
            mut item_component_system_data,
        }: Self::SystemData,
    ) {
        item_id_ec
            .read(
                self.item_id_rid.as_mut().expect(
                    "Expected `item_id_rid` to exist for ItemComponentComponentAugmentSystem.",
                ),
            )
            .for_each(|ev| {
                let ItemIdEvent::CreateOrUpdate { entity, item_id } = *ev;
                let item_components = asset_world.read_storage::<IC>();
                let item_component = item_components.get(item_id.0).unwrap_or_else(|| {
                    panic!(
                        "Expected storage to exist for `ItemComponent`: `{}`",
                        std::any::type_name::<IC>()
                    )
                });
                item_component.augment(&mut item_component_system_data, entity);
            });
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.item_id_rid = Some(
            world
                .fetch_mut::<EventChannel<ItemIdEvent>>()
                .register_reader(),
        );
    }
}
