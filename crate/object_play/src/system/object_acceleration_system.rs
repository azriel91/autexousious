use amethyst::{
    ecs::{Join, Read, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use game_input::ControllerInput;
use kinematic_model::config::{
    ObjectAcceleration, ObjectAccelerationKind, ObjectAccelerationValue,
    ObjectAccelerationValueExpr, ObjectAccelerationValueMultiplier, Velocity,
};
use object_model::play::Mirrored;
use sequence_model::play::SequenceUpdateEvent;
use typename_derive::TypeName;

/// Increases velocity of `Object`s based on their `ObjectAcceleration`.
#[derive(Debug, Default, TypeName, new)]
pub struct ObjectAccelerationSystem {
    /// Reader ID for the `SequenceUpdateEvent` event channel.
    #[new(default)]
    sequence_update_event_rid: Option<ReaderId<SequenceUpdateEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ObjectAccelerationSystemData<'s> {
    /// `SequenceUpdateEvent` channel.
    #[derivative(Debug = "ignore")]
    pub sequence_update_ec: Read<'s, EventChannel<SequenceUpdateEvent>>,
    /// `ControllerInput` components.
    #[derivative(Debug = "ignore")]
    pub controller_inputs: ReadStorage<'s, ControllerInput>,
    /// `Mirrored` components.
    #[derivative(Debug = "ignore")]
    pub mirroreds: ReadStorage<'s, Mirrored>,
    /// `ObjectAcceleration` components.
    #[derivative(Debug = "ignore")]
    pub object_accelerations: ReadStorage<'s, ObjectAcceleration>,
    /// `Velocity<f32>` components.
    #[derivative(Debug = "ignore")]
    pub velocities: WriteStorage<'s, Velocity<f32>>,
}

impl ObjectAccelerationSystem {
    fn update_velocity(
        controller_input: Option<ControllerInput>,
        mirrored: Option<Mirrored>,
        object_acceleration: ObjectAcceleration,
        velocity: &mut Velocity<f32>,
    ) {
        let negate = mirrored.map(|mirrored| mirrored.0).unwrap_or(false);
        let acc_x = Self::acceleration_value(controller_input, object_acceleration.x);
        if negate {
            velocity[0] -= acc_x;
        } else {
            velocity[0] += acc_x;
        }
        velocity[1] += Self::acceleration_value(controller_input, object_acceleration.y);
        velocity[2] += Self::acceleration_value(controller_input, object_acceleration.z);
    }

    fn acceleration_value(
        controller_input: Option<ControllerInput>,
        object_acceleration_value: ObjectAccelerationValue,
    ) -> f32 {
        match object_acceleration_value {
            ObjectAccelerationValue::Const(value) => value,
            ObjectAccelerationValue::Expr(ObjectAccelerationValueExpr { multiplier, value }) => {
                match multiplier {
                    ObjectAccelerationValueMultiplier::One => value,
                    ObjectAccelerationValueMultiplier::XAxis => {
                        let multiplier = controller_input
                            .map(|controller_input| controller_input.x_axis_value.abs())
                            .unwrap_or(0.);
                        multiplier * value
                    }
                    ObjectAccelerationValueMultiplier::ZAxis => {
                        let multiplier = controller_input
                            .map(|controller_input| controller_input.z_axis_value)
                            .unwrap_or(0.);
                        multiplier * value
                    }
                }
            }
        }
    }
}

impl<'s> System<'s> for ObjectAccelerationSystem {
    type SystemData = ObjectAccelerationSystemData<'s>;

    fn run(
        &mut self,
        ObjectAccelerationSystemData {
            sequence_update_ec,
            controller_inputs,
            mirroreds,
            object_accelerations,
            mut velocities,
        }: Self::SystemData,
    ) {
        sequence_update_ec
            .read(
                self.sequence_update_event_rid
                    .as_mut()
                    .expect("Expected `sequence_update_event_rid` to exist."),
            )
            .for_each(|ev| {
                if let SequenceUpdateEvent::SequenceBegin { entity, .. }
                | SequenceUpdateEvent::FrameBegin { entity, .. } = ev
                {
                    let entity = *entity;
                    let object_acceleration = object_accelerations.get(entity);
                    let velocity = velocities.get_mut(entity);
                    let controller_input = controller_inputs.get(entity).copied();
                    let mirrored = mirroreds.get(entity).copied();

                    if let (Some(object_acceleration), Some(velocity)) =
                        (object_acceleration, velocity)
                    {
                        if object_acceleration.kind == ObjectAccelerationKind::Once {
                            Self::update_velocity(
                                controller_input,
                                mirrored,
                                *object_acceleration,
                                velocity,
                            );
                        }
                    }
                }
            });

        (
            &object_accelerations,
            &mut velocities,
            controller_inputs.maybe(),
            mirroreds.maybe(),
        )
            .join()
            .filter(|(object_acceleration, _, _, _)| {
                object_acceleration.kind == ObjectAccelerationKind::Continuous
            })
            .for_each(
                |(object_acceleration, velocity, controller_input, mirrored)| {
                    Self::update_velocity(
                        controller_input.copied(),
                        mirrored.copied(),
                        *object_acceleration,
                        velocity,
                    );
                },
            );
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.sequence_update_event_rid = Some(
            world
                .fetch_mut::<EventChannel<SequenceUpdateEvent>>()
                .register_reader(),
        );
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entity, WorldExt},
        shrev::EventChannel,
        Error,
    };
    use amethyst_test::AmethystApplication;
    use game_input::ControllerInput;
    use kinematic_model::config::{
        ObjectAcceleration, ObjectAccelerationKind, ObjectAccelerationValue,
        ObjectAccelerationValueExpr, ObjectAccelerationValueMultiplier, Velocity,
    };
    use object_model::play::Mirrored;
    use sequence_model::{loaded::SequenceId, play::SequenceUpdateEvent};
    use typename::TypeName;

    use super::ObjectAccelerationSystem;

    #[test]
    fn increases_velocity_for_continuous_acceleration() -> Result<(), Error> {
        let object_acceleration = ObjectAcceleration {
            kind: ObjectAccelerationKind::Continuous,
            x: ObjectAccelerationValue::Const(1.),
            y: ObjectAccelerationValue::Const(2.),
            z: ObjectAccelerationValue::Const(3.),
        };
        let velocity = Velocity::new(10., 20., 30.);

        run_test(
            SetupParams {
                controller_input: None,
                mirrored: None,
                object_acceleration,
                velocity,
                sequence_update_event_fn: None,
            },
            ExpectedParams {
                velocity: Velocity::new(11., 22., 33.),
            },
        )
    }

    #[test]
    fn increases_velocity_for_once_acceleration_on_sequence_begin_event() -> Result<(), Error> {
        let object_acceleration = ObjectAcceleration {
            kind: ObjectAccelerationKind::Once,
            x: ObjectAccelerationValue::Const(1.),
            y: ObjectAccelerationValue::Const(2.),
            z: ObjectAccelerationValue::Const(3.),
        };
        let velocity = Velocity::new(10., 20., 30.);

        run_test(
            SetupParams {
                controller_input: None,
                mirrored: None,
                object_acceleration,
                velocity,
                sequence_update_event_fn: Some(sequence_begin_event),
            },
            ExpectedParams {
                velocity: Velocity::new(11., 22., 33.),
            },
        )
    }

    #[test]
    fn increases_velocity_for_once_acceleration_on_frame_begin_event() -> Result<(), Error> {
        let object_acceleration = ObjectAcceleration {
            kind: ObjectAccelerationKind::Once,
            x: ObjectAccelerationValue::Const(1.),
            y: ObjectAccelerationValue::Const(2.),
            z: ObjectAccelerationValue::Const(3.),
        };
        let velocity = Velocity::new(10., 20., 30.);

        run_test(
            SetupParams {
                controller_input: None,
                mirrored: None,
                object_acceleration,
                velocity,
                sequence_update_event_fn: Some(frame_begin_event),
            },
            ExpectedParams {
                velocity: Velocity::new(11., 22., 33.),
            },
        )
    }

    #[test]
    fn does_not_increase_velocity_for_once_acceleration_on_sequence_end_event() -> Result<(), Error>
    {
        let object_acceleration = ObjectAcceleration {
            kind: ObjectAccelerationKind::Once,
            x: ObjectAccelerationValue::Const(1.),
            y: ObjectAccelerationValue::Const(2.),
            z: ObjectAccelerationValue::Const(3.),
        };
        let velocity = Velocity::new(10., 20., 30.);

        run_test(
            SetupParams {
                controller_input: None,
                mirrored: None,
                object_acceleration,
                velocity,
                sequence_update_event_fn: Some(sequence_end_event),
            },
            ExpectedParams { velocity },
        )
    }

    #[test]
    fn does_not_increase_velocity_for_once_acceleration_when_no_event() -> Result<(), Error> {
        let object_acceleration = ObjectAcceleration {
            kind: ObjectAccelerationKind::Once,
            x: ObjectAccelerationValue::Const(1.),
            y: ObjectAccelerationValue::Const(2.),
            z: ObjectAccelerationValue::Const(3.),
        };
        let velocity = Velocity::new(10., 20., 30.);

        run_test(
            SetupParams {
                controller_input: None,
                mirrored: None,
                object_acceleration,
                velocity,
                sequence_update_event_fn: None,
            },
            ExpectedParams { velocity },
        )
    }

    #[test]
    fn does_not_negate_velocity_when_not_mirrored() -> Result<(), Error> {
        let object_acceleration = ObjectAcceleration {
            kind: ObjectAccelerationKind::Continuous,
            x: ObjectAccelerationValue::Const(1.),
            y: ObjectAccelerationValue::Const(2.),
            z: ObjectAccelerationValue::Const(3.),
        };
        let velocity = Velocity::new(10., 20., 30.);

        run_test(
            SetupParams {
                controller_input: None,
                mirrored: Some(Mirrored::new(false)),
                object_acceleration,
                velocity,
                sequence_update_event_fn: None,
            },
            ExpectedParams {
                velocity: Velocity::new(11., 22., 33.),
            },
        )
    }

    #[test]
    fn negates_x_velocity_when_mirrored() -> Result<(), Error> {
        let object_acceleration = ObjectAcceleration {
            kind: ObjectAccelerationKind::Continuous,
            x: ObjectAccelerationValue::Const(1.),
            y: ObjectAccelerationValue::Const(2.),
            z: ObjectAccelerationValue::Const(3.),
        };
        let velocity = Velocity::new(10., 20., 30.);

        run_test(
            SetupParams {
                controller_input: None,
                mirrored: Some(Mirrored::new(true)),
                object_acceleration,
                velocity,
                sequence_update_event_fn: None,
            },
            ExpectedParams {
                velocity: Velocity::new(9., 22., 33.),
            },
        )
    }

    #[test]
    fn does_not_increase_velocity_for_x_expr_value_when_x_controller_input_zero(
    ) -> Result<(), Error> {
        let object_acceleration = ObjectAcceleration {
            kind: ObjectAccelerationKind::Continuous,
            x: ObjectAccelerationValue::Expr(ObjectAccelerationValueExpr {
                multiplier: ObjectAccelerationValueMultiplier::XAxis,
                value: 0.,
            }),
            y: ObjectAccelerationValue::Const(0.),
            z: ObjectAccelerationValue::Const(0.),
        };
        let velocity = Velocity::new(10., 0., 0.);
        let controller_input = ControllerInput {
            x_axis_value: 0.,
            ..Default::default()
        };

        run_test(
            SetupParams {
                controller_input: Some(controller_input),
                mirrored: Some(Mirrored::new(false)),
                object_acceleration,
                velocity,
                sequence_update_event_fn: None,
            },
            ExpectedParams {
                velocity: Velocity::new(10., 0., 0.),
            },
        )
    }

    #[test]
    fn increases_velocity_for_x_expr_value_when_x_controller_input_valued() -> Result<(), Error> {
        let object_acceleration = ObjectAcceleration {
            kind: ObjectAccelerationKind::Continuous,
            x: ObjectAccelerationValue::Expr(ObjectAccelerationValueExpr {
                multiplier: ObjectAccelerationValueMultiplier::XAxis,
                value: 0.5,
            }),
            y: ObjectAccelerationValue::Const(0.),
            z: ObjectAccelerationValue::Const(0.),
        };
        let velocity = Velocity::new(10., 0., 0.);
        let controller_input = ControllerInput {
            x_axis_value: 2.,
            ..Default::default()
        };

        run_test(
            SetupParams {
                controller_input: Some(controller_input),
                mirrored: Some(Mirrored::new(false)),
                object_acceleration,
                velocity,
                sequence_update_event_fn: None,
            },
            ExpectedParams {
                velocity: Velocity::new(11., 0., 0.),
            },
        )
    }

    #[test]
    fn negates_velocity_for_x_expr_value_when_x_controller_input_positive_and_mirrored(
    ) -> Result<(), Error> {
        let object_acceleration = ObjectAcceleration {
            kind: ObjectAccelerationKind::Continuous,
            x: ObjectAccelerationValue::Expr(ObjectAccelerationValueExpr {
                multiplier: ObjectAccelerationValueMultiplier::XAxis,
                value: 0.5,
            }),
            y: ObjectAccelerationValue::Const(0.),
            z: ObjectAccelerationValue::Const(0.),
        };
        let velocity = Velocity::new(10., 0., 0.);
        let controller_input = ControllerInput {
            x_axis_value: 2.,
            ..Default::default()
        };

        run_test(
            SetupParams {
                controller_input: Some(controller_input),
                mirrored: Some(Mirrored::new(true)),
                object_acceleration,
                velocity,
                sequence_update_event_fn: None,
            },
            ExpectedParams {
                velocity: Velocity::new(9., 0., 0.),
            },
        )
    }

    #[test]
    fn negates_velocity_once_for_x_expr_value_when_x_controller_input_negative_and_mirrored(
    ) -> Result<(), Error> {
        let object_acceleration = ObjectAcceleration {
            kind: ObjectAccelerationKind::Continuous,
            x: ObjectAccelerationValue::Expr(ObjectAccelerationValueExpr {
                multiplier: ObjectAccelerationValueMultiplier::XAxis,
                value: -0.5,
            }),
            y: ObjectAccelerationValue::Const(0.),
            z: ObjectAccelerationValue::Const(0.),
        };
        let velocity = Velocity::new(10., 0., 0.);
        let controller_input = ControllerInput {
            x_axis_value: 2.,
            ..Default::default()
        };

        run_test(
            SetupParams {
                controller_input: Some(controller_input),
                mirrored: Some(Mirrored::new(true)),
                object_acceleration,
                velocity,
                sequence_update_event_fn: None,
            },
            ExpectedParams {
                velocity: Velocity::new(11., 0., 0.),
            },
        )
    }

    #[test]
    fn negates_velocity_for_x_expr_value_when_x_controller_input_valued_and_mirrored_and_once(
    ) -> Result<(), Error> {
        let object_acceleration = ObjectAcceleration {
            kind: ObjectAccelerationKind::Once,
            x: ObjectAccelerationValue::Expr(ObjectAccelerationValueExpr {
                multiplier: ObjectAccelerationValueMultiplier::XAxis,
                value: 0.5,
            }),
            y: ObjectAccelerationValue::Const(0.),
            z: ObjectAccelerationValue::Const(0.),
        };
        let velocity = Velocity::new(10., 0., 0.);
        let controller_input = ControllerInput {
            x_axis_value: 2.,
            ..Default::default()
        };

        run_test(
            SetupParams {
                controller_input: Some(controller_input),
                mirrored: Some(Mirrored::new(true)),
                object_acceleration,
                velocity,
                sequence_update_event_fn: Some(frame_begin_event),
            },
            ExpectedParams {
                velocity: Velocity::new(9., 0., 0.),
            },
        )
    }

    #[test]
    fn does_not_increase_velocity_for_x_expr_value_when_x_controller_input_valued_and_no_event(
    ) -> Result<(), Error> {
        let object_acceleration = ObjectAcceleration {
            kind: ObjectAccelerationKind::Once,
            x: ObjectAccelerationValue::Expr(ObjectAccelerationValueExpr {
                multiplier: ObjectAccelerationValueMultiplier::XAxis,
                value: 0.5,
            }),
            y: ObjectAccelerationValue::Const(0.),
            z: ObjectAccelerationValue::Const(0.),
        };
        let velocity = Velocity::new(10., 0., 0.);
        let controller_input = ControllerInput {
            x_axis_value: 2.,
            ..Default::default()
        };

        run_test(
            SetupParams {
                controller_input: Some(controller_input),
                mirrored: Some(Mirrored::new(true)),
                object_acceleration,
                velocity,
                sequence_update_event_fn: None,
            },
            ExpectedParams {
                velocity: Velocity::new(10., 0., 0.),
            },
        )
    }

    #[test]
    fn does_not_increase_velocity_for_z_expr_value_when_z_controller_input_zero(
    ) -> Result<(), Error> {
        let object_acceleration = ObjectAcceleration {
            kind: ObjectAccelerationKind::Continuous,
            x: ObjectAccelerationValue::Const(0.),
            y: ObjectAccelerationValue::Const(0.),
            z: ObjectAccelerationValue::Expr(ObjectAccelerationValueExpr {
                multiplier: ObjectAccelerationValueMultiplier::ZAxis,
                value: 0.,
            }),
        };
        let velocity = Velocity::new(0., 0., 10.);
        let controller_input = ControllerInput {
            z_axis_value: 0.,
            ..Default::default()
        };

        run_test(
            SetupParams {
                controller_input: Some(controller_input),
                mirrored: Some(Mirrored::new(false)),
                object_acceleration,
                velocity,
                sequence_update_event_fn: None,
            },
            ExpectedParams {
                velocity: Velocity::new(0., 0., 10.),
            },
        )
    }

    #[test]
    fn increases_velocity_for_z_expr_value_when_z_controller_input_valued() -> Result<(), Error> {
        let object_acceleration = ObjectAcceleration {
            kind: ObjectAccelerationKind::Continuous,
            x: ObjectAccelerationValue::Const(0.),
            y: ObjectAccelerationValue::Const(0.),
            z: ObjectAccelerationValue::Expr(ObjectAccelerationValueExpr {
                multiplier: ObjectAccelerationValueMultiplier::ZAxis,
                value: 0.5,
            }),
        };
        let velocity = Velocity::new(0., 0., 10.);
        let controller_input = ControllerInput {
            z_axis_value: 2.,
            ..Default::default()
        };

        run_test(
            SetupParams {
                controller_input: Some(controller_input),
                mirrored: Some(Mirrored::new(false)),
                object_acceleration,
                velocity,
                sequence_update_event_fn: None,
            },
            ExpectedParams {
                velocity: Velocity::new(0., 0., 11.),
            },
        )
    }

    fn run_test(
        SetupParams {
            controller_input,
            mirrored,
            object_acceleration,
            velocity: velocity_setup,
            sequence_update_event_fn,
        }: SetupParams,
        ExpectedParams {
            velocity: velocity_expected,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(
                ObjectAccelerationSystem::new(),
                ObjectAccelerationSystem::type_name(),
                &[],
            )
            .with_setup(move |world| {
                let entity = {
                    let mut entity_builder = world
                        .create_entity()
                        .with(object_acceleration)
                        .with(velocity_setup);

                    if let Some(controller_input) = controller_input {
                        entity_builder = entity_builder.with(controller_input);
                    }
                    if let Some(mirrored) = mirrored {
                        entity_builder = entity_builder.with(mirrored);
                    }

                    entity_builder.build()
                };

                if let Some(sequence_update_event_fn) = sequence_update_event_fn {
                    let sequence_update_event = sequence_update_event_fn(entity);
                    world
                        .write_resource::<EventChannel<SequenceUpdateEvent>>()
                        .single_write(sequence_update_event);
                }

                world.insert(entity);
            })
            .with_assertion(move |world| {
                let entity = *world.read_resource::<Entity>();
                let velocities = world.read_storage::<Velocity<f32>>();
                let velocity_actual = velocities
                    .get(entity)
                    .copied()
                    .expect("Expected entity to have `Velocity<f32>` component.");

                assert_eq!(velocity_expected, velocity_actual);
            })
            .run()
    }

    fn sequence_begin_event(entity: Entity) -> SequenceUpdateEvent {
        SequenceUpdateEvent::SequenceBegin {
            entity,
            sequence_id: SequenceId::new(0),
        }
    }

    fn frame_begin_event(entity: Entity) -> SequenceUpdateEvent {
        SequenceUpdateEvent::FrameBegin {
            entity,
            frame_index: 0,
        }
    }

    fn sequence_end_event(entity: Entity) -> SequenceUpdateEvent {
        SequenceUpdateEvent::SequenceEnd {
            entity,
            frame_index: 0,
        }
    }

    struct SetupParams {
        controller_input: Option<ControllerInput>,
        mirrored: Option<Mirrored>,
        object_acceleration: ObjectAcceleration,
        velocity: Velocity<f32>,
        sequence_update_event_fn: Option<fn(Entity) -> SequenceUpdateEvent>,
    }

    struct ExpectedParams {
        velocity: Velocity<f32>,
    }
}
