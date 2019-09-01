use std::convert::TryInto;

use amethyst::ecs::Entity;
use character_model::{config::CharacterDefinition, play::RunCounter};
use charge_model::play::{ChargeRetention, ChargeTrackerClock};
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
            ref mut charge_tracker_clocks,
            ref mut charge_limits,
            ref mut charge_delays,
            ref mut charge_use_modes,
            ref mut charge_retentions,
        }: &mut CharacterComponentStorages<'s>,
        character_definition: &CharacterDefinition,
    ) {
        // Controller of this entity
        controller_inputs
            .insert(entity, ControllerInput::default())
            .expect("Failed to insert `ControllerInput` component.");
        health_pointses
            .insert(entity, HealthPoints::default())
            .expect("Failed to insert `HealthPoints` component.");
        stun_pointses
            .insert(entity, StunPoints::default())
            .expect("Failed to insert `StunPoints` component.");
        run_counters
            .insert(entity, RunCounter::default())
            .expect("Failed to insert `RunCounter` component.");
        groundings
            .insert(entity, Grounding::default())
            .expect("Failed to insert `Grounding` component.");
        masses
            .insert(entity, CHARACTER_MASS_DEFAULT)
            .expect("Failed to insert `Mass` component.");
        charge_tracker_clocks
            .insert(
                entity,
                ChargeTrackerClock::new(
                    (*character_definition.charge_limit)
                        .try_into()
                        .expect("Failed to convert `ChargeLimit` `u32` into `usize`."),
                ),
            )
            .expect("Failed to insert `ChargeTrackerClock` component.");
        charge_limits
            .insert(entity, character_definition.charge_limit)
            .expect("Failed to insert `ChargeLimit` component.");
        charge_delays
            .insert(entity, character_definition.charge_delay)
            .expect("Failed to insert `ChargeDelay` component.");
        charge_use_modes
            .insert(entity, character_definition.charge_use_mode)
            .expect("Failed to insert `ChargeUseMode` component.");
        charge_retentions
            .insert(
                entity,
                ChargeRetention::from(character_definition.charge_retention_mode),
            )
            .expect("Failed to insert `ChargeUseMode` component.");
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
    use character_model::{config::CharacterDefinition, play::RunCounter};
    use charge_model::{
        config::{ChargeDelay, ChargeLimit, ChargeUseMode},
        play::{ChargeRetention, ChargeTrackerClock},
    };
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
                let mut character_component_storages = CharacterComponentStorages::fetch(&world);
                CharacterEntityAugmenter::augment(
                    entity,
                    &mut character_component_storages,
                    &CharacterDefinition::default(),
                );
            }

            assert!(world.read_storage::<ControllerInput>().contains(entity));
            assert!(world.read_storage::<HealthPoints>().contains(entity));
            assert!(world.read_storage::<StunPoints>().contains(entity));
            assert!(world.read_storage::<RunCounter>().contains(entity));
            assert!(world.read_storage::<Grounding>().contains(entity));
            assert!(world.read_storage::<Mass>().contains(entity));
            assert!(world.read_storage::<ChargeTrackerClock>().contains(entity));
            assert!(world.read_storage::<ChargeLimit>().contains(entity));
            assert!(world.read_storage::<ChargeDelay>().contains(entity));
            assert!(world.read_storage::<ChargeUseMode>().contains(entity));
            assert!(world.read_storage::<ChargeRetention>().contains(entity));
        };

        AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
            .with_effect(|world| {
                <CharacterComponentStorages as SystemData>::setup(world);
            })
            .with_assertion(assertion)
            .run_isolated()
    }
}
