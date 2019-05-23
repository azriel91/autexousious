use amethyst::ecs::Entity;
use character_model::play::RunCounter;
use game_input::ControllerInput;
use object_model::play::{Grounding, HealthPoints};
use object_status_model::config::StunPoints;

use crate::CharacterComponentStorages;

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
        }: &mut CharacterComponentStorages<'s>,
    ) {
        // Controller of this entity
        controller_inputs
            .insert(entity, ControllerInput::default())
            .expect("Failed to insert controller_input component.");
        // Health points.
        health_pointses
            .insert(entity, HealthPoints::default())
            .expect("Failed to insert health_points component.");
        // Stun points.
        stun_pointses
            .insert(entity, StunPoints::default())
            .expect("Failed to insert stun_points component.");
        // Run counter.
        run_counters
            .insert(entity, RunCounter::default())
            .expect("Failed to insert run_counter component.");
        // Grounding.
        groundings
            .insert(entity, Grounding::default())
            .expect("Failed to insert grounding component.");
    }
}

#[cfg(test)]
mod test {
    use amethyst::{
        ecs::{Builder, SystemData, World},
        Error,
    };
    use amethyst_test::{AmethystApplication, RenderBaseAppExt};
    use character_model::play::RunCounter;
    use game_input::ControllerInput;
    use object_model::play::{Grounding, HealthPoints};
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
        };

        AmethystApplication::render_base()
            .with_setup(|world| {
                <CharacterComponentStorages as SystemData>::setup(&mut world.res);
            })
            .with_assertion(assertion)
            .run()
    }
}
