use amethyst::{
    ecs::{Entities, Join, Read, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::loaded::AssetId;
use character_model::{config::CharacterSequenceName, play::RunCounter};
use character_play::{
    CharacterSequenceUpdateComponents, CharacterSequenceUpdater, MirroredUpdater, RunCounterUpdater,
};
use derivative::Derivative;
use derive_new::new;
use game_input::ControllerInput;
use kinematic_model::config::{Position, Velocity};
use mirrored_model::play::Mirrored;
use object_model::play::{Grounding, HealthPoints};
use sequence_model::{
    config::SequenceNameString,
    loaded::{AssetSequenceIdMappings, SequenceId},
    play::SequenceStatus,
};

/// Updates character sequence name based on input (or lack of).
#[derive(Debug, Default, new)]
pub struct CharacterSequenceUpdateSystem;

/// `CharacterSequenceUpdateSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CharacterSequenceUpdateSystemData<'s> {
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `AssetId` components.
    #[derivative(Debug = "ignore")]
    pub asset_ids: ReadStorage<'s, AssetId>,
    /// `AssetSequenceIdMappings<CharacterSequenceName>` resource.
    #[derivative(Debug = "ignore")]
    pub asset_sequence_id_mappings_character:
        Read<'s, AssetSequenceIdMappings<CharacterSequenceName>>,
    /// `ControllerInput` components.
    #[derivative(Debug = "ignore")]
    pub controller_inputs: ReadStorage<'s, ControllerInput>,
    /// `Position<f32>` components.
    #[derivative(Debug = "ignore")]
    pub positions: ReadStorage<'s, Position<f32>>,
    /// `Velocity<f32>` components.
    #[derivative(Debug = "ignore")]
    pub velocities: ReadStorage<'s, Velocity<f32>>,
    /// `HealthPoints` components.
    #[derivative(Debug = "ignore")]
    pub health_pointses: ReadStorage<'s, HealthPoints>,
    /// `SequenceStatus` components.
    #[derivative(Debug = "ignore")]
    pub sequence_statuses: ReadStorage<'s, SequenceStatus>,
    /// `RunCounter` components.
    #[derivative(Debug = "ignore")]
    pub run_counters: WriteStorage<'s, RunCounter>,
    /// `SequenceId` components.
    #[derivative(Debug = "ignore")]
    pub sequence_ids: WriteStorage<'s, SequenceId>,
    /// `Mirrored` components.
    #[derivative(Debug = "ignore")]
    pub mirroreds: WriteStorage<'s, Mirrored>,
    /// `Grounding` components.
    #[derivative(Debug = "ignore")]
    pub groundings: WriteStorage<'s, Grounding>,
}

impl<'s> System<'s> for CharacterSequenceUpdateSystem {
    type SystemData = CharacterSequenceUpdateSystemData<'s>;

    fn run(
        &mut self,
        CharacterSequenceUpdateSystemData {
            entities,
            asset_ids,
            asset_sequence_id_mappings_character,
            controller_inputs,
            positions,
            velocities,
            health_pointses,
            sequence_statuses,
            mut run_counters,
            mut sequence_ids,
            mut mirroreds,
            mut groundings,
        }: Self::SystemData,
    ) {
        for (
            entity,
            asset_id,
            controller_input,
            position,
            velocity,
            health_points,
            sequence_status,
            run_counter,
            mirrored,
            grounding,
        ) in (
            &entities,
            &asset_ids,
            &controller_inputs,
            &positions,
            &velocities,
            &health_pointses,
            &sequence_statuses,
            &mut run_counters,
            &mut mirroreds,
            &mut groundings,
        )
            .join()
        {
            // Retrieve sequence ID separately as we use a `FlaggedStorage` to track if it has been
            // changed.
            let sequence_id = sequence_ids.get(entity);
            if sequence_id.is_none() {
                continue; // kcov-ignore
            }
            let sequence_id = sequence_id.expect("Expected `SequenceId` to exist.");

            let sequence_id_mappings = asset_sequence_id_mappings_character
                .get(*asset_id)
                .unwrap_or_else(|| {
                    panic!(
                        "Expected `SequenceIdMappings<CharacterSequenceName>` to exist for `{:?}`.",
                        asset_id
                    )
                });
            let character_sequence_name_string = sequence_id_mappings
                .name(*sequence_id)
                .expect("Expected sequence ID mapping to exist.");

            let next_sequence_name = if let SequenceNameString::Name(character_sequence_name) =
                character_sequence_name_string
            {
                CharacterSequenceUpdater::update(CharacterSequenceUpdateComponents::new(
                    &controller_input,
                    *health_points,
                    *character_sequence_name,
                    *sequence_status,
                    &position,
                    &velocity,
                    *mirrored,
                    *grounding,
                    *run_counter,
                ))
            } else {
                None
            };

            *run_counter = RunCounterUpdater::update(
                *run_counter,
                controller_input,
                character_sequence_name_string,
                *mirrored,
                *grounding,
            );
            *mirrored = MirroredUpdater::update(
                controller_input,
                character_sequence_name_string,
                *mirrored,
            );

            if let Some(next_sequence_name) = next_sequence_name {
                let sequence_id = sequence_ids
                    .get_mut(entity)
                    .expect("Expected `SequenceId` to exist.");

                let next_sequence_id = sequence_id_mappings
                    .id(&SequenceNameString::Name(next_sequence_name))
                    .copied()
                    .expect("Expected sequence ID mapping to exist.");

                *sequence_id = next_sequence_id;
            }
        }
    }
}
