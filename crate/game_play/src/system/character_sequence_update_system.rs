use amethyst::{
    animation::{get_animation_set, ControlState},
    assets::AssetStorage,
    ecs::prelude::*,
    renderer::SpriteRender,
};
use game_input::ControllerInput;
use game_loading::ObjectAnimationStorages;
use object_model::{
    config::object::CharacterSequenceId,
    entity::{
        CharacterStatus, Grounding, Kinematics, Mirrored, ObjectStatus, RunCounter, SequenceStatus,
    },
    loaded::{Character, CharacterHandle},
};
use object_play::{
    CharacterSequenceUpdateComponents, CharacterSequenceUpdater, MirroredUpdater, RunCounterUpdater,
};

/// Updates `Character` sequence based on input
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct CharacterSequenceUpdateSystem;

type CharacterSequenceUpdateSystemData<'s> = (
    Entities<'s>,
    Read<'s, AssetStorage<Character>>,
    ReadStorage<'s, CharacterHandle>,
    ReadStorage<'s, ControllerInput>,
    ReadStorage<'s, Kinematics<f32>>,
    WriteStorage<'s, RunCounter>,
    ReadStorage<'s, CharacterStatus>,
    WriteStorage<'s, ObjectStatus<CharacterSequenceId>>,
    WriteStorage<'s, SequenceStatus>,
    WriteStorage<'s, Mirrored>,
    WriteStorage<'s, Grounding>,
    WriteStorage<'s, SpriteRender>,
    ObjectAnimationStorages<'s, CharacterSequenceId>,
);

impl<'s> System<'s> for CharacterSequenceUpdateSystem {
    type SystemData = CharacterSequenceUpdateSystemData<'s>;

    fn run(
        &mut self,
        (
            entities,
            character_assets,
            character_handles,
            controller_inputs,
            kinematicses,
            mut run_counters,
            character_statuses,
            mut object_statuses,
            mut sequence_statuses,
            mut mirroreds,
            mut groundings,
            mut sprite_renders,
            (mut sprite_acs, _body_frame_acs, _interaction_acs),
        ): Self::SystemData,
    ) {
        for (
            entity,
            character_handle,
            controller_input,
            kinematics,
            run_counter,
            character_status,
            object_status,
            sequence_status,
            mirrored,
            grounding,
            sprite_render,
        ) in (
            &*entities,
            &character_handles,
            &controller_inputs,
            &kinematicses,
            &mut run_counters,
            &character_statuses,
            &mut object_statuses,
            &mut sequence_statuses,
            &mut mirroreds,
            &mut groundings,
            &mut sprite_renders,
        )
            .join()
        {
            let character = character_assets
                .get(character_handle)
                .expect("Expected character to be loaded.");

            // TODO: Is it faster if we update the character statuses first, then calculate the
            // sequence updates in parallel?
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
                    .find(|&&(ref id, ref _control)| id == &object_status.sequence_id)
                    .map_or(true, |(_id, control)| control.state == ControlState::Done)
            };
            if sequence_ended {
                *sequence_status = SequenceStatus::End;
            }

            let object_status_update = CharacterSequenceUpdater::update(
                character,
                CharacterSequenceUpdateComponents::new(
                    &controller_input,
                    &character_status,
                    &object_status,
                    *sequence_status,
                    &kinematics,
                    *mirrored,
                    *grounding,
                    *run_counter,
                ),
            );

            *run_counter = RunCounterUpdater::update(
                *run_counter,
                controller_input,
                object_status,
                *mirrored,
                *grounding,
            );
            *mirrored = MirroredUpdater::update(controller_input, object_status, *mirrored);

            sprite_render.flip_horizontal = mirrored.0;

            if let Some(sequence_id) = object_status_update.sequence_id {
                object_status.sequence_id = sequence_id;
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
        entity::{Grounding, Kinematics, Mirrored, ObjectStatus, SequenceStatus},
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
                            mut object_statuses,
                            mut sequence_statuses,
                            mut kinematicses,
                            mut groundings,
                        ): (
                            ReadExpect<MapSelection>,
                            Read<AssetStorage<Map>>,
                            WriteStorage<ObjectStatus<CharacterSequenceId>>,
                            WriteStorage<SequenceStatus>,
                            WriteStorage<Kinematics<f32>>,
                            WriteStorage<Grounding>,
                        )| {
                            let map = maps
                                .get(map_selection.handle())
                                .expect("Expected map to be loaded.");

                            for (object_status, sequence_status, kinematics, grounding) in (
                                &mut object_statuses,
                                &mut sequence_statuses,
                                &mut kinematicses,
                                &mut groundings,
                            )
                                .join()
                            {
                                object_status.sequence_id = CharacterSequenceId::Stand;
                                *sequence_status = SequenceStatus::Begin;
                                *grounding = Grounding::OnGround;

                                kinematics.position[1] = map.margins.bottom;
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
                            mut object_statuses,
                            mut sequence_statuses,
                            mut kinematicses,
                            mut mirroreds,
                            mut groundings,
                        ): (
                            ReadExpect<MapSelection>,
                            Read<AssetStorage<Map>>,
                            WriteStorage<ControllerInput>,
                            WriteStorage<ObjectStatus<CharacterSequenceId>>,
                            WriteStorage<SequenceStatus>,
                            WriteStorage<Kinematics<f32>>,
                            WriteStorage<Mirrored>,
                            WriteStorage<Grounding>,
                        )| {
                            let map = maps
                                .get(map_selection.handle())
                                .expect("Expected map to be loaded.");

                            for (
                                controller_input,
                                object_status,
                                sequence_status,
                                kinematics,
                                mirrored,
                                grounding,
                            ) in (
                                &mut controller_inputs,
                                &mut object_statuses,
                                &mut sequence_statuses,
                                &mut kinematicses,
                                &mut mirroreds,
                                &mut groundings,
                            )
                                .join()
                            {
                                controller_input.x_axis_value = -1.;
                                controller_input.z_axis_value = -1.;

                                object_status.sequence_id = CharacterSequenceId::Stand;
                                *sequence_status = SequenceStatus::Ongoing;
                                *mirrored = Mirrored(false);
                                *grounding = Grounding::OnGround;

                                kinematics.position[1] = map.margins.bottom;
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
                        |(object_statuses, sequence_statuses, mirroreds): (
                            ReadStorage<ObjectStatus<CharacterSequenceId>>,
                            ReadStorage<SequenceStatus>,
                            ReadStorage<Mirrored>,
                        )| {
                            for (object_status, sequence_status, mirrored) in
                                (&object_statuses, &sequence_statuses, &mirroreds).join()
                            {
                                assert_eq!(CharacterSequenceId::Walk, object_status.sequence_id);
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
