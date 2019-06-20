use amethyst::{
    assets::AssetStorage,
    ecs::{Entities, Join, Read, ReadStorage, System, WriteStorage},
};
use derivative::Derivative;
use derive_new::new;
use energy_prefab::EnergyPrefabHandle;
use game_model::loaded::EnergyPrefabs;
use log::error;
use object_model::play::ParentObject;
use shred_derive::SystemData;
use spawn_model::config::{Spawns, SpawnsHandle};
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
    /// `ParentObject` components.
    #[derivative(Debug = "ignore")]
    pub parent_objects: WriteStorage<'s, ParentObject>,
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
            mut parent_objects,
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
                        parent_objects
                            .insert(entity_spawned, ParentObject::new(entity_parent))
                            .expect("Failed to insert `ParentObject` component.");
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
        ecs::{Builder, Join, Read, ReadExpect, World},
        Error,
    };
    use application_test_support::AutexousiousApplication;
    use asset_model::config::AssetSlug;
    use energy_loading::EnergyLoadingStatus;
    use energy_model::loaded::Energy;
    use energy_prefab::{EnergyPrefab, EnergyPrefabHandle};
    use kinematic_model::config::{Position, Velocity};
    use loading::ObjectAssetLoadingSystem;
    use spawn_model::config::{Spawn, Spawns};
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
            .with_assertion(|world| assert_energy_count(world, 0))
            .with_setup(create_entity_with_spawns)
            .with_assertion(|world| assert_energy_count(world, 1))
            .run()
    }

    fn create_entity_with_spawns(world: &mut World) {
        let spawns_handle = {
            let (loader, spawns_assets) =
                world.system_data::<(ReadExpect<'_, Loader>, Read<'_, AssetStorage<Spawns>>)>();
            loader.load_from_data(
                Spawns::new(vec![Spawn::new(
                    AssetSlug::from_str("test/square")
                        .expect("Expected `test/square` to be a valid asset slug."),
                    Position::<i32>::from((0, 0, 0)),
                    Velocity::<i32>::from((0, 0, 0)),
                )]),
                (),
                &spawns_assets,
            )
        };

        let entity = world.create_entity().with(spawns_handle).build();
        world.add_resource(entity);
    }

    fn assert_energy_count(world: &mut World, count: usize) {
        let energy_prefab_handles = world.read_storage::<EnergyPrefabHandle>();
        assert_eq!(count, (&energy_prefab_handles).join().count());
    }
}
