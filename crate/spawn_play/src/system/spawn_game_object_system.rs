use amethyst::{
    assets::AssetStorage,
    ecs::{Entity, Read, ReadStorage, System, World},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use sequence_model::play::SequenceUpdateEvent;
use spawn_model::loaded::{Spawns, SpawnsHandle};

use crate::{GameObjectSpawner, SpawnGameObjectResources};

/// Spawns `GameObject`s. Currently only supports `Energy` objects.
#[derive(Debug, Default, new)]
pub struct SpawnGameObjectSystem {
    /// Reader ID for the `SequenceUpdateEvent` event channel.
    #[new(default)]
    reader_id: Option<ReaderId<SequenceUpdateEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SpawnGameObjectSystemData<'s> {
    /// `SpawnGameObjectResources`.
    pub spawn_game_object_resources: SpawnGameObjectResources<'s>,
    /// Event channel for `SequenceUpdateEvent`s.
    #[derivative(Debug = "ignore")]
    pub sequence_update_ec: Read<'s, EventChannel<SequenceUpdateEvent>>,
    /// `SpawnsHandle` components.
    #[derivative(Debug = "ignore")]
    pub spawns_handles: ReadStorage<'s, SpawnsHandle>,
    /// `Spawns` assets.
    #[derivative(Debug = "ignore")]
    pub spawns_assets: Read<'s, AssetStorage<Spawns>>,
}

impl SpawnGameObjectSystem {
    /// Creates an entity for each `Spawn` and attaches its prefab handle.
    fn spawn_game_objects(
        spawn_game_object_resources: &mut SpawnGameObjectResources<'_>,
        spawns: &Spawns,
        entity_parent: Entity,
    ) {
        spawns.iter().for_each(|spawn| {
            GameObjectSpawner::spawn(spawn_game_object_resources, entity_parent, spawn);
        });
    }
}

impl<'s> System<'s> for SpawnGameObjectSystem {
    type SystemData = SpawnGameObjectSystemData<'s>;

    fn run(
        &mut self,
        SpawnGameObjectSystemData {
            mut spawn_game_object_resources,
            sequence_update_ec,
            spawns_handles,
            spawns_assets,
        }: Self::SystemData,
    ) {
        sequence_update_ec
            .read(
                self.reader_id
                    .as_mut()
                    .expect("Expected reader ID to exist for FrameComponentUpdateSystem."),
            )
            .filter(|ev| {
                matches!(
                    ev,
                    SequenceUpdateEvent::SequenceBegin { .. }
                        | SequenceUpdateEvent::FrameBegin { .. }
                )
            })
            .for_each(|ev| {
                let entity_parent = ev.entity();
                let spawns_handle = spawns_handles.get(entity_parent);

                // Some entities will have sequence update events, but not a spawns handle
                // component.
                if let Some(spawns_handle) = spawns_handle {
                    let spawns = spawns_assets
                        .get(spawns_handle)
                        .expect("Expected `Spawns` to be loaded.");

                    Self::spawn_game_objects(
                        &mut spawn_game_object_resources,
                        spawns,
                        entity_parent,
                    );
                }
            });
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.reader_id = Some(
            world
                .fetch_mut::<EventChannel<SequenceUpdateEvent>>()
                .register_reader(),
        );
    }
}
