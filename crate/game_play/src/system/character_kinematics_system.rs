use amethyst::{
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    shred::Resources,
    shrev::{EventChannel, ReaderId},
};
use character_model::config::CharacterSequenceId;
use derive_new::new;
use game_input::ControllerInput;
use kinematic_model::config::Velocity;
use object_model::play::Mirrored;
use sequence_model::play::SequenceUpdateEvent;
use typename_derive::TypeName;

/// Updates `Character` velocity based on sequence.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct CharacterKinematicsSystem {
    /// Reader ID for the `SequenceUpdateEvent` event channel.
    #[new(default)]
    sequence_update_event_rid: Option<ReaderId<SequenceUpdateEvent>>,
}

type CharacterKinematicsSystemData<'s> = (
    Read<'s, EventChannel<SequenceUpdateEvent>>,
    ReadStorage<'s, ControllerInput>,
    ReadStorage<'s, CharacterSequenceId>,
    WriteStorage<'s, Velocity<f32>>,
    WriteStorage<'s, Mirrored>,
);

impl<'s> System<'s> for CharacterKinematicsSystem {
    type SystemData = CharacterKinematicsSystemData<'s>;

    fn run(
        &mut self,
        (
            sequence_update_ec,
            controller_inputs,
            character_sequence_ids,
            mut velocities,
            mut mirroreds,
        ): Self::SystemData,
    ) {
        for (controller_input, character_sequence_id, velocity, mirrored) in (
            &controller_inputs,
            &character_sequence_ids,
            &mut velocities,
            &mut mirroreds,
        )
            .join()
        {
            // TODO: Character stats should be configuration.
            // Use a stats component from the character definition.

            match character_sequence_id {
                CharacterSequenceId::Stand
                | CharacterSequenceId::Flinch0
                | CharacterSequenceId::Flinch1 => {
                    velocity[0] = 0.;
                    velocity[2] = 0.;
                }
                CharacterSequenceId::Walk => {
                    velocity[0] = controller_input.x_axis_value as f32 * 3.5;
                    velocity[2] = controller_input.z_axis_value as f32 * 2.;
                }
                CharacterSequenceId::Run => {
                    velocity[0] = controller_input.x_axis_value as f32 * 6.;
                    velocity[2] = controller_input.z_axis_value as f32 * 1.5;
                }
                CharacterSequenceId::RunStop => {
                    velocity[0] = if mirrored.0 { -2. } else { 2. };
                    velocity[2] = controller_input.z_axis_value as f32 * 0.5;
                }
                // TODO: velocity as config
                CharacterSequenceId::Dodge => {
                    velocity[0] = if mirrored.0 { -3. } else { 3. };
                    velocity[2] = controller_input.z_axis_value as f32;
                }
                CharacterSequenceId::JumpDescendLand
                | CharacterSequenceId::DashDescendLand
                | CharacterSequenceId::FallForwardLand
                | CharacterSequenceId::LieFaceDown
                | CharacterSequenceId::StandAttack0
                | CharacterSequenceId::StandAttack1 => {
                    velocity[0] /= 2.;
                    velocity[1] = 0.;
                    velocity[2] /= 2.;
                }
                _ => {}
            };
        }

        sequence_update_ec
            .read(
                self.sequence_update_event_rid
                    .as_mut()
                    .expect("Expected `sequence_update_event_rid` to exist."),
            )
            .for_each(|ev| {
                if let SequenceUpdateEvent::SequenceBegin { entity } = ev {
                    if let (
                        Some(character_sequence_id),
                        Some(mirrored),
                        Some(controller_input),
                        Some(velocity),
                    ) = (
                        character_sequence_ids.get(*entity),
                        mirroreds.get(*entity),
                        controller_inputs.get(*entity),
                        velocities.get_mut(*entity),
                    ) {
                        match character_sequence_id {
                            CharacterSequenceId::StandAttack0 => {
                                velocity[0] = controller_input.x_axis_value as f32 * 2.5;
                                velocity[2] = controller_input.z_axis_value as f32 * 1.5;
                            }
                            CharacterSequenceId::StandAttack1 => {
                                velocity[0] = controller_input.x_axis_value as f32 * 2.5;
                                velocity[2] = controller_input.z_axis_value as f32 * 1.5;
                            }
                            CharacterSequenceId::JumpOff => {
                                velocity[0] = controller_input.x_axis_value as f32 * 5.;
                                velocity[1] = 10.5;
                                velocity[2] = controller_input.z_axis_value as f32 * 2.;
                            }
                            CharacterSequenceId::DashForward => {
                                velocity[0] = if mirrored.0 { -8. } else { 8. };
                                velocity[1] = 7.5;
                                velocity[2] = controller_input.z_axis_value as f32 * 2.5;
                            }
                            CharacterSequenceId::DashBack => {
                                velocity[0] = if mirrored.0 { 11. } else { -11. };
                                velocity[1] = 7.5;
                                velocity[2] = controller_input.z_axis_value as f32 * 2.5;
                            }
                            _ => {}
                        };
                    }
                }
            });
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.sequence_update_event_rid = Some(
            res.fetch_mut::<EventChannel<SequenceUpdateEvent>>()
                .register_reader(),
        );
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        assets::AssetStorage,
        ecs::{Entities, Entity, Join, Read, ReadExpect, ReadStorage, Write, WriteStorage},
        shrev::EventChannel,
        Error,
    };
    use application_test_support::AutexousiousApplication;
    use character_model::config::CharacterSequenceId;
    use game_input::ControllerInput;
    use kinematic_model::config::{Position, Velocity};
    use map_model::loaded::Map;
    use map_selection_model::MapSelection;
    use object_model::play::{Grounding, Mirrored};
    use sequence_model::play::SequenceUpdateEvent;
    use typename::TypeName;

