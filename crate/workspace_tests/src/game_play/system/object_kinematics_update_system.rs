#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entity, SystemData, WorldExt},
        input::StringBindings,
        Error,
    };
    use amethyst_test::AmethystApplication;
    use kinematic_model::config::{Position, Velocity};
    use sequence_model::play::FrameFreezeClock;
    use std::any;

    use game_play::{ObjectKinematicsUpdateSystem, ObjectKinematicsUpdateSystemData};

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
            .with_effect(move |world| {
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
                any::type_name::<ObjectKinematicsUpdateSystem>(),
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
