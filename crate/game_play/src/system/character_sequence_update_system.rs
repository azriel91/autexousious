use amethyst::{
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
    renderer::Flipped,
};
use character_model::{config::CharacterSequenceId, play::RunCounter};
use character_play::{
    CharacterSequenceUpdateComponents, CharacterSequenceUpdater, MirroredUpdater, RunCounterUpdater,
};
use derive_new::new;
use game_input::ControllerInput;
use object_model::play::{Grounding, HealthPoints, Mirrored, Position, Velocity};
use sequence_model::{loaded::SequenceEndTransitions, play::SequenceStatus};
use shred_derive::SystemData;
use typename_derive::TypeName;

/// Updates character sequence ID based on input (or lack of).
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct CharacterSequenceUpdateSystem;

#[allow(missing_debug_implementations)]
#[derive(SystemData)]
pub struct CharacterSequenceUpdateSystemData<'s> {
    entities: Entities<'s>,
    controller_inputs: ReadStorage<'s, ControllerInput>,
    sequence_end_transitionses: ReadStorage<'s, SequenceEndTransitions<CharacterSequenceId>>,
    positions: ReadStorage<'s, Position<f32>>,
    velocities: ReadStorage<'s, Velocity<f32>>,
    run_counters: WriteStorage<'s, RunCounter>,
    health_pointses: ReadStorage<'s, HealthPoints>,
    character_sequence_ids: WriteStorage<'s, CharacterSequenceId>,
    sequence_statuses: WriteStorage<'s, SequenceStatus>,
    mirroreds: WriteStorage<'s, Mirrored>,
    groundings: WriteStorage<'s, Grounding>,
    flippeds: WriteStorage<'s, Flipped>,
}

impl<'s> System<'s> for CharacterSequenceUpdateSystem {
    type SystemData = CharacterSequenceUpdateSystemData<'s>;

