use amethyst::{
    ecs::{Entities, LazyUpdate, Read, System, World, WriteExpect},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use asset_model::loaded::AssetIdMappings;
use derivative::Derivative;
use derive_new::new;
use log::error;
use shrev_support::EventChannelExt;
use state_registry::{StateIdUpdateEvent, StateUiData};
use state_support::StateAssetUtils;
use typename_derive::TypeName;
use ui_label_play::{
    UiSpriteLabelComponentStorages, UiSpriteLabelEntitySpawner, UiSpriteLabelSpawningResources,
};

/// Spawns state `UiSpriteLabel`s when the `StateId` changes.
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
    /// `UiSpriteLabelSpawningResources`.
    pub ui_sprite_label_spawning_resources: UiSpriteLabelSpawningResources<'s>,
    /// `UiSpriteLabelComponentStorages`.
    pub ui_sprite_label_component_storages: UiSpriteLabelComponentStorages<'s>,
    /// `LazyUpdate` resource.
    #[derivative(Debug = "ignore")]
    pub lazy_update: Read<'s, LazyUpdate>,
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
            ui_sprite_label_spawning_resources,
            mut ui_sprite_label_component_storages,
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
            let ui_sprite_label_entities = StateAssetUtils::asset_id(&asset_id_mappings, state_id)
                .map(|asset_id| {
                    UiSpriteLabelEntitySpawner::spawn_system(
                        &ui_sprite_label_spawning_resources,
                        &mut ui_sprite_label_component_storages,
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
                if let Some(ui_sprite_label_entities) = ui_sprite_label_entities {
                    state_ui_data.entities = ui_sprite_label_entities;
                }
            } else {
                lazy_update.exec_mut(move |world| {
                    world.insert(StateUiData::new(
                        state_id,
                        ui_sprite_label_entities.unwrap_or_else(Vec::new),
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
