use amethyst::{
    assets::AssetStorage,
    ecs::{Entities, Entity, Read, ReadStorage, System, World, Write},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use asset_model::{
    config::AssetType,
    loaded::{AssetIdMappings, AssetTypeMappings},
};
use character_prefab::{
    CharacterComponentStorages, CharacterEntityAugmenter, CharacterSpawningResources,
};
use derivative::Derivative;
use derive_new::new;
use energy_prefab::{EnergyComponentStorages, EnergyEntityAugmenter};
use log::error;
use object_prefab::{ObjectComponentStorages, ObjectEntityAugmenter, ObjectSpawningResources};
use object_type::ObjectType;
use sequence_model::play::SequenceUpdateEvent;
use spawn_model::{
    loaded::{Spawns, SpawnsHandle},
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
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `AssetIdMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_id_mappings: Read<'s, AssetIdMappings>,
    /// `AssetTypeMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_type_mappings: Read<'s, AssetTypeMappings>,
    /// `ObjectSpawningResources`.
    #[derivative(Debug = "ignore")]
    pub object_spawning_resources: ObjectSpawningResources<'s>,
    /// `ObjectComponentStorages`.
    #[derivative(Debug = "ignore")]
    pub object_component_storages: ObjectComponentStorages<'s>,
    /// `CharacterSpawningResources`.
    #[derivative(Debug = "ignore")]
    pub character_spawning_resources: CharacterSpawningResources<'s>,
    /// `CharacterComponentStorages`.
    #[derivative(Debug = "ignore")]
    pub character_component_storages: CharacterComponentStorages<'s>,
    /// `EnergyComponentStorages`.
    #[derivative(Debug = "ignore")]
    pub energy_component_storages: EnergyComponentStorages<'s>,
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
            entities,
            asset_id_mappings,
            asset_type_mappings,
            object_spawning_resources,
            object_component_storages,
            character_spawning_resources,
            character_component_storages,
            energy_component_storages,
            spawn_ec,
        }: &mut SpawnGameObjectResources<'_>,
        spawns: &Spawns,
        entity_parent: Entity,
    ) {
        spawns.iter().for_each(|spawn| {
            let asset_id = spawn.object;
            let asset_type = asset_type_mappings
                .get(&asset_id)
                .unwrap_or_else(|| panic!("`AssetType` not found for `{:?}`.", asset_id));
            let entity_spawned = entities.create();

            ObjectEntityAugmenter::augment(
                object_spawning_resources,
                object_component_storages,
                asset_id,
                entity_spawned,
            );

            match asset_type {
                AssetType::Object(ObjectType::Character) => {
                    CharacterEntityAugmenter::augment(
                        character_spawning_resources,
                        character_component_storages,
                        asset_id,
                        entity_spawned,
                    );
                }
                AssetType::Object(ObjectType::Energy) => {
                    EnergyEntityAugmenter::augment(entity_spawned, energy_component_storages);
                }
                _ => {
                    let asset_slug = asset_id_mappings
                        .slug(asset_id)
                        .unwrap_or_else(|| panic!("`AssetSlug` not found for `{:?}`.", asset_id));
                    error!(
                        "Spawning of asset type `{:?}` (`{}`) is not supported.",
                        asset_type, asset_slug
                    );
                }
            }

            let spawn_event =
                SpawnEvent::new(spawn.clone(), entity_parent, entity_spawned, asset_id);
            spawn_ec.single_write(spawn_event);
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
    use amethyst::{
        assets::{AssetStorage, Loader},
        ecs::{Builder, Entity, Read, ReadExpect, World, WorldExt},
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use application_test_support::{AssetQueries, AutexousiousApplication};
    use assets_test::{CHAR_BAT_SLUG, ENERGY_SQUARE_SLUG};
    use kinematic_model::config::{Position, Velocity};
    use sequence_model::{loaded::SequenceId, play::SequenceUpdateEvent};
    use spawn_model::{
        loaded::{Spawn, Spawns},
        play::SpawnEvent,
    };
    use typename::TypeName;

    use super::SpawnGameObjectSystem;

    #[test]
    fn spawns_entity_for_sequence_begin_events() -> Result<(), Error> {
        run_test(
            Some(|entity| SequenceUpdateEvent::SequenceBegin {
                entity,
                sequence_id: SequenceId::new(0),
            }),
            2,
        )
    }

    #[test]
    fn spawns_entity_for_frame_begin_events() -> Result<(), Error> {
        run_test(
            Some(|entity| SequenceUpdateEvent::FrameBegin {
                entity,
                frame_index: 0,
            }),
            2,
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
                &[],
            )
            .with_effect(setup_spawn_ec_reader)
            .with_assertion(|world| {
                assert_object_count(world, 0);
                assert_events(world, 0);
            })
            .with_effect(move |world| create_entity_with_spawns(world, sequence_update_event_fn))
            .with_assertion(move |world| {
                assert_object_count(world, spawn_count_expected);
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
            loader.load_from_data(
                Spawns::new(vec![spawn_character(world), spawn_energy(world)]),
                (),
                &spawns_assets,
            )
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

    fn spawn_character(world: &World) -> Spawn {
        let asset_id = AssetQueries::id(world, &*CHAR_BAT_SLUG);
        Spawn::new(
            asset_id,
            Position::<f32>::new(0., 0., 0.),
            Velocity::<f32>::new(0., 0., 0.),
            SequenceId::new(0),
        )
    }

    fn spawn_energy(world: &World) -> Spawn {
        let asset_id = AssetQueries::id(world, &*ENERGY_SQUARE_SLUG);
        Spawn::new(
            asset_id,
            Position::<f32>::new(0., 0., 0.),
            Velocity::<f32>::new(0., 0., 0.),
            SequenceId::new(0),
        )
    }

    fn assert_object_count(world: &mut World, count: usize) {
        let positions = world.read_storage::<Position<f32>>();
        assert_eq!(count, positions.count());
    }

    fn assert_events(world: &mut World, event_count: usize) {
        let mut spawn_event_rid = &mut world.write_resource::<ReaderId<SpawnEvent>>();

        let spawn_ec = world.read_resource::<EventChannel<SpawnEvent>>();
        let actual_events = spawn_ec
            .read(&mut spawn_event_rid)
            .collect::<Vec<&SpawnEvent>>();

        assert_eq!(event_count, actual_events.len());

        if event_count > 0 {
            let spawns_expected = vec![spawn_character(world), spawn_energy(world)];
            let entity_parent = *world.read_resource::<Entity>();
            spawns_expected
                .into_iter()
                .zip(actual_events.into_iter())
                .for_each(|(spawn_expected, ev)| {
                    assert_eq!(&spawn_expected, &ev.spawn);
                    assert_eq!(entity_parent, ev.entity_parent);
                });
        }
    }
}
