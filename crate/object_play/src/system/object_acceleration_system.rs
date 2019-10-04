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
