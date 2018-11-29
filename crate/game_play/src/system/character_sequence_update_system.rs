use amethyst::{
    animation::{get_animation_set, ControlState},
    assets::AssetStorage,
    ecs::prelude::*,
    renderer::SpriteRender,
};
use game_input::ControllerInput;
use game_loading::ObjectAnimationStorages;
use object_model::{
    config::object::{CharacterSequenceId, SequenceStatus},
    entity::{CharacterStatus, Grounding, Kinematics, Mirrored, ObjectStatus, RunCounter},
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
            if object_status.sequence_status == SequenceStatus::Begin {
                object_status.sequence_status = SequenceStatus::Ongoing;
            }

            let sequence_ended = {
                sprite_animation_set
                    .animations
                    .iter()
                    .find(|&&(ref id, ref _control)| id == &object_status.sequence_id)
                    .map_or(true, |(_id, control)| control.state == ControlState::Done)
            };
            if sequence_ended {
                object_status.sequence_status = SequenceStatus::End;
            }

            let object_status_update = CharacterSequenceUpdater::update(
                character,
                CharacterSequenceUpdateComponents::new(
                    &controller_input,
                    &character_status,
                    &object_status,
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

            *object_status += object_status_update;
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
        config::object::{CharacterSequenceId, SequenceStatus},
        entity::{Grounding, Kinematics, Mirrored, ObjectStatus},
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
                            mut kinematicses,
                            mut groundings,
                        ): (
                            ReadExpect<MapSelection>,
                            Read<AssetStorage<Map>>,
                            WriteStorage<ObjectStatus<CharacterSequenceId>>,
                            WriteStorage<Kinematics<f32>>,
                            WriteStorage<Grounding>,
                        )| {
                            let map = maps
                                .get(map_selection.handle())
                                .expect("Expected map to be loaded.");

                            for (object_status, kinematics, grounding) in
                                (&mut object_statuses, &mut kinematicses, &mut groundings).join()
                            {
                                object_status.sequence_id = CharacterSequenceId::Stand;
                                object_status.sequence_status = SequenceStatus::Begin;
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
                        |object_statuses: ReadStorage<ObjectStatus<CharacterSequenceId>>| {
                            for object_status in object_statuses.join() {
                                assert_eq!(SequenceStatus::Ongoing, object_status.sequence_status);
                            }
                        },
                    );
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
                            mut kinematicses,
                            mut mirroreds,
                            mut groundings,
                        ): (
                            ReadExpect<MapSelection>,
                            Read<AssetStorage<Map>>,
                            WriteStorage<ControllerInput>,
                            WriteStorage<ObjectStatus<CharacterSequenceId>>,
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
                                kinematics,
                                mirrored,
                                grounding,
                            ) in (
                                &mut controller_inputs,
                                &mut object_statuses,
                                &mut kinematicses,
                                &mut mirroreds,
                                &mut groundings,
                            )
                                .join()
                            {
                                controller_input.x_axis_value = -1.;
                                controller_input.z_axis_value = -1.;

                                object_status.sequence_id = CharacterSequenceId::Stand;
                                object_status.sequence_status = SequenceStatus::Ongoing;
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
                        |(object_statuses, mirroreds): (
                            ReadStorage<ObjectStatus<CharacterSequenceId>>,
                            ReadStorage<Mirrored>,
                        )| {
                            for (object_status, mirrored) in (&object_statuses, &mirroreds).join() {
                                assert_eq!(CharacterSequenceId::Walk, object_status.sequence_id);
                                assert_eq!(SequenceStatus::Begin, object_status.sequence_status);
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
