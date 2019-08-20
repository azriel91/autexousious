use amethyst::{
    assets::AssetStorage,
    ecs::{Entities, Entity, Read, ReadStorage, System, World, WorldExt, Write, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use energy_prefab::EnergyPrefabHandle;
use game_model::loaded::EnergyPrefabs;
use log::error;
use sequence_model::play::SequenceUpdateEvent;
use spawn_model::{
    config::{Spawns, SpawnsHandle},
    play::SpawnEvent,
};
use typename_derive::TypeName;

/// Spawns `GameObject`s. Currently only supports `Energy` objects.
#[derive(Debug, Default, TypeName, new)]
pub struct SpawnGameObjectSystem {
    /// Reader ID for the `SequenceUpdateEvent` event channel.
    #[new(default)]
    reader_id: Option<ReaderId<SequenceUpdateEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SpawnGameObjectResources<'s> {
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `EnergyPrefabs` resource.
    #[derivative(Debug = "ignore")]
    pub energy_prefabs: Read<'s, EnergyPrefabs>,
    /// `EnergyPrefabHandle` components.
    #[derivative(Debug = "ignore")]
    pub energy_prefab_handles: WriteStorage<'s, EnergyPrefabHandle>,
    /// `SpawnEvent` channel.
    #[derivative(Debug = "ignore")]
    pub spawn_ec: Write<'s, EventChannel<SpawnEvent>>,
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
        SpawnGameObjectResources {
            ref entities,
            ref energy_prefabs,
            ref mut energy_prefab_handles,
            ref mut spawn_ec,
        }: &mut SpawnGameObjectResources<'_>,
        spawns: &Spawns,
        entity_parent: Entity,
    ) {
        spawns.iter().for_each(|spawn| {
            if let Some(energy_prefab_handle) = energy_prefabs.get(&spawn.object) {
                let entity_spawned = entities.create();
                energy_prefab_handles
                    .insert(entity_spawned, energy_prefab_handle.clone())
                    .expect("Failed to insert `EnergyPrefabHandle` component.");

                let spawn_event = SpawnEvent::new(spawn.clone(), entity_parent, entity_spawned);
                spawn_ec.single_write(spawn_event);
            } else {
                error!(
                    "`{}` does not exist in loaded `Energy` objects.",
                    &spawn.object
                )
            }
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
                if let SequenceUpdateEvent::SequenceBegin { .. }
                | SequenceUpdateEvent::FrameBegin { .. } = ev
                {
                    true
                } else {
                    false
                }
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use amethyst::{
        assets::{AssetStorage, Loader},
        ecs::{Builder, Entity, Join, Read, ReadExpect, World, WorldExt},
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use application_test_support::AutexousiousApplication;
    use asset_model::config::AssetSlug;
    use energy_loading::EnergyLoadingStatus;
    use energy_model::loaded::Energy;
    use energy_prefab::{EnergyPrefab, EnergyPrefabHandle};
    use kinematic_model::config::{Position, Velocity};
    use loading::ObjectAssetLoadingSystem;
    use sequence_model::play::SequenceUpdateEvent;
    use spawn_model::{
        config::{Spawn, Spawns},
        play::SpawnEvent,
    };
    use typename::TypeName;

    use super::SpawnGameObjectSystem;

    #[test]
    fn spawns_entity_for_sequence_begin_events() -> Result<(), Error> {
        run_test(
            Some(|entity| SequenceUpdateEvent::SequenceBegin { entity }),
            1,
        )
    }

    #[test]
    fn spawns_entity_for_frame_begin_events() -> Result<(), Error> {
        run_test(
            Some(|entity| SequenceUpdateEvent::FrameBegin {
                entity,
                frame_index: 0,
            }),
            1,
        )
    }

    #[test]
    fn does_not_spawn_entity_for_sequence_end_events() -> Result<(), Error> {
        run_test(
            Some(|entity| SequenceUpdateEvent::SequenceEnd {
                entity,
                frame_index: 0,
            }),
            0,
        )
    }

    #[test]
    fn does_not_spawn_entity_when_no_sequence_update_event() -> Result<(), Error> {
        run_test(None, 0)
    }

    fn run_test(
        sequence_update_event_fn: Option<fn(Entity) -> SequenceUpdateEvent>,
        spawn_count_expected: usize,
    ) -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_system(
                SpawnGameObjectSystem::new(),
                SpawnGameObjectSystem::type_name(),
                &[ObjectAssetLoadingSystem::<
                    Energy,
                    EnergyPrefab,
                    EnergyLoadingStatus,
                >::type_name()],
            )
            // TODO: Split prefab system.
            // TODO: <https://gitlab.com/azriel91/autexousious/issues/138>
            // .with_bundle(EnergyPrefabBundle::new().with_system_dependencies(&[
            //     String::from(ENERGY_PROCESSOR),
            //     SpawnGameObjectSystem::type_name(),
            // ]))
            .with_setup(setup_spawn_ec_reader)
            .with_assertion(|world| {
                assert_energy_count(world, 0);
                assert_events(world, 0);
            })
            .with_effect(move |world| create_entity_with_spawns(world, sequence_update_event_fn))
            .with_assertion(move |world| {
                assert_energy_count(world, spawn_count_expected);
                assert_events(world, spawn_count_expected);
            })
            .run_isolated()
    }

    fn setup_spawn_ec_reader(world: &mut World) {
        let spawn_event_rid = world
            .write_resource::<EventChannel<SpawnEvent>>()
            .register_reader(); // kcov-ignore

        world.insert(spawn_event_rid);
    }

    fn create_entity_with_spawns(
        world: &mut World,
        sequence_update_event_fn: Option<fn(Entity) -> SequenceUpdateEvent>,
    ) {
        let spawns_handle = {
            let (loader, spawns_assets) =
                world.system_data::<(ReadExpect<'_, Loader>, Read<'_, AssetStorage<Spawns>>)>();
            loader.load_from_data(Spawns::new(vec![spawn()]), (), &spawns_assets)
        };

        let entity = world.create_entity().with(spawns_handle).build();
        world.insert(entity);

        if let Some(sequence_update_event_fn) = sequence_update_event_fn {
            let mut sequence_update_ec =
                world.write_resource::<EventChannel<SequenceUpdateEvent>>();
            let sequence_update_event = sequence_update_event_fn(entity);
            sequence_update_ec.single_write(sequence_update_event);
        }
    }

    fn spawn() -> Spawn {
        Spawn::new(
            AssetSlug::from_str("test/square")
                .expect("Expected `test/square` to be a valid asset slug."),
            Position::<i32>::from((0, 0, 0)),
            Velocity::<i32>::from((0, 0, 0)),
        )
    }

    fn assert_energy_count(world: &mut World, count: usize) {
        let energy_prefab_handles = world.read_storage::<EnergyPrefabHandle>();
        assert_eq!(count, (&energy_prefab_handles).join().count());
    }

    fn assert_events(world: &mut World, event_count: usize) {
        let mut spawn_event_rid = &mut world.write_resource::<ReaderId<SpawnEvent>>();

        let spawn_ec = world.read_resource::<EventChannel<SpawnEvent>>();
        let actual_events = spawn_ec
            .read(&mut spawn_event_rid)
            .collect::<Vec<&SpawnEvent>>();

        assert_eq!(event_count, actual_events.len());

        if event_count > 0 {
            let spawn_expected = spawn();
            let entity_parent = *world.read_resource::<Entity>();
            actual_events.into_iter().for_each(|ev| {
                assert_eq!(&spawn_expected, &ev.spawn);
                assert_eq!(entity_parent, ev.entity_parent);
            });
        }
    }
}
