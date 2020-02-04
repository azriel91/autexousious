use std::convert::TryInto;

use amethyst::ecs::Entity;
use asset_model::loaded::AssetId;
use character_model::{
    config::CharacterSequenceName, loaded::CharacterHitTransitions, play::RunCounter,
};
use charge_model::play::{ChargeRetention, ChargeTrackerClock};
use game_input_model::play::ControllerInput;
use map_model::play::MapBounded;
use object_model::{config::Mass, play::HealthPoints};
use object_status_model::config::StunPoints;
use sequence_model::{config::SequenceNameString, loaded::SequenceId};

use crate::{CharacterComponentStorages, CharacterSpawningResources};

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
    /// * `character_spawning_resources`: Resources needed to spawn the character.
    /// * `character_component_storages`: Character specific `Component` storages.
    /// * `asset_id`: Asset ID of the character.
    /// * `entity`: The entity to augment.
    pub fn augment<'s>(
        CharacterSpawningResources {
            asset_sequence_id_mappings_character,
            asset_character_definition_handle,
            character_definition_assets,
        }: &CharacterSpawningResources<'s>,
        CharacterComponentStorages {
            controller_inputs,
            health_pointses,
            stun_pointses,
            run_counters,
            masses,
            map_boundeds,
            charge_tracker_clocks,
            charge_limits,
            charge_delays,
            charge_use_modes,
            charge_retentions,
            character_hit_transitionses,
        }: &mut CharacterComponentStorages<'s>,
        asset_id: AssetId,
        entity: Entity,
    ) {
        let character_definition_handle = asset_character_definition_handle
            .get(asset_id)
            .unwrap_or_else(|| {
                panic!(
                    "Expected `CharacterDefinitionHandle` to exist for `{:?}`.",
                    asset_id
                )
            });
        let character_definition = character_definition_assets
            .get(&character_definition_handle)
            .expect("Expected `CharacterDefinition` to be loaded.");

        let sequence_id_mappings = asset_sequence_id_mappings_character
            .get(asset_id)
            .unwrap_or_else(|| {
                panic!(
                    "Expected `SequenceIdMappings<CharacterSequenceName>` to exist for `{:?}`.",
                    asset_id
                )
            });
        let low_stun = sequence_id_mappings
            .id(&SequenceNameString::Name(CharacterSequenceName::Flinch0))
            .copied()
            .unwrap_or(SequenceId(0));
        let mid_stun = sequence_id_mappings
            .id(&SequenceNameString::Name(CharacterSequenceName::Flinch1))
            .copied()
            .unwrap_or(SequenceId(0));
        let high_stun = sequence_id_mappings
            .id(&SequenceNameString::Name(CharacterSequenceName::Dazed))
            .copied()
            .unwrap_or(SequenceId(0));
        let falling = sequence_id_mappings
            .id(&SequenceNameString::Name(
                CharacterSequenceName::FallForwardAscend,
            ))
            .copied()
            .unwrap_or(SequenceId(0));

        let character_hit_transitions = CharacterHitTransitions {
            low_stun,
            mid_stun,
            high_stun,
            falling,
        };

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
        masses
            .insert(entity, CHARACTER_MASS_DEFAULT)
            .expect("Failed to insert `Mass` component.");
        map_boundeds
            .insert(entity, MapBounded::default())
            .expect("Failed to insert `MapBounded` component.");
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
        character_hit_transitionses
            .insert(entity, character_hit_transitions)
            .expect("Failed to insert `CharacterHitTransitions` component.");
    }
}