    use super::CharacterKinematicsSystem;

    #[test]
    fn stand_x_and_z_velocity_are_zero() -> Result<(), Error> {
        run_test(
            ParamsSetup {
                velocity: Velocity::new(3., 0., 3.),
                grounding: Grounding::OnGround,
                sequence_id: CharacterSequenceId::Stand,
                controller_input: None,
                mirrored: None,
                event_fn: None,
            },
            Velocity::new(0., 0., 0.),
        )
    }

    #[test]
    fn updates_stand_attack_x_and_z_velocity() -> Result<(), Error> {
        vec![
            CharacterSequenceId::StandAttack0,
            CharacterSequenceId::StandAttack1,
        ]
        .into_iter()
        .try_for_each(|stand_attack_id| {
            let mut controller_input = ControllerInput::default();
            controller_input.x_axis_value = 1.;
            controller_input.z_axis_value = -1.;

            run_test(
                ParamsSetup {
                    velocity: Velocity::new(0., 0., 0.),
                    grounding: Grounding::OnGround,
                    sequence_id: stand_attack_id,
                    controller_input: Some(controller_input),
                    mirrored: None,
                    event_fn: Some(|entity| SequenceUpdateEvent::SequenceBegin { entity }),
                },
                Velocity::new(2.5, 0., -1.5),
            )
        })
    }

    #[test]
    fn updates_walk_x_and_z_velocity() -> Result<(), Error> {
        let mut controller_input = ControllerInput::default();
        controller_input.x_axis_value = 1.;
        controller_input.z_axis_value = -1.;

        run_test(
            ParamsSetup {
                velocity: Velocity::new(0., 0., 0.),
                grounding: Grounding::OnGround,
                sequence_id: CharacterSequenceId::Walk,
                controller_input: Some(controller_input),
                mirrored: None,
                event_fn: None,
            },
            Velocity::new(3.5, 0., -2.),
        )
    }

    #[test]
    fn updates_run_x_and_z_velocity() -> Result<(), Error> {
        let mut controller_input = ControllerInput::default();
        controller_input.x_axis_value = 1.;
        controller_input.z_axis_value = -1.;

        run_test(
            ParamsSetup {
                velocity: Velocity::new(0., 0., 0.),
                grounding: Grounding::OnGround,
                sequence_id: CharacterSequenceId::Run,
                controller_input: Some(controller_input),
                mirrored: None,
                event_fn: None,
            },
            Velocity::new(6., 0., -1.5),
        )
    }

