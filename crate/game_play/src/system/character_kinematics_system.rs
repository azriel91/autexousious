use amethyst::{assets::AssetStorage, ecs::prelude::*};
use game_input::ControllerInput;
use object_model::{
    config::object::{CharacterSequenceId, SequenceStatus},
    entity::{Kinematics, Mirrored, ObjectStatus},
    loaded::{Character, CharacterHandle},
};

/// Updates `Character` kinematics based on sequence.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct CharacterKinematicsSystem;

type CharacterKinematicsSystemData<'s> = (
    Read<'s, AssetStorage<Character>>,
    ReadStorage<'s, CharacterHandle>,
    ReadStorage<'s, ControllerInput>,
    ReadStorage<'s, ObjectStatus<CharacterSequenceId>>,
    WriteStorage<'s, Kinematics<f32>>,
    WriteStorage<'s, Mirrored>,
);

impl<'s> System<'s> for CharacterKinematicsSystem {
    type SystemData = CharacterKinematicsSystemData<'s>;

    fn run(
        &mut self,
        (
            characters,
            handle_storage,
            controller_inputs,
            object_statuses,
            mut kinematicses,
            mut mirroreds,
        ): Self::SystemData,
    ) {
        for (character_handle, controller_input, object_status, mut kinematics, mut mirrored) in (
            &handle_storage,
            &controller_inputs,
            &object_statuses,
            &mut kinematicses,
            &mut mirroreds,
        )
            .join()
        {
            // TODO: Character stats should be configuration.
            // Use the stats from the character definition.
            let _character = characters
                .get(character_handle)
                .expect("Expected character to be loaded.");

            match object_status.sequence_id {
                CharacterSequenceId::Stand
                | CharacterSequenceId::StandAttack
                | CharacterSequenceId::Flinch0
                | CharacterSequenceId::Flinch1 => {
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
                    kinematics.velocity[0] = if mirrored.0 { -2. } else { 2. };
                    kinematics.velocity[2] = controller_input.z_axis_value as f32 * 0.5;
                }
                CharacterSequenceId::JumpOff => {
                    if object_status.sequence_status == SequenceStatus::Begin {
                        kinematics.velocity[0] = controller_input.x_axis_value as f32 * 5.;
                        kinematics.velocity[1] = 17.;
                        kinematics.velocity[2] = controller_input.z_axis_value as f32 * 2.;
                    }
                }
                CharacterSequenceId::JumpDescendLand
                | CharacterSequenceId::FallForwardLand
                | CharacterSequenceId::LieFaceDown => {
                    kinematics.velocity[0] /= 2.;
                    kinematics.velocity[1] = 0.;
                    kinematics.velocity[2] /= 2.;
                }
                CharacterSequenceId::Jump
                | CharacterSequenceId::JumpAscend
                | CharacterSequenceId::JumpDescend
                | CharacterSequenceId::FallForwardAscend
                | CharacterSequenceId::FallForwardDescend => {}
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
        entity::{Grounding, Kinematics, Mirrored, ObjectStatus},
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
                                *grounding = Grounding::OnGround;

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
                        |(object_statuses, kinematicses): (
                            ReadStorage<ObjectStatus<CharacterSequenceId>>,
                            ReadStorage<Kinematics<f32>>,
                        )| {
                            for (_, kinematics) in (&object_statuses, &kinematicses).join() {
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
                            mut controller_inputs,
                            mut object_statuses,
                            mut kinematicses,
                            mut groundings,
                        ): (
                            ReadExpect<MapSelection>,
                            Read<AssetStorage<Map>>,
                            WriteStorage<ControllerInput>,
                            WriteStorage<ObjectStatus<CharacterSequenceId>>,
                            WriteStorage<Kinematics<f32>>,
                            WriteStorage<Grounding>,
                        )| {
                            let map = maps
                                .get(map_selection.handle())
                                .expect("Expected map to be loaded.");

                            for (controller_input, object_status, kinematics, grounding) in (
                                &mut controller_inputs,
                                &mut object_statuses,
                                &mut kinematicses,
                                &mut groundings,
                            )
                                .join()
                            {
                                controller_input.x_axis_value = 1.;
                                controller_input.z_axis_value = -1.;

                                object_status.sequence_id = CharacterSequenceId::Walk;
                                *grounding = Grounding::OnGround;

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
                        |(object_statuses, kinematicses): (
                            ReadStorage<ObjectStatus<CharacterSequenceId>>,
                            ReadStorage<Kinematics<f32>>,
                        )| {
                            for (_, kinematics) in (&object_statuses, &kinematicses).join() {
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
                            mut controller_inputs,
                            mut object_statuses,
                            mut kinematicses,
                            mut groundings,
                        ): (
                            ReadExpect<MapSelection>,
                            Read<AssetStorage<Map>>,
                            WriteStorage<ControllerInput>,
                            WriteStorage<ObjectStatus<CharacterSequenceId>>,
                            WriteStorage<Kinematics<f32>>,
                            WriteStorage<Grounding>,
                        )| {
                            let map = maps
                                .get(map_selection.handle())
                                .expect("Expected map to be loaded.");

                            for (controller_input, object_status, kinematics, grounding) in (
                                &mut controller_inputs,
                                &mut object_statuses,
                                &mut kinematicses,
                                &mut groundings,
                            )
                                .join()
                            {
                                controller_input.x_axis_value = 1.;
                                controller_input.z_axis_value = -1.;

                                object_status.sequence_id = CharacterSequenceId::Run;
                                *grounding = Grounding::OnGround;

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
                        |(object_statuses, kinematicses): (
                            ReadStorage<ObjectStatus<CharacterSequenceId>>,
                            ReadStorage<Kinematics<f32>>,
                        )| {
                            for (_, kinematics) in (&object_statuses, &kinematicses).join() {
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
            .for_each(|(mirrored_bool, vx)| {
                let setup_fn = move |world: &mut World| {
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
                                controller_input.z_axis_value = 1.;

                                object_status.sequence_id = CharacterSequenceId::RunStop;
                                *grounding = Grounding::OnGround;
                                *mirrored = mirrored_bool.into();

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
                        |(object_statuses, kinematicses): (
                            ReadStorage<ObjectStatus<CharacterSequenceId>>,
                            ReadStorage<Kinematics<f32>>,
                        )| {
                            for (_, kinematics) in (&object_statuses, &kinematicses).join() {
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
                            mut controller_inputs,
                            mut object_statuses,
                            mut kinematicses,
                            mut groundings,
                        ): (
                            ReadExpect<MapSelection>,
                            Read<AssetStorage<Map>>,
                            WriteStorage<ControllerInput>,
                            WriteStorage<ObjectStatus<CharacterSequenceId>>,
                            WriteStorage<Kinematics<f32>>,
                            WriteStorage<Grounding>,
                        )| {
                            let map = maps
                                .get(map_selection.handle())
                                .expect("Expected map to be loaded.");

                            for (controller_input, object_status, kinematics, grounding) in (
                                &mut controller_inputs,
                                &mut object_statuses,
                                &mut kinematicses,
                                &mut groundings,
                            )
                                .join()
                            {
                                controller_input.x_axis_value = -1.;
                                controller_input.z_axis_value = 1.;

                                object_status.sequence_id = CharacterSequenceId::JumpOff;
                                *grounding = Grounding::OnGround;

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
                        |(object_statuses, kinematicses): (
                            ReadStorage<ObjectStatus<CharacterSequenceId>>,
                            ReadStorage<Kinematics<f32>>,
                        )| {
                            for (_, kinematics) in (&object_statuses, &kinematicses).join() {
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
                                object_status.sequence_id = CharacterSequenceId::JumpDescendLand;
                                *grounding = Grounding::Airborne;

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
                        |(object_statuses, kinematicses): (
                            ReadStorage<ObjectStatus<CharacterSequenceId>>,
                            ReadStorage<Kinematics<f32>>,
                        )| {
                            for (_, kinematics) in (&object_statuses, &kinematicses).join() {
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

    #[test]
    fn updates_fall_forward_land_xyz_velocity() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::game_base("updates_fall_forward_land_xyz_velocity", false)
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
                                object_status.sequence_id = CharacterSequenceId::FallForwardLand;
                                *grounding = Grounding::Airborne;

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
                        |(object_statuses, kinematicses): (
                            ReadStorage<ObjectStatus<CharacterSequenceId>>,
                            ReadStorage<Kinematics<f32>>,
                        )| {
                            for (_, kinematics) in (&object_statuses, &kinematicses).join() {
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

    #[test]
    fn updates_lie_face_down_xyz_velocity() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::game_base("updates_lie_face_down_xyz_velocity", false)
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
                                object_status.sequence_id = CharacterSequenceId::LieFaceDown;
                                *grounding = Grounding::OnGround;

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
                        |(object_statuses, kinematicses): (
                            ReadStorage<ObjectStatus<CharacterSequenceId>>,
                            ReadStorage<Kinematics<f32>>,
                        )| {
                            for (_, kinematics) in (&object_statuses, &kinematicses).join() {
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
