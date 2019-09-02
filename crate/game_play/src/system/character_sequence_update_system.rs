use amethyst::{
    assets::AssetStorage,
    ecs::{Entities, Join, Read, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use character_model::{
    loaded::{Character, CharacterHandle},
    play::RunCounter,
};
use character_play::{
    CharacterSequenceUpdateComponents, CharacterSequenceUpdater, MirroredUpdater, RunCounterUpdater,
};
use derivative::Derivative;
use derive_new::new;
use game_input::ControllerInput;
use kinematic_model::config::{Position, Velocity};
use object_model::play::{Grounding, HealthPoints, Mirrored};
use sequence_model::{config::SequenceNameString, loaded::SequenceId, play::SequenceStatus};
use typename_derive::TypeName;

/// Updates character sequence name based on input (or lack of).
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct CharacterSequenceUpdateSystem;

#[derive(Derivative, SystemData)]
pub struct CharacterSequenceUpdateSystemData<'s> {
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `CharacterHandle` components.
    #[derivative(Debug = "ignore")]
    pub character_handles: ReadStorage<'s, CharacterHandle>,
    /// `Character` assets.
    #[derivative(Debug = "ignore")]
    pub character_assets: Read<'s, AssetStorage<Character>>,
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
            character_handles,
            character_assets,
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
            character_handle,
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
            &character_handles,
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

            let character = character_assets
                .get(character_handle)
                .expect("Expected `Character` to be loaded.");
            let character_sequence_name_string = character
                .sequence_id_mappings
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

                let next_sequence_id = *character
                    .sequence_id_mappings
                    .id(&SequenceNameString::Name(next_sequence_name))
                    .expect("Expected sequence ID mapping to exist.");

                *sequence_id = next_sequence_id;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        assets::AssetStorage,
        ecs::{Join, Read, ReadExpect, ReadStorage, WriteStorage},
        Error,
    };
    use application_test_support::AutexousiousApplication;
    use game_input::ControllerInput;
    use kinematic_model::config::Position;
    use map_model::loaded::Map;
    use map_selection_model::MapSelection;
    use object_model::play::{Grounding, Mirrored};
    use sequence_model::{loaded::SequenceId, play::SequenceStatus};
    use typename::TypeName;

    use super::CharacterSequenceUpdateSystem;

    #[test]
    fn updates_walk_x_and_z_velocity() -> Result<(), Error> {
        let mut controller_input = ControllerInput::default();
        controller_input.x_axis_value = -1.;
        controller_input.z_axis_value = -1.;

        run_test(
            SetupParams {
                sequence_id: SequenceId::new(0),
                controller_input,
                mirrored: Mirrored(false),
            },
            ExpectedParams {
                sequence_id: SequenceId::new(3),
                mirrored: Mirrored(true),
            },
        )
    }

    #[test]
    fn flipped_is_none_when_walking_right() -> Result<(), Error> {
        let mut controller_input = ControllerInput::default();
        controller_input.x_axis_value = 1.;

        run_test(
            SetupParams {
                sequence_id: SequenceId::new(0),
                controller_input,
                mirrored: Mirrored(false),
            },
            ExpectedParams {
                sequence_id: SequenceId::new(3),
                mirrored: Mirrored(false),
            },
        )
    }

    fn run_test(
        SetupParams {
            sequence_id: setup_sequence_id,
            controller_input: setup_controller_input,
            mirrored: setup_mirrored,
        }: SetupParams,
        ExpectedParams {
            sequence_id: expected_sequence_id,
            mirrored: expected_mirrored,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_effect(move |world| {
                let (
                    map_selection,
                    maps,
                    mut controller_inputs,
                    mut sequence_ids,
                    mut sequence_statuses,
                    mut positions,
                    mut mirroreds,
                    mut groundings,
                ) = world.system_data::<TestSystemData>();

                let map = maps
                    .get(map_selection.handle())
                    .expect("Expected map to be loaded.");

                (
                    &mut controller_inputs,
                    &mut sequence_ids,
                    &mut sequence_statuses,
                    &mut positions,
                    &mut mirroreds,
                    &mut groundings,
                )
                    .join()
                    .for_each(
                        |(
                            controller_input,
                            sequence_id,
                            sequence_status,
                            position,
                            mirrored,
                            grounding,
                        )| {
                            *controller_input = setup_controller_input;

                            *sequence_id = setup_sequence_id;
                            *sequence_status = SequenceStatus::Ongoing;
                            *mirrored = setup_mirrored;
                            *grounding = Grounding::OnGround;

                            position[1] = map.margins.bottom;
                        },
                    );
            })
            .with_system_single(
                CharacterSequenceUpdateSystem::new(),
                CharacterSequenceUpdateSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_assertion(move |world| {
                world.exec(
                    |(sequence_ids, mirroreds): (
                        ReadStorage<'_, SequenceId>,
                        ReadStorage<'_, Mirrored>,
                    )| {
                        for (sequence_id, mirrored) in (&sequence_ids, &mirroreds).join() {
                            assert_eq!(expected_sequence_id, *sequence_id);
                            assert_eq!(expected_mirrored, *mirrored);
                        }
                    },
                );
            })
            .run_isolated()
    }

    type TestSystemData<'s> = (
        ReadExpect<'s, MapSelection>,
        Read<'s, AssetStorage<Map>>,
        WriteStorage<'s, ControllerInput>,
        WriteStorage<'s, SequenceId>,
        WriteStorage<'s, SequenceStatus>,
        WriteStorage<'s, Position<f32>>,
        WriteStorage<'s, Mirrored>,
        WriteStorage<'s, Grounding>,
    );

    #[derive(Debug)]
    struct SetupParams {
        sequence_id: SequenceId,
        controller_input: ControllerInput,
        mirrored: Mirrored,
    }

    #[derive(Debug)]
    struct ExpectedParams {
        sequence_id: SequenceId,
        mirrored: Mirrored,
    }
}