    #[test]
    fn updates_run_stop_x_and_z_velocity() -> Result<(), Error> {
        vec![(false, 2.), (true, -2.)].into_iter().try_for_each(
            |(mirrored_bool, vx)| -> Result<(), Error> {
                let mut controller_input = ControllerInput::default();
                controller_input.z_axis_value = 1.;

                run_test(
                    ParamsSetup {
                        velocity: Velocity::new(0., 0., 0.),
                        grounding: Grounding::OnGround,
                        sequence_id: CharacterSequenceId::RunStop,
                        controller_input: Some(controller_input),
                        mirrored: Some(mirrored_bool.into()),
                        event_fn: None,
                    },
                    Velocity::new(vx, 0., 0.5),
                )
            },
        )
    }

    #[test]
    fn updates_dodge_x_and_z_velocity() -> Result<(), Error> {
        vec![(false, 3.), (true, -3.)].into_iter().try_for_each(
            |(mirrored_bool, vx)| -> Result<(), Error> {
                let mut controller_input = ControllerInput::default();
                controller_input.z_axis_value = 1.;

                run_test(
                    ParamsSetup {
                        velocity: Velocity::new(0., 0., 0.),
                        grounding: Grounding::OnGround,
                        sequence_id: CharacterSequenceId::Dodge,
                        controller_input: Some(controller_input),
                        mirrored: Some(mirrored_bool.into()),
                        event_fn: None,
                    },
                    Velocity::new(vx, 0., 1.),
                )
            },
        )
    }

    #[test]
    fn updates_jump_off_xyz_velocity() -> Result<(), Error> {
        let mut controller_input = ControllerInput::default();
        controller_input.x_axis_value = -1.;
        controller_input.z_axis_value = 1.;

        run_test(
            ParamsSetup {
                velocity: Velocity::new(0., 0., 0.),
                grounding: Grounding::OnGround,
                sequence_id: CharacterSequenceId::JumpOff,
                controller_input: Some(controller_input),
                mirrored: None,
                event_fn: Some(|entity| SequenceUpdateEvent::SequenceBegin { entity }),
            },
            Velocity::new(-5., 10.5, 2.),
        )
    }

    #[test]
    fn updates_dash_forward_xyz_velocity() -> Result<(), Error> {
        let mut controller_input = ControllerInput::default();
        controller_input.z_axis_value = 1.;

        run_test(
            ParamsSetup {
                velocity: Velocity::new(0., 0., 0.),
                grounding: Grounding::OnGround,
                sequence_id: CharacterSequenceId::DashForward,
                controller_input: Some(controller_input),
                mirrored: None,
                event_fn: Some(|entity| SequenceUpdateEvent::SequenceBegin { entity }),
            },
            Velocity::new(8., 7.5, 2.5),
        )
    }

    #[test]
    fn updates_dash_back_xyz_velocity() -> Result<(), Error> {
        let mut controller_input = ControllerInput::default();
        controller_input.z_axis_value = 1.;

        run_test(
            ParamsSetup {
                velocity: Velocity::new(0., 0., 0.),
                grounding: Grounding::OnGround,
                sequence_id: CharacterSequenceId::DashBack,
                controller_input: Some(controller_input),
                mirrored: None,
                event_fn: Some(|entity| SequenceUpdateEvent::SequenceBegin { entity }),
            },
            Velocity::new(-11., 7.5, 2.5),
        )
    }

    #[test]
    fn updates_jump_descend_land_xyz_velocity() -> Result<(), Error> {
        run_test(
            ParamsSetup {
                velocity: Velocity::new(-6., -10., -4.),
                grounding: Grounding::OnGround,
                sequence_id: CharacterSequenceId::JumpDescendLand,
                controller_input: None,
                mirrored: None,
                event_fn: None,
            },
            Velocity::new(-3., 0., -2.),
        )
    }

