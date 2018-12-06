use amethyst::{
    animation::{get_animation_set, ControlState},
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};
use game_input::ControllerInput;
use game_loading::ObjectAnimationStorages;
use object_model::{
    config::object::CharacterSequenceId,
    entity::{Grounding, HealthPoints, Mirrored, Position, RunCounter, SequenceStatus, Velocity},
    loaded::SequenceEndTransitions,
};
use object_play::{
    CharacterSequenceUpdateComponents, CharacterSequenceUpdater, MirroredUpdater, RunCounterUpdater,
};
use shred_derive::SystemData;

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
    sprite_renders: WriteStorage<'s, SpriteRender>,
    object_acses: ObjectAnimationStorages<'s, CharacterSequenceId>,
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
            mut sprite_renders,
            object_acses,
        }: Self::SystemData,
    ) {
        let (mut sprite_acs, _body_frame_acs, _interaction_acs) = object_acses;
        for (
            entity,
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
            sprite_render,
        ) in (
            &*entities,
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
            &mut sprite_renders,
        )
            .join()
        {
            let mut sprite_animation_set = get_animation_set(&mut sprite_acs, entity)
                .expect("Sprite animation should exist as entity should be valid.");

            // Mark sequence as `Ongoing` for subsequent tick.
            if *sequence_status == SequenceStatus::Begin {
                *sequence_status = SequenceStatus::Ongoing;
            }

            let sequence_ended = {
                sprite_animation_set
                    .animations
                    .iter()
                    .find(|&&(ref id, ref _control)| id == character_sequence_id)
                    .map_or(true, |(_id, control)| control.state == ControlState::Done)
            };
            if sequence_ended {
                *sequence_status = SequenceStatus::End;
            }

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

            sprite_render.flip_horizontal = mirrored.0;

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
    use game_input::ControllerInput;
    use map_model::loaded::Map;
    use map_selection_model::MapSelection;
    use object_model::{
        config::object::CharacterSequenceId,
        entity::{Grounding, Mirrored, Position, SequenceStatus},
    };
    use typename::TypeName;

    use super::CharacterSequenceUpdateSystem;

    #[test]
    fn updates_sequence_status_begin_to_ongoing() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::game_base("updates_sequence_status_begin_to_ongoing", false)
                .with_setup(|world| {
                    world.exec(
                        |(
                            map_selection,
                            maps,
                            mut character_sequence_ids,
                            mut sequence_statuses,
                            mut positions,
                            mut groundings,
                        ): (
                            ReadExpect<MapSelection>,
                            Read<AssetStorage<Map>>,
                            WriteStorage<CharacterSequenceId>,
                            WriteStorage<SequenceStatus>,
                            WriteStorage<Position<f32>>,
                            WriteStorage<Grounding>,
                        )| {
                            let map = maps
                                .get(map_selection.handle())
                                .expect("Expected map to be loaded.");

                            for (character_sequence_id, sequence_status, position, grounding) in (
                                &mut character_sequence_ids,
                                &mut sequence_statuses,
                                &mut positions,
                                &mut groundings,
                            )
                                .join()
                            {
                                *character_sequence_id = CharacterSequenceId::Stand;
                                *sequence_status = SequenceStatus::Begin;
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
                    world.exec(|sequence_statuses: ReadStorage<SequenceStatus>| {
                        for sequence_status in sequence_statuses.join() {
                            assert_eq!(SequenceStatus::Ongoing, *sequence_status);
                        }
                    });
                })
                .run()
                .is_ok()
        );
    }

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
                            ReadExpect<MapSelection>,
                            Read<AssetStorage<Map>>,
                            WriteStorage<ControllerInput>,
                            WriteStorage<CharacterSequenceId>,
                            WriteStorage<SequenceStatus>,
                            WriteStorage<Position<f32>>,
                            WriteStorage<Mirrored>,
                            WriteStorage<Grounding>,
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
                            ReadStorage<CharacterSequenceId>,
                            ReadStorage<SequenceStatus>,
                            ReadStorage<Mirrored>,
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
