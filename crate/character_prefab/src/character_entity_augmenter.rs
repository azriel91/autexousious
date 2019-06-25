use amethyst::ecs::Entity;
use character_model::play::RunCounter;
use game_input::ControllerInput;
use object_model::{
    config::Mass,
    play::{Grounding, HealthPoints},
};
use object_status_model::config::StunPoints;

use crate::CharacterComponentStorages;

/// Default `Character` `Mass`.
const CHARACTER_MASS_DEFAULT: Mass = Mass(0.7);

/// Augments an entity with `Character` components.
#[derive(Debug)]
pub struct CharacterEntityAugmenter;

impl CharacterEntityAugmenter {
    /// Augments an entity with `Character` components.
    ///
    /// # Parameters
    ///
    /// * `entity`: The entity to augment.
    /// * `character_component_storages`: Character specific `Component` storages.
    pub fn augment<'s>(
        entity: Entity,
        CharacterComponentStorages {
            ref mut controller_inputs,
            ref mut health_pointses,
            ref mut stun_pointses,
            ref mut run_counters,
            ref mut groundings,
            ref mut masses,
        }: &mut CharacterComponentStorages<'s>,
    ) {
        // Controller of this entity
        controller_inputs
            .insert(entity, ControllerInput::default())
            .expect("Failed to insert controller_input component.");
        health_pointses
            .insert(entity, HealthPoints::default())
            .expect("Failed to insert health_points component.");
        stun_pointses
            .insert(entity, StunPoints::default())
            .expect("Failed to insert stun_points component.");
        run_counters
            .insert(entity, RunCounter::default())
            .expect("Failed to insert run_counter component.");
        groundings
            .insert(entity, Grounding::default())
            .expect("Failed to insert grounding component.");
        masses
            .insert(entity, CHARACTER_MASS_DEFAULT)
            .expect("Failed to insert grounding component.");
    }
}

#[cfg(test)]
mod test {
    use amethyst::{
        core::TransformBundle,
        ecs::{Builder, SystemData, World},
        renderer::{types::DefaultBackend, RenderEmptyBundle},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use character_model::play::RunCounter;
    use game_input::ControllerInput;
    use object_model::{
        config::Mass,
        play::{Grounding, HealthPoints},
    };
    use object_status_model::config::StunPoints;

    use super::CharacterEntityAugmenter;
    use crate::CharacterComponentStorages;

    #[test]
    fn augments_entity_with_character_components() -> Result<(), Error> {
        let assertion = |world: &mut World| {
            let entity = world.create_entity().build();
            {
                let mut character_component_storages =
                    CharacterComponentStorages::fetch(&world.res);
                CharacterEntityAugmenter::augment(entity, &mut character_component_storages);
            }

            assert!(world.read_storage::<ControllerInput>().contains(entity));
            assert!(world.read_storage::<HealthPoints>().contains(entity));
            assert!(world.read_storage::<StunPoints>().contains(entity));
            assert!(world.read_storage::<RunCounter>().contains(entity));
            assert!(world.read_storage::<Grounding>().contains(entity));
            assert!(world.read_storage::<Mass>().contains(entity));
        };

        AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
            .with_setup(|world| {
                <CharacterComponentStorages as SystemData>::setup(&mut world.res);
            })
            .with_assertion(assertion)
            .run_isolated()
    }
}