    #[test]
    fn updates_fall_forward_land_xyz_velocity() -> Result<(), Error> {
        run_test(
            ParamsSetup {
                velocity: Velocity::new(-6., -10., -4.),
                grounding: Grounding::OnGround,
                sequence_id: CharacterSequenceId::FallForwardLand,
                controller_input: None,
                mirrored: None,
                event_fn: None,
            },
            Velocity::new(-3., 0., -2.),
        )
    }

    #[test]
    fn updates_lie_face_down_xyz_velocity() -> Result<(), Error> {
        run_test(
            ParamsSetup {
                velocity: Velocity::new(-6., -10., -4.),
                grounding: Grounding::OnGround,
                sequence_id: CharacterSequenceId::LieFaceDown,
                controller_input: None,
                mirrored: None,
                event_fn: None,
            },
            Velocity::new(-3., 0., -2.),
        )
    }

    fn run_test(
        ParamsSetup {
            velocity: velocity_setup,
            grounding: grounding_setup,
            sequence_id: sequence_id_setup,
            controller_input: controller_input_setup,
            mirrored: mirrored_setup,
            event_fn,
        }: ParamsSetup,
        velocity_expected: Velocity<f32>,
    ) -> Result<(), Error> {
        AutexousiousApplication::game_base()
            // kcov-ignore-start
            .with_system(
                CharacterKinematicsSystem::new(),
                CharacterKinematicsSystem::type_name(),
                &[],
            )
            // kcov-ignore-end
            .with_setup(move |world| {
                let (
                    entities,
                    map_selection,
                    maps,
                    mut character_sequence_ids,
                    mut positions,
                    mut velocities,
                    mut groundings,
                    mut controller_inputs,
                    mut mirroreds,
                    mut event_channel,
                ) = world.system_data::<TestSystemData>();
                let map = maps
                    .get(map_selection.handle())
                    .expect("Expected map to be loaded.");

                for (
                    entity,
                    character_sequence_id,
                    position,
                    velocity,
                    grounding,
                    controller_input,
                    mirrored,
                ) in (
                    &entities,
                    &mut character_sequence_ids,
                    &mut positions,
                    &mut velocities,
                    &mut groundings,
                    &mut controller_inputs,
                    &mut mirroreds,
                )
                    .join()
                {
                    *character_sequence_id = sequence_id_setup;
                    *grounding = grounding_setup;

                    position[1] = map.margins.bottom;
                    *velocity = velocity_setup;

                    if let Some(controller_input_setup) = controller_input_setup {
                        *controller_input = controller_input_setup;
                    }
                    if let Some(mirrored_setup) = mirrored_setup {
                        *mirrored = mirrored_setup;
                    }
                    if let Some(event_fn) = event_fn {
                        event_channel.single_write(event_fn(entity));
                    }
                }
            })
            .with_assertion(move |world| {
                world.exec(
                    |(character_sequence_ids, velocities): (
                        ReadStorage<'_, CharacterSequenceId>,
                        ReadStorage<'_, Velocity<f32>>,
                    )| {
                        for (_, velocity_actual) in (&character_sequence_ids, &velocities).join() {
                            assert_eq!(&velocity_expected, velocity_actual);
                        }
                    },
                );
            })
            .run_isolated()
    }

    #[derive(Debug)]
    struct ParamsSetup {
        velocity: Velocity<f32>,
        sequence_id: CharacterSequenceId,
        grounding: Grounding,
        controller_input: Option<ControllerInput>,
        mirrored: Option<Mirrored>,
        event_fn: Option<fn(entity: Entity) -> SequenceUpdateEvent>,
    }

    type TestSystemData<'s> = (
        Entities<'s>,
        ReadExpect<'s, MapSelection>,
        Read<'s, AssetStorage<Map>>,
        WriteStorage<'s, CharacterSequenceId>,
        WriteStorage<'s, Position<f32>>,
        WriteStorage<'s, Velocity<f32>>,
        WriteStorage<'s, Grounding>,
        WriteStorage<'s, ControllerInput>,
        WriteStorage<'s, Mirrored>,
        Write<'s, EventChannel<SequenceUpdateEvent>>,
    );
}
