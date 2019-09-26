use amethyst::ecs::Entity;
use collision_model::loaded::{HitTransition, HittingTransition};
use map_model::play::MapUnboundedDelete;
use sequence_model::loaded::SequenceId;

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
            map_unbounded_deletes,
            hit_transitions,
            hitting_transitions,
        }: &mut EnergyComponentStorages<'s>,
    ) {
        map_unbounded_deletes
            .insert(entity, MapUnboundedDelete::default())
            .expect("Failed to insert `MapUnboundedDelete` component.");

        // Hack: this should be read off an asset.
        hit_transitions
            .insert(entity, HitTransition::new(SequenceId::new(2)))
            .expect("Failed to insert `HitTransition` component.");
        hitting_transitions
            .insert(entity, HittingTransition::new(SequenceId::new(2)))
            .expect("Failed to insert `HittingTransition` component.");
        // End Hack.
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
    use collision_model::loaded::{HitTransition, HittingTransition};
    use map_model::play::MapUnboundedDelete;

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
            .run_isolated()
    }
}
