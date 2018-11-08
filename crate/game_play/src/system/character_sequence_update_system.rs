use amethyst::{
    animation::{get_animation_set, ControlState},
    assets::AssetStorage,
    ecs::prelude::*,
    renderer::SpriteRender,
};
use game_input::ControllerInput;
use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{CharacterStatus, Kinematics},
    loaded::{AnimatedComponentAnimation, Character, CharacterHandle},
};
use object_play::CharacterSequenceUpdater;

use game_loading::{AnimationRunner, ObjectAnimationStorages};

/// Updates `Character` sequence based on input
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct CharacterSequenceUpdateSystem;

type CharacterSequenceUpdateSystemData<'s> = (
    Entities<'s>,
    Read<'s, AssetStorage<Character>>,
    ReadStorage<'s, CharacterHandle>,
    ReadStorage<'s, ControllerInput>,
    ReadStorage<'s, Kinematics<f32>>,
    WriteStorage<'s, CharacterStatus>,
    WriteStorage<'s, SpriteRender>,
    ObjectAnimationStorages<'s, CharacterSequenceId>,
);

impl<'s> System<'s> for CharacterSequenceUpdateSystem {
    type SystemData = CharacterSequenceUpdateSystemData<'s>;

    fn run(
        &mut self,
        (
            entities,
            characters,
            handle_storage,
            controller_input_storage,
            kinematics_storage,
            mut character_status_storage,
            mut sprite_render_storage,
            (mut sprite_acs, mut body_frame_acs, mut interaction_acs),
        ): Self::SystemData,
    ) {
        for (
            entity,
            character_handle,
            controller_input,
            kinematics,
            mut character_status,
            mut sprite_render,
        ) in (
            &*entities,
            &handle_storage,
            &controller_input_storage,
            &kinematics_storage,
            &mut character_status_storage,
            &mut sprite_render_storage,
        )
            .join()
        {
            let character = characters
                .get(character_handle)
                .expect("Expected character to be loaded.");

            // TODO: Is it faster if we update the character statuses first, then calculate the
            // sequence updates in parallel?
            let mut sprite_animation_set = get_animation_set(&mut sprite_acs, entity)
                .expect("Sprite animation should exist as entity should be valid.");
            let mut body_animation_set = get_animation_set(&mut body_frame_acs, entity)
                .expect("Body animation should exist as entity should be valid.");
            let mut interaction_animation_set = get_animation_set(&mut interaction_acs, entity)
                .expect("Interaction animation should exist as entity should be valid.");

            // Mark sequence as `Ongoing` for subsequent tick.
            if character_status.object_status.sequence_state == SequenceState::Begin {
                character_status.object_status.sequence_state = SequenceState::Ongoing;
            }

            let sequence_ended = {
                sprite_animation_set
                    .animations
                    .iter()
                    .find(|&&(ref id, ref _control)| {
                        id == &character_status.object_status.sequence_id
                    })
                    .map_or(true, |(_id, control)| control.state == ControlState::Done)
            };
            if sequence_ended {
                character_status.object_status.sequence_state = SequenceState::End;
            }

            let status_update = CharacterSequenceUpdater::update(
                character,
                &controller_input,
                &character_status,
                &kinematics,
            );

            // Update the current sequence ID
            if let Some(next_sequence_id) = status_update.object_status.sequence_id {
                let animations = &character
                    .object
                    .animations
                    .get(&next_sequence_id)
                    .unwrap_or_else(|| {
                        panic!(
                            "Failed to get animations for sequence: `{:?}`",
                            next_sequence_id
                        )
                    });

                animations
                    .iter()
                    .for_each(|animated_component| match animated_component {
                        AnimatedComponentAnimation::SpriteRender(ref handle) => {
                            AnimationRunner::swap(
                                character_status.object_status.sequence_id,
                                next_sequence_id,
                                &mut sprite_animation_set,
                                handle,
                            );
                        }
                        AnimatedComponentAnimation::BodyFrame(ref handle) => {
                            AnimationRunner::swap(
                                character_status.object_status.sequence_id,
                                next_sequence_id,
                                &mut body_animation_set,
                                handle,
                            );
                        }
                        AnimatedComponentAnimation::InteractionFrame(ref handle) => {
                            AnimationRunner::swap(
                                character_status.object_status.sequence_id,
                                next_sequence_id,
                                &mut interaction_animation_set,
                                handle,
                            );
                        }
                    });
            }

            if let Some(mirrored) = status_update.object_status.mirrored {
                sprite_render.flip_horizontal = mirrored;
            }

            *character_status += status_update;
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
        config::object::{CharacterSequenceId, SequenceState},
        entity::{CharacterStatus, Grounding, Kinematics},
    };
    use typename::TypeName;

    use super::CharacterSequenceUpdateSystem;

    #[test]
    fn updates_sequence_state_begin_to_ongoing() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::game_base("updates_sequence_state_begin_to_ongoing", false)
                .with_setup(|world| {
                    world.exec(
                        |(map_selection, maps, mut status_storage, mut kinematics_storage): (
                            ReadExpect<MapSelection>,
                            Read<AssetStorage<Map>>,
                            WriteStorage<CharacterStatus>,
                            WriteStorage<Kinematics<f32>>,
                        )| {
                            let map = maps
                                .get(map_selection.handle())
                                .expect("Expected map to be loaded.");

                            for (status, kinematics) in
                                (&mut status_storage, &mut kinematics_storage).join()
                            {
                                status.object_status.grounding = Grounding::OnGround;
                                status.object_status.sequence_id = CharacterSequenceId::Stand;
                                status.object_status.sequence_state = SequenceState::Begin;

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
                    world.exec(|status_storage: ReadStorage<CharacterStatus>| {
                        for status in status_storage.join() {
                            assert_eq!(SequenceState::Ongoing, status.object_status.sequence_state);
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
                            mut controller_input_storage,
                            mut status_storage,
                            mut kinematics_storage,
                        ): (
                            ReadExpect<MapSelection>,
                            Read<AssetStorage<Map>>,
                            WriteStorage<ControllerInput>,
                            WriteStorage<CharacterStatus>,
                            WriteStorage<Kinematics<f32>>,
                        )| {
                            let map = maps
                                .get(map_selection.handle())
                                .expect("Expected map to be loaded.");

                            for (controller_input, status, kinematics) in (
                                &mut controller_input_storage,
                                &mut status_storage,
                                &mut kinematics_storage,
                            )
                                .join()
                            {
                                controller_input.x_axis_value = -1.;
                                controller_input.z_axis_value = -1.;

                                status.object_status.grounding = Grounding::OnGround;
                                status.object_status.sequence_id = CharacterSequenceId::Stand;
                                status.object_status.sequence_state = SequenceState::Ongoing;
                                status.object_status.mirrored = false;

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
                    world.exec(|status_storage: ReadStorage<CharacterStatus>| {
                        for status in status_storage.join() {
                            assert_eq!(CharacterSequenceId::Walk, status.object_status.sequence_id);
                            assert_eq!(SequenceState::Begin, status.object_status.sequence_state);
                            assert!(status.object_status.mirrored);
                        }
                    });
                })
                .run()
                .is_ok()
        );
    }
}
