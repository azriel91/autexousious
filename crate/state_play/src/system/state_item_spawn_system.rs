use amethyst::{
    ecs::{Entities, Entity, Read, System, World, Write, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use asset_model::loaded::{AssetId, AssetIdMappings, AssetItemIds, ItemId};
use derivative::Derivative;
use derive_new::new;
use log::error;
use shrev_support::EventChannelExt;
use state_registry::{StateIdUpdateEvent, StateItemEntities};
use state_support::StateAssetUtils;
use typename_derive::TypeName;

/// Spawns entities for each `ItemId` for the state `AssetId` when the `StateId` changes.
#[derive(Debug, Default, TypeName, new)]
pub struct StateItemSpawnSystem {
    /// Reader ID for the `StateIdUpdateEvent` channel.
    #[new(default)]
    state_id_update_event_rid: Option<ReaderId<StateIdUpdateEvent>>,
}

/// `StateItemSpawnSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct StateItemSpawnSystemData<'s> {
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `StateIdUpdateEvent` channel.
    #[derivative(Debug = "ignore")]
    pub state_id_update_ec: Read<'s, EventChannel<StateIdUpdateEvent>>,
    /// `StateItemEntities` resource.
    #[derivative(Debug = "ignore")]
    pub state_item_entities: Write<'s, StateItemEntities>,
    /// `AssetIdMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_id_mappings: Read<'s, AssetIdMappings>,
    /// `AssetItemIds` resource.
    #[derivative(Debug = "ignore")]
    pub asset_item_ids: Read<'s, AssetItemIds>,
    /// `AssetId` components.
    #[derivative(Debug = "ignore")]
    pub asset_ids: WriteStorage<'s, AssetId>,
    /// `ItemId` components.
    #[derivative(Debug = "ignore")]
    pub item_ids: WriteStorage<'s, ItemId>,
}

impl<'s> System<'s> for StateItemSpawnSystem {
    type SystemData = StateItemSpawnSystemData<'s>;

    fn run(
        &mut self,
        StateItemSpawnSystemData {
            entities,
            state_id_update_ec,
            mut state_item_entities,
            asset_id_mappings,
            asset_item_ids,
            mut asset_ids,
            mut item_ids,
        }: Self::SystemData,
    ) {
        let state_id_update_event_rid = self
            .state_id_update_event_rid
            .as_mut()
            .expect("Expected `state_id_update_event_rid` field to be set.");

        if let Some(ev) = state_id_update_ec.last_event(state_id_update_event_rid) {
            let state_id = ev.state_id;

            (*state_item_entities)
                .entities
                .drain(..)
                .for_each(|entity| {
                    entities.delete(entity).unwrap_or_else(|e| {
                        let message =
                            format!("Failed to delete state item entity: {:?}. {}", entity, e);
                        error!("{}", &message);
                        panic!("{}", &message);
                    });
                });

            state_item_entities.state_id = state_id;

            // Don't panic if there are no assets for the current `StateId`.
            let asset_id = StateAssetUtils::asset_id(&asset_id_mappings, state_id);
            let items = asset_id.and_then(|asset_id| asset_item_ids.get(asset_id));
            if let (Some(asset_id), Some(items)) = (asset_id, items) {
                let item_entities = items
                    .iter()
                    .copied()
                    .map(|item_id| {
                        entities
                            .build_entity()
                            .with(asset_id, &mut asset_ids)
                            .with(item_id, &mut item_ids)
                            .build()
                    })
                    .collect::<Vec<Entity>>();

                state_item_entities.entities = item_entities;
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
