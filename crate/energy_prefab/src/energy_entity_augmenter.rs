use amethyst::ecs::Entity;
use map_model::play::MapUnboundedDelete;
use object_model::play::Grounding;

use crate::EnergyComponentStorages;

/// Augments an entity with `Energy` components.
#[derive(Debug)]
pub struct EnergyEntityAugmenter;

impl EnergyEntityAugmenter {
    /// Augments an entity with `Energy` components.
    ///
    /// # Parameters
    ///
    /// * `entity`: The entity to augment.
    /// * `energy_component_storages`: Energy specific `Component` storages.
    pub fn augment<'s>(
        entity: Entity,
        EnergyComponentStorages {
            ref mut groundings,
            ref mut map_unbounded_deletes,
        }: &mut EnergyComponentStorages<'s>,
    ) {
        groundings
            .insert(entity, Grounding::Airborne)
            .expect("Failed to insert `Grounding` component.");
        map_unbounded_deletes
            .insert(entity, MapUnboundedDelete::default())
            .expect("Failed to insert `MapUnboundedDelete` component.");
    }
}

#[cfg(test)]
mod test {
    use amethyst::{
        core::TransformBundle,
        ecs::{Builder, World, WorldExt},
        renderer::{types::DefaultBackend, RenderEmptyBundle},
        shred::SystemData,
        Error,
    };
    use amethyst_test::AmethystApplication;
    use map_model::play::MapUnboundedDelete;
    use object_model::play::Grounding;

    use super::EnergyEntityAugmenter;
    use crate::EnergyComponentStorages;

    #[test]
    fn augments_entity_with_energy_components() -> Result<(), Error> {
        let assertion = |world: &mut World| {
            let entity = world.create_entity().build();
            {
                let mut energy_component_storages = EnergyComponentStorages::fetch(&world);
                EnergyEntityAugmenter::augment(entity, &mut energy_component_storages);
            }

            assert!(world.read_storage::<Grounding>().contains(entity));
            assert!(world.read_storage::<MapUnboundedDelete>().contains(entity));
        };

        AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
            .with_effect(|world| {
                <EnergyComponentStorages as SystemData>::setup(world);
            })
            .with_assertion(assertion)
            .run_isolated()
    }
}