    fn run(
        &mut self,
        CharacterSequenceUpdateSystemData {
            entities,
            controller_inputs,
            sequence_end_transitionses,
            positions,
            velocities,
            mut run_counters,
            health_pointses,
            mut character_sequence_ids,
            mut sequence_statuses,
            mut mirroreds,
            mut groundings,
            mut flippeds,
        }: Self::SystemData,
    ) {
        for (
            entity,
            controller_input,
            sequence_end_transitions,
            position,
            velocity,
            run_counter,
            health_points,
            sequence_status,
            mirrored,
            grounding,
            flipped,
        ) in (
            &entities,
            &controller_inputs,
            &sequence_end_transitionses,
            &positions,
            &velocities,
            &mut run_counters,
            &health_pointses,
            &mut sequence_statuses,
            &mut mirroreds,
            &mut groundings,
            &mut flippeds,
        )
            .join()
        {
            // Retrieve sequence ID separately as we use a `FlaggedStorage` to track if it has been
            // changed.
            let character_sequence_id = character_sequence_ids.get(entity);
            if character_sequence_id.is_none() {
                continue; // kcov-ignore
            }
            let character_sequence_id =
                character_sequence_id.expect("Expected `CharacterSequenceId` to exist.");

            let next_character_sequence_id = CharacterSequenceUpdater::update(
                sequence_end_transitions,
                CharacterSequenceUpdateComponents::new(
                    &controller_input,
                    *health_points,
                    *character_sequence_id,
                    *sequence_status,
                    &position,
                    &velocity,
                    *mirrored,
                    *grounding,
                    *run_counter,
                ),
            );

            *run_counter = RunCounterUpdater::update(
                *run_counter,
                controller_input,
                *character_sequence_id,
                *mirrored,
                *grounding,
            );
            *mirrored =
                MirroredUpdater::update(controller_input, *character_sequence_id, *mirrored);

            *flipped = if mirrored.0 {
                Flipped::Horizontal
            } else {
                Flipped::None
            };

            if let Some(next_character_sequence_id) = next_character_sequence_id {
                let character_sequence_id = character_sequence_ids
                    .get_mut(entity)
                    .expect("Expected `CharacterSequenceId` to exist.");

                *character_sequence_id = next_character_sequence_id;
                *sequence_status = SequenceStatus::Begin;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        assets::AssetStorage,
        ecs::{Join, Read, ReadExpect, ReadStorage, WriteStorage},
        renderer::Flipped,
        Error,
    };
    use application_test_support::AutexousiousApplication;
    use character_model::config::CharacterSequenceId;
    use game_input::ControllerInput;
    use map_model::loaded::Map;
    use map_selection_model::MapSelection;
    use object_model::play::{Grounding, Mirrored, Position};
    use sequence_model::play::SequenceStatus;
    use typename::TypeName;

    use super::CharacterSequenceUpdateSystem;

    #[test]
    fn updates_walk_x_and_z_velocity() -> Result<(), Error> {
        let mut controller_input = ControllerInput::default();
        controller_input.x_axis_value = -1.;
        controller_input.z_axis_value = -1.;

        run_test(
            "updates_walk_x_and_z_velocity",
            SetupParams {
                sequence_id: CharacterSequenceId::Stand,
                controller_input,
                mirrored: Mirrored(false),
            },
            ExpectedParams {
                sequence_id: CharacterSequenceId::Walk,
                mirrored: Mirrored(true),
                flipped: Flipped::Horizontal,
            },
        )
    }

    #[test]
    fn flipped_is_none_when_walking_right() -> Result<(), Error> {
        let mut controller_input = ControllerInput::default();
        controller_input.x_axis_value = 1.;

        run_test(
            "updates_walk_x_and_z_velocity",
            SetupParams {
                sequence_id: CharacterSequenceId::Stand,
                controller_input,
                mirrored: Mirrored(false),
            },
            ExpectedParams {
                sequence_id: CharacterSequenceId::Walk,
                mirrored: Mirrored(false),
                flipped: Flipped::None,
            },
        )
    }

    fn run_test(
        test_name: &str,
        SetupParams {
            sequence_id: setup_sequence_id,
            controller_input: setup_controller_input,
            mirrored: setup_mirrored,
        }: SetupParams,
        ExpectedParams {
            sequence_id: expected_sequence_id,
            mirrored: expected_mirrored,
            flipped: expected_flipped,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AutexousiousApplication::game_base(test_name, false)
            .with_setup(move |world| {
                let (
                    map_selection,
                    maps,
                    mut controller_inputs,
                    mut character_sequence_ids,
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
                    &mut character_sequence_ids,
                    &mut sequence_statuses,
                    &mut positions,
                    &mut mirroreds,
                    &mut groundings,
                )
                    .join()
                    .for_each(
                        |(
                            controller_input,
                            character_sequence_id,
                            sequence_status,
                            position,
                            mirrored,
                            grounding,
                        )| {
                            *controller_input = setup_controller_input;

                            *character_sequence_id = setup_sequence_id;
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
                    |(character_sequence_ids, sequence_statuses, mirroreds, flippeds): (
                        ReadStorage<'_, CharacterSequenceId>,
                        ReadStorage<'_, SequenceStatus>,
                        ReadStorage<'_, Mirrored>,
                        ReadStorage<'_, Flipped>,
                    )| {
                        for (character_sequence_id, sequence_status, mirrored, flipped) in (
                            &character_sequence_ids,
                            &sequence_statuses,
                            &mirroreds,
                            &flippeds,
                        )
                            .join()
                        {
                            assert_eq!(expected_sequence_id, *character_sequence_id);
                            assert_eq!(SequenceStatus::Begin, *sequence_status);
                            assert_eq!(expected_mirrored, *mirrored);
                            assert_eq!(expected_flipped, *flipped);
                        }
                    },
                );
            })
            .run()
    }

    type TestSystemData<'s> = (
        ReadExpect<'s, MapSelection>,
        Read<'s, AssetStorage<Map>>,
        WriteStorage<'s, ControllerInput>,
        WriteStorage<'s, CharacterSequenceId>,
        WriteStorage<'s, SequenceStatus>,
        WriteStorage<'s, Position<f32>>,
        WriteStorage<'s, Mirrored>,
        WriteStorage<'s, Grounding>,
    );

    #[derive(Debug)]
    struct SetupParams {
        sequence_id: CharacterSequenceId,
        controller_input: ControllerInput,
        mirrored: Mirrored,
    }

    #[derive(Debug)]
    struct ExpectedParams {
        sequence_id: CharacterSequenceId,
        mirrored: Mirrored,
        flipped: Flipped,
    }
}
