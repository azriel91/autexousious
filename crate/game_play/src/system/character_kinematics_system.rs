use amethyst::{assets::AssetStorage, ecs::prelude::*};
use game_input::ControllerInput;
use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{CharacterStatus, Kinematics},
    loaded::{Character, CharacterHandle},
};

/// Updates `Character` kinematics based on sequence.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct CharacterKinematicsSystem;

type CharacterKinematicsSystemData<'s> = (
    Read<'s, AssetStorage<Character>>,
    ReadStorage<'s, CharacterHandle>,
    ReadStorage<'s, ControllerInput>,
    ReadStorage<'s, CharacterStatus>,
    WriteStorage<'s, Kinematics<f32>>,
);

impl<'s> System<'s> for CharacterKinematicsSystem {
    type SystemData = CharacterKinematicsSystemData<'s>;

    fn run(
        &mut self,
        (
            characters,
            handle_storage,
            controller_input_storage,
            status_storage,
            mut kinematics_storage,
        ): Self::SystemData,
    ) {
        for (character_handle, controller_input, status, mut kinematics) in (
            &handle_storage,
            &controller_input_storage,
            &status_storage,
            &mut kinematics_storage,
        )
            .join()
        {
            // TODO: Character stats should be configuration.
            // Use the stats from the character definition.
            let _character = characters
                .get(character_handle)
                .expect("Expected character to be loaded.");

            match status.object_status.sequence_id {
                CharacterSequenceId::Stand | CharacterSequenceId::StandAttack => {
                    kinematics.velocity[0] = 0.;
                    kinematics.velocity[2] = 0.;
                }
                CharacterSequenceId::Walk => {
                    kinematics.velocity[0] = controller_input.x_axis_value as f32 * 3.5;
                    kinematics.velocity[2] = controller_input.z_axis_value as f32 * 2.;
                }
                CharacterSequenceId::Run => {
                    kinematics.velocity[0] = controller_input.x_axis_value as f32 * 6.;
                    kinematics.velocity[2] = controller_input.z_axis_value as f32 * 1.5;
                }
                CharacterSequenceId::RunStop => {
                    kinematics.velocity[0] = if status.object_status.mirrored {
                        -2.
                    } else {
                        2.
                    };
                    kinematics.velocity[2] = controller_input.z_axis_value as f32 * 0.5;
                }
                CharacterSequenceId::Jump => {}
                CharacterSequenceId::JumpOff => {
                    if status.object_status.sequence_state == SequenceState::Begin {
                        kinematics.velocity[0] = controller_input.x_axis_value as f32 * 5.;
                        kinematics.velocity[1] = 17.;
                        kinematics.velocity[2] = controller_input.z_axis_value as f32 * 2.;
                    }
                }
                CharacterSequenceId::JumpAscend => {}
                CharacterSequenceId::JumpDescend => {}
                CharacterSequenceId::JumpDescendLand => {
                    kinematics.velocity[0] /= 2.;
                    kinematics.velocity[1] = 0.;
                    kinematics.velocity[2] /= 2.;
                }
            };
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
        entity::{CharacterStatus, Grounding, Kinematics},
    };
    use typename::TypeName;

    use super::CharacterKinematicsSystem;

    #[test]
    fn stand_x_and_z_velocity_are_zero() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::game_base("stand_x_and_z_velocity_are_zero", false)
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
                                status.object_status.sequence_id = CharacterSequenceId::Stand;
                                status.object_status.grounding = Grounding::OnGround;

                                kinematics.position[1] = map.margins.bottom;
                                kinematics.velocity[0] = 3.;
                                kinematics.velocity[1] = 0.;
                                kinematics.velocity[2] = 3.;
                            }
                        },
                    );
                })
                .with_system_single(
                    CharacterKinematicsSystem::new(),
                    CharacterKinematicsSystem::type_name(),
                    &[]
                )
                .with_assertion(|world| {
                    world.exec(
                        |(status_storage, kinematics_storage): (
                            ReadStorage<CharacterStatus>,
                            ReadStorage<Kinematics<f32>>,
                        )| {
                            for (_, kinematics) in (&status_storage, &kinematics_storage).join() {
                                assert_eq!(0., kinematics.velocity[0]);
                                assert_eq!(0., kinematics.velocity[2]);
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
                                controller_input.x_axis_value = 1.;
                                controller_input.z_axis_value = -1.;

                                status.object_status.sequence_id = CharacterSequenceId::Walk;
                                status.object_status.grounding = Grounding::OnGround;

                                kinematics.position[1] = map.margins.bottom;
                                kinematics.velocity[0] = 0.;
                                kinematics.velocity[1] = 0.;
                                kinematics.velocity[2] = 0.;
                            }
                        },
                    );
                })
                .with_system_single(
                    CharacterKinematicsSystem::new(),
                    CharacterKinematicsSystem::type_name(),
                    &[]
                )
                .with_assertion(|world| {
                    world.exec(
                        |(status_storage, kinematics_storage): (
                            ReadStorage<CharacterStatus>,
                            ReadStorage<Kinematics<f32>>,
                        )| {
                            for (_, kinematics) in (&status_storage, &kinematics_storage).join() {
                                assert_eq!(3.5, kinematics.velocity[0]);
                                assert_eq!(-2., kinematics.velocity[2]);
                            }
                        },
                    );
                })
                .run()
                .is_ok()
        );
    }

    #[test]
    fn updates_run_x_and_z_velocity() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::game_base("updates_run_x_and_z_velocity", false)
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
                                controller_input.x_axis_value = 1.;
                                controller_input.z_axis_value = -1.;

                                status.object_status.sequence_id = CharacterSequenceId::Run;
                                status.object_status.grounding = Grounding::OnGround;

                                kinematics.position[1] = map.margins.bottom;
                                kinematics.velocity[0] = 0.;
                                kinematics.velocity[1] = 0.;
                                kinematics.velocity[2] = 0.;
                            }
                        },
                    );
                })
                .with_system_single(
                    CharacterKinematicsSystem::new(),
                    CharacterKinematicsSystem::type_name(),
                    &[]
                )
                .with_assertion(|world| {
                    world.exec(
                        |(status_storage, kinematics_storage): (
                            ReadStorage<CharacterStatus>,
                            ReadStorage<Kinematics<f32>>,
                        )| {
                            for (_, kinematics) in (&status_storage, &kinematics_storage).join() {
                                assert_eq!(6., kinematics.velocity[0]);
                                assert_eq!(-1.5, kinematics.velocity[2]);
                            }
                        },
                    );
                })
                .run()
                .is_ok()
        );
    }

    #[test]
    fn updates_run_stop_x_and_z_velocity() {
        vec![(false, 2.), (true, -2.)]
            .into_iter()
            .for_each(|(mirrored, vx)| {
                let setup_fn = move |world: &mut World| {
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
                                controller_input.z_axis_value = 1.;

                                status.object_status.sequence_id = CharacterSequenceId::RunStop;
                                status.object_status.grounding = Grounding::OnGround;
                                status.object_status.mirrored = mirrored;

                                kinematics.position[1] = map.margins.bottom;
                                kinematics.velocity[0] = 0.;
                                kinematics.velocity[1] = 0.;
                                kinematics.velocity[2] = 0.;
                            }
                        },
                    );
                };

                let assertion_fn = move |world: &mut World| {
                    world.exec(
                        |(status_storage, kinematics_storage): (
                            ReadStorage<CharacterStatus>,
                            ReadStorage<Kinematics<f32>>,
                        )| {
                            for (_, kinematics) in (&status_storage, &kinematics_storage).join() {
                                assert_eq!(vx, kinematics.velocity[0]);
                                assert_eq!(0.5, kinematics.velocity[2]);
                            }
                        },
                    );
                };

                // kcov-ignore-start
                assert!(
                    // kcov-ignore-end
                    AutexousiousApplication::game_base(
                        "updates_run_stop_x_and_z_velocity_non_mirrored",
                        false
                    )
                    .with_setup(setup_fn)
                    .with_system_single(
                        CharacterKinematicsSystem::new(),
                        CharacterKinematicsSystem::type_name(),
                        &[]
                    )
                    .with_assertion(assertion_fn)
                    .run()
                    .is_ok()
                );
            });
    }

    #[test]
    fn updates_jump_off_xyz_velocity() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::game_base("updates_jump_off_xyz_velocity", false)
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
                                controller_input.z_axis_value = 1.;

                                status.object_status.sequence_id = CharacterSequenceId::JumpOff;
                                status.object_status.grounding = Grounding::OnGround;

                                kinematics.position[1] = map.margins.bottom;
                                kinematics.velocity[0] = 0.;
                                kinematics.velocity[1] = 0.;
                                kinematics.velocity[2] = 0.;
                            }
                        },
                    );
                })
                .with_system_single(
                    CharacterKinematicsSystem::new(),
                    CharacterKinematicsSystem::type_name(),
                    &[]
                )
                .with_assertion(|world| {
                    world.exec(
                        |(status_storage, kinematics_storage): (
                            ReadStorage<CharacterStatus>,
                            ReadStorage<Kinematics<f32>>,
                        )| {
                            for (_, kinematics) in (&status_storage, &kinematics_storage).join() {
                                assert_eq!(-5., kinematics.velocity[0]);
                                assert_eq!(17., kinematics.velocity[1]);
                                assert_eq!(2., kinematics.velocity[2]);
                            }
                        },
                    );
                })
                .run()
                .is_ok()
        );
    }

    #[test]
    fn updates_jump_descend_land_xyz_velocity() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::game_base("updates_jump_descend_land_xyz_velocity", false)
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
                                status.object_status.sequence_id =
                                    CharacterSequenceId::JumpDescendLand;
                                status.object_status.grounding = Grounding::Airborne;

                                kinematics.position[1] = map.margins.bottom;
                                kinematics.velocity[0] = -6.;
                                kinematics.velocity[1] = -10.;
                                kinematics.velocity[2] = -4.;
                            }
                        },
                    );
                })
                .with_system_single(
                    CharacterKinematicsSystem::new(),
                    CharacterKinematicsSystem::type_name(),
                    &[]
                )
                .with_assertion(|world| {
                    world.exec(
                        |(status_storage, kinematics_storage): (
                            ReadStorage<CharacterStatus>,
                            ReadStorage<Kinematics<f32>>,
                        )| {
                            for (_, kinematics) in (&status_storage, &kinematics_storage).join() {
                                assert_eq!(-3., kinematics.velocity[0]);
                                assert_eq!(0., kinematics.velocity[1]);
                                assert_eq!(-2., kinematics.velocity[2]);
                            }
                        },
                    );
                })
                .run()
                .is_ok()
        );
    }
}
