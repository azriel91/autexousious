use amethyst::{
    ecs::{Entities, LazyUpdate, Read, System, World, WriteExpect},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use asset_model::loaded::{AssetId, AssetIdMappings};
use background_play::{LayerComponentStorages, LayerEntitySpawner, LayerSpawningResources};
use derivative::Derivative;
use derive_new::new;
use log::{error, warn};
use state_registry::{StateId, StateIdUpdateEvent, StateUiData};
use typename_derive::TypeName;

/// Spawns state UI backgrounds when the `StateId` changes.
#[derive(Debug, Default, TypeName, new)]
pub struct StateUiSpawnSystem {
    /// Reader ID for the `StateIdUpdateEvent` channel.
    #[new(default)]
    state_id_update_event_rid: Option<ReaderId<StateIdUpdateEvent>>,
}

/// `StateUiSpawnSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct StateUiSpawnSystemData<'s> {
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
    /// `LayerSpawningResources`.
    pub layer_spawning_resources: LayerSpawningResources<'s>,
    /// `LayerComponentStorages`.
    pub layer_component_storages: LayerComponentStorages<'s>,
    /// `LazyUpdate` resource.
    #[derivative(Debug = "ignore")]
    pub lazy_update: Read<'s, LazyUpdate>,
}

impl StateUiSpawnSystem {
    /// Returns the last event from the event channel.
    fn last_event(
        &mut self,
        state_id_update_ec: &EventChannel<StateIdUpdateEvent>,
    ) -> Option<StateIdUpdateEvent> {
        let state_id_update_event_rid = self
            .state_id_update_event_rid
            .as_mut()
            .expect("Expected `state_id_update_event_rid` field to be set.");

        let events_iterator = state_id_update_ec.read(state_id_update_event_rid);
        let event_count = events_iterator.len();

        if event_count > 1 {
            warn!(
                "{} state ID update events received, only processing the last event.",
                event_count
            );
        }

        // Only process the last event.
        events_iterator
            .skip(event_count.saturating_sub(1))
            .next()
            .copied()
    }

    /// Returns the State's UI collective asset ID.
    fn asset_id(asset_id_mappings: &AssetIdMappings, state_id: StateId) -> Option<AssetId> {
        let state_id_name = state_id.to_string();
        asset_id_mappings.iter().find_map(|(id, slug)| {
            if &slug.name == &state_id_name {
                Some(id)
            } else {
                None
            }
        })
    }
}

impl<'s> System<'s> for StateUiSpawnSystem {
    type SystemData = StateUiSpawnSystemData<'s>;

    fn run(
        &mut self,
        StateUiSpawnSystemData {
            entities,
            state_id_update_ec,
            mut state_ui_data,
            asset_id_mappings,
            layer_spawning_resources,
            mut layer_component_storages,
            lazy_update,
        }: Self::SystemData,
    ) {
        if let Some(ev) = self.last_event(&state_id_update_ec) {
            let state_id = ev.state_id;

            // Don't panic if there are no assets for the current `StateId`.
            if let Some(asset_id) = Self::asset_id(&asset_id_mappings, state_id) {
                let layer_entities = LayerEntitySpawner::spawn_system(
                    &layer_spawning_resources,
                    &mut layer_component_storages,
                    asset_id,
                );

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
                    state_ui_data.entities = layer_entities;
                } else {
                    lazy_update.exec_mut(move |world| {
                        world.insert(StateUiData::new(state_id, layer_entities));
                    });
                }
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
