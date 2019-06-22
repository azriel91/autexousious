use amethyst::{
    assets::AssetStorage,
    ecs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage},
    shrev::EventChannel,
};
use derivative::Derivative;
use derive_new::new;
use energy_prefab::EnergyPrefabHandle;
use game_model::loaded::EnergyPrefabs;
use log::error;
use shred_derive::SystemData;
use spawn_model::{
    config::{Spawns, SpawnsHandle},
    play::SpawnEvent,
};
use typename_derive::TypeName;

/// Spawns `GameObject`s.
#[derive(Debug, Default, TypeName, new)]
pub struct SpawnGameObjectSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SpawnGameObjectSystemData<'s> {
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `SpawnsHandle` components.
    #[derivative(Debug = "ignore")]
    pub spawns_handles: ReadStorage<'s, SpawnsHandle>,
    /// `Spawns` assets.
    #[derivative(Debug = "ignore")]
    pub spawns_assets: Read<'s, AssetStorage<Spawns>>,
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

impl<'s> System<'s> for SpawnGameObjectSystem {
    type SystemData = SpawnGameObjectSystemData<'s>;

    fn run(
        &mut self,
        SpawnGameObjectSystemData {
            entities,
            spawns_handles,
            spawns_assets,
            energy_prefabs,
            mut energy_prefab_handles,
            mut spawn_ec,
        }: Self::SystemData,
    ) {
        (&entities, &spawns_handles)
            .join()
            .for_each(|(entity_parent, spawns_handle)| {
                let spawns = spawns_assets
                    .get(spawns_handle)
                    .expect("Expected `Spawns` to be loaded.");
                spawns.iter().for_each(|spawn| {
                    if let Some(energy_prefab_handle) = energy_prefabs.get(&spawn.object) {
                        let entity_spawned = entities.create();
                        energy_prefab_handles
                            .insert(entity_spawned, energy_prefab_handle.clone())
                            .expect("Failed to insert `EnergyPrefabHandle` component.");

                        let spawn_event =
                            SpawnEvent::new(spawn.clone(), entity_parent, entity_spawned);
                        spawn_ec.single_write(spawn_event);
                    } else {
                        error!(
                            "`{}` does not exist in loaded `Energy` objects.",
                            &spawn.object
                        )
                    }
                });
            });
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use amethyst::{
        assets::{AssetStorage, Loader},
        ecs::{Builder, Entity, Join, Read, ReadExpect, World},
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
    use spawn_model::{
        config::{Spawn, Spawns},
        play::SpawnEvent,
    };
    use typename::TypeName;

    use super::SpawnGameObjectSystem;

    #[test]
    fn spawns_entity_for_each_spawn() -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_system(
                SpawnGameObjectSystem::new(),
                &SpawnGameObjectSystem::type_name(),
                &[&ObjectAssetLoadingSystem::<
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
            .with_setup(create_entity_with_spawns)
            .with_assertion(|world| {
                assert_energy_count(world, 1);
                assert_events(world, 1);
            })
            .run()
    }

    fn setup_spawn_ec_reader(world: &mut World) {
        let spawn_event_rid = world
            .write_resource::<EventChannel<SpawnEvent>>()
            .register_reader(); // kcov-ignore

        world.add_resource(spawn_event_rid);
    }

    fn create_entity_with_spawns(world: &mut World) {
        let spawns_handle = {
            let (loader, spawns_assets) =
                world.system_data::<(ReadExpect<'_, Loader>, Read<'_, AssetStorage<Spawns>>)>();
            loader.load_from_data(Spawns::new(vec![spawn()]), (), &spawns_assets)
        };

        let entity = world.create_entity().with(spawns_handle).build();
        world.add_resource(entity);
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
