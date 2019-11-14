use amethyst::{
    ecs::{Entities, LazyUpdate, Read, System, World, WriteExpect},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use asset_model::loaded::AssetIdMappings;
use background_play::{
    BackgroundLayerComponentStorages, BackgroundLayerEntitySpawner,
    BackgroundLayerSpawningResources,
};
use derivative::Derivative;
use derive_new::new;
use log::error;
use shrev_support::EventChannelExt;
use state_registry::{StateIdUpdateEvent, StateUiData};
use state_support::StateAssetUtils;
use typename_derive::TypeName;

/// Spawns state UI backgrounds when the `StateId` changes.
#[derive(Debug, Default, TypeName, new)]
pub struct StateBackgroundSpawnSystem {
    /// Reader ID for the `StateIdUpdateEvent` channel.
    #[new(default)]
    state_id_update_event_rid: Option<ReaderId<StateIdUpdateEvent>>,
}

/// `StateBackgroundSpawnSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct StateBackgroundSpawnSystemData<'s> {
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `StateIdUpdateEvent` channel.
    #[derivative(Debug = "ignore")]
    pub state_id_update_ec: Read<'s, EventChannel<StateIdUpdateEvent>>,
    /// `StateUiData` resource.
    #[derivative(Debug = "ignore")]
    pub state_ui_data: Option<WriteExpect<'s, StateUiData>>,
    /// `AssetIdMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_id_mappings: Read<'s, AssetIdMappings>,
    /// `BackgroundLayerSpawningResources`.
    pub background_layer_spawning_resources: BackgroundLayerSpawningResources<'s>,
    /// `BackgroundLayerComponentStorages`.
    pub background_layer_component_storages: BackgroundLayerComponentStorages<'s>,
    /// `LazyUpdate` resource.
    #[derivative(Debug = "ignore")]
    pub lazy_update: Read<'s, LazyUpdate>,
}

impl<'s> System<'s> for StateBackgroundSpawnSystem {
    type SystemData = StateBackgroundSpawnSystemData<'s>;

    fn run(
        &mut self,
        StateBackgroundSpawnSystemData {
            entities,
            state_id_update_ec,
            mut state_ui_data,
            asset_id_mappings,
            background_layer_spawning_resources,
            mut background_layer_component_storages,
            lazy_update,
        }: Self::SystemData,
    ) {
        let state_id_update_event_rid = self
            .state_id_update_event_rid
            .as_mut()
            .expect("Expected `state_id_update_event_rid` field to be set.");

        if let Some(ev) = state_id_update_ec.last_event(state_id_update_event_rid) {
            let state_id = ev.state_id;

            // Don't panic if there are no assets for the current `StateId`.
            let layer_entities =
                StateAssetUtils::asset_id(&asset_id_mappings, state_id).map(|asset_id| {
                    BackgroundLayerEntitySpawner::spawn_system(
                        &background_layer_spawning_resources,
                        &mut background_layer_component_storages,
                        asset_id,
                    )
                });

            if let Some(state_ui_data) = state_ui_data.as_mut() {
                (*state_ui_data).entities.drain(..).for_each(|entity| {
                    entities.delete(entity).unwrap_or_else(|e| {
                        let message =
                            format!("Failed to delete state UI entity: {:?}. {}", entity, e);
                        error!("{}", &message);
                        panic!("{}", &message);
                    });
                });

                state_ui_data.state_id = state_id;
                if let Some(layer_entities) = layer_entities {
                    state_ui_data.entities = layer_entities;
                }
            } else {
                lazy_update.exec_mut(move |world| {
                    world.insert(StateUiData::new(
                        state_id,
                        layer_entities.unwrap_or_else(Vec::new),
                    ));
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
