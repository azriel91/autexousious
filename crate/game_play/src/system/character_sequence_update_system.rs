use amethyst::{
    ecs::{Join, ReadStorage, System, WriteStorage},
    renderer::Flipped,
};
use character_model::config::CharacterSequenceId;
use derive_new::new;
use game_input::ControllerInput;
use object_model::{
    entity::{Grounding, HealthPoints, Mirrored, Position, RunCounter, SequenceStatus, Velocity},
    loaded::SequenceEndTransitions,
};
use object_play::{
    CharacterSequenceUpdateComponents, CharacterSequenceUpdater, MirroredUpdater, RunCounterUpdater,
};
use shred_derive::SystemData;
use typename_derive::TypeName;

/// Updates character sequence ID based on input (or lack of).
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct CharacterSequenceUpdateSystem;

#[allow(missing_debug_implementations)]
#[derive(SystemData)]
pub struct CharacterSequenceUpdateSystemData<'s> {
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
            controller_input,
            sequence_end_transitions,
            position,
            velocity,
            run_counter,
            health_points,
            character_sequence_id,
            sequence_status,
            mirrored,
            grounding,
            flipped,
        ) in (
            &controller_inputs,
            &sequence_end_transitionses,
            &positions,
            &velocities,
            &mut run_counters,
            &health_pointses,
            &mut character_sequence_ids,
            &mut sequence_statuses,
            &mut mirroreds,
            &mut groundings,
            &mut flippeds,
        )
            .join()
        {
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
                *character_sequence_id = next_character_sequence_id;
                *sequence_status = SequenceStatus::Begin;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{assets::AssetStorage, ecs::prelude::*};
    use application_test_support::AutexousiousApplication;
    use character_model::config::CharacterSequenceId;
    use game_input::ControllerInput;
    use map_model::loaded::Map;
    use map_selection_model::MapSelection;
    use object_model::entity::{Grounding, Mirrored, Position, SequenceStatus};
    use typename::TypeName;

    use super::CharacterSequenceUpdateSystem;

    #[test]
    fn updates_walk_x_and_z_velocity() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::game_base("updates_walk_x_and_z_velocity", false)
                .with_setup(|world| {
                    world.exec(
                        |(
                            map_selection,
                            maps,
                            mut controller_inputs,
                            mut character_sequence_ids,
                            mut sequence_statuses,
                            mut positions,
                            mut mirroreds,
                            mut groundings,
                        ): (
                            ReadExpect<'_, MapSelection>,
                            Read<'_, AssetStorage<Map>>,
                            WriteStorage<'_, ControllerInput>,
                            WriteStorage<'_, CharacterSequenceId>,
                            WriteStorage<'_, SequenceStatus>,
                            WriteStorage<'_, Position<f32>>,
                            WriteStorage<'_, Mirrored>,
                            WriteStorage<'_, Grounding>,
                        )| {
                            let map = maps
                                .get(map_selection.handle())
                                .expect("Expected map to be loaded.");

                            for (
                                controller_input,
                                character_sequence_id,
                                sequence_status,
                                position,
                                mirrored,
                                grounding,
                            ) in (
                                &mut controller_inputs,
                                &mut character_sequence_ids,
                                &mut sequence_statuses,
                                &mut positions,
                                &mut mirroreds,
                                &mut groundings,
                            )
                                .join()
                            {
                                controller_input.x_axis_value = -1.;
                                controller_input.z_axis_value = -1.;

                                *character_sequence_id = CharacterSequenceId::Stand;
                                *sequence_status = SequenceStatus::Ongoing;
                                *mirrored = Mirrored(false);
                                *grounding = Grounding::OnGround;

                                position[1] = map.margins.bottom;
                            }
                        },
                    );
                })
                .with_system_single(
                    CharacterSequenceUpdateSystem::new(),
                    CharacterSequenceUpdateSystem::type_name(),
                    &[]
                )
                .with_assertion(|world| {
                    world.exec(
                        |(character_sequence_ids, sequence_statuses, mirroreds): (
                            ReadStorage<'_, CharacterSequenceId>,
                            ReadStorage<'_, SequenceStatus>,
                            ReadStorage<'_, Mirrored>,
                        )| {
                            for (character_sequence_id, sequence_status, mirrored) in
                                (&character_sequence_ids, &sequence_statuses, &mirroreds).join()
                            {
                                assert_eq!(CharacterSequenceId::Walk, *character_sequence_id);
                                assert_eq!(SequenceStatus::Begin, *sequence_status);
                                assert_eq!(Mirrored(true), *mirrored);
                            }
                        },
                    );
                })
                .run()
                .is_ok()
        );
    }
}
