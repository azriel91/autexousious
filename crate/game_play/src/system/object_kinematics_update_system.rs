use amethyst::ecs::WorldExt; use amethyst::{
    ecs::{Join, ReadStorage, WriteStorage},
    shred::System,
};
use derive_new::new;
use kinematic_model::config::{Position, Velocity};
use sequence_model::play::FrameFreezeClock;
use typename_derive::TypeName;

/// Updates each entity's `Position` based on their `Velocity` in game.
///
/// This system should be run after all other systems that affect kinematics have run.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct ObjectKinematicsUpdateSystem;

type ObjectKinematicsUpdateSystemData<'s> = (
    ReadStorage<'s, FrameFreezeClock>,
    WriteStorage<'s, Position<f32>>,
    ReadStorage<'s, Velocity<f32>>,
);

impl<'s> System<'s> for ObjectKinematicsUpdateSystem {
    type SystemData = ObjectKinematicsUpdateSystemData<'s>;

    fn run(&mut self, (frame_freeze_clocks, mut positions, velocities): Self::SystemData) {
        (frame_freeze_clocks.maybe(), &mut positions, &velocities)
            .join()
            .for_each(|(frame_freeze_clock, position, velocity)| {
                let frozen = frame_freeze_clock
                    .map(|frame_freeze_clock| !frame_freeze_clock.is_complete())
                    .unwrap_or(false);
                if !frozen {
                    position.0 += velocity.0;
                }
            })
    }
}

#[cfg(test)]
mod test {
    use amethyst::ecs::WorldExt; use amethyst::{
        ecs::{Builder, Entity, SystemData},
        input::StringBindings,
        Error,
    };
    use amethyst_test::AmethystApplication;
    use kinematic_model::config::{Position, Velocity};
    use sequence_model::play::FrameFreezeClock;
    use typename::TypeName;

    use super::{ObjectKinematicsUpdateSystem, ObjectKinematicsUpdateSystemData};

    #[test]
    fn adds_velocity_to_position() -> Result<(), Error> {
        let mut frame_freeze_clock = FrameFreezeClock::new(1);
        frame_freeze_clock.tick();

        run_test(
            SetupParams {
                position: Position::<f32>::new(-2., -2., -2.),
                velocity: Velocity::<f32>::new(-3., -3., -3.),
                frame_freeze_clock: None,
            },
            Position::<f32>::new(-5., -5., -5.),
        )
    }

    #[test]
    fn does_not_add_velocity_to_position_when_frame_freeze_clock_incomplete() -> Result<(), Error> {
        run_test(
            SetupParams {
                position: Position::<f32>::new(-2., -2., -2.),
                velocity: Velocity::<f32>::new(-3., -3., -3.),
                frame_freeze_clock: Some(FrameFreezeClock::new(1)),
            },
            Position::<f32>::new(-2., -2., -2.),
        )
    }

    #[test]
    fn adds_velocity_to_position_when_frame_freeze_clock_complete() -> Result<(), Error> {
        let mut frame_freeze_clock = FrameFreezeClock::new(1);
        frame_freeze_clock.tick();

        run_test(
            SetupParams {
                position: Position::<f32>::new(-2., -2., -2.),
                velocity: Velocity::<f32>::new(-3., -3., -3.),
                frame_freeze_clock: Some(frame_freeze_clock),
            },
            Position::<f32>::new(-5., -5., -5.),
        )
    }

    fn run_test(
        SetupParams {
            position,
            velocity,
            frame_freeze_clock,
        }: SetupParams,
        expected_position: Position<f32>,
    ) -> Result<(), Error> {
        AmethystApplication::ui_base::<StringBindings>()
            .with_setup(move |world| {
                ObjectKinematicsUpdateSystemData::setup(world);

                let mut entity_builder = world.create_entity().with(position).with(velocity);
                if let Some(frame_freeze_clock) = frame_freeze_clock {
                    entity_builder = entity_builder.with(frame_freeze_clock);
                }
                let entity = entity_builder.build();

                world.insert(entity);
            })
            .with_system_single(
                ObjectKinematicsUpdateSystem::new(),
                ObjectKinematicsUpdateSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_assertion(move |world| {
                let entity = *world.read_resource::<Entity>();
                let positions = world.read_storage::<Position<f32>>();

                assert_eq!(Some(&expected_position), positions.get(entity));
            })
            .run()
    }

    struct SetupParams {
        position: Position<f32>,
        velocity: Velocity<f32>,
        frame_freeze_clock: Option<FrameFreezeClock>,
    }
}
