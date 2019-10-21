#[cfg(test)]
mod tests {
    use amethyst::{
        core::TransformBundle,
        ecs::{Builder, World, WorldExt},
        renderer::{types::DefaultBackend, RenderEmptyBundle},
        shred::SystemData,
        Error,
    };
    use amethyst_test::AmethystApplication;
    use collision_model::loaded::{HitTransition, HittingTransition};
    use map_model::play::MapUnboundedDelete;

    use energy_prefab::{EnergyComponentStorages, EnergyEntityAugmenter};

    #[test]
    fn augments_entity_with_energy_components() -> Result<(), Error> {
        let assertion = |world: &mut World| {
            let entity = world.create_entity().build();
            {
                let mut energy_component_storages = EnergyComponentStorages::fetch(&world);
                EnergyEntityAugmenter::augment(entity, &mut energy_component_storages);
            }

            assert!(world.read_storage::<MapUnboundedDelete>().contains(entity));
            assert!(world.read_storage::<HitTransition>().contains(entity));
            assert!(world.read_storage::<HittingTransition>().contains(entity));
        };

        AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
            .with_effect(|world| {
                <EnergyComponentStorages as SystemData>::setup(world);
            })
            .with_assertion(assertion)
            .run()
    }
}
