#[cfg(test)]
mod tests {
    use std::any;

    use amethyst::{
        ecs::{Entity, WorldExt, WriteStorage},
        window::ScreenDimensions,
        Error,
    };
    use amethyst_test::{AmethystApplication, HIDPI, SCREEN_HEIGHT, SCREEN_WIDTH};
    use camera_model::play::CameraTargetCoordinates;
    use kinematic_model::config::{Position, Velocity};
    use pretty_assertions::assert_eq;

    use camera_play::{CameraCreator, CameraVelocitySystem};

    #[test]
    fn velocity_remains_zero_when_position_matches_target() -> Result<(), Error> {
        run_test(
            SetupParams {
                camera_target_coordinates: CameraTargetCoordinates::new(100., 200., 300.),
                position: Position::new(100., 200., 300.),
                velocity: Velocity::new(0., 0., 0.),
            },
            ExpectedParams {
                velocity_steps: vec![Velocity::new(0., 0., 0.), Velocity::new(0., 0., 0.)],
            },
        )
    }

    #[test]
    fn velocity_increments_smoothly_when_target_is_far() -> Result<(), Error> {
        run_test(
            SetupParams {
                camera_target_coordinates: CameraTargetCoordinates::new(100., 200., 300.),
                position: Position::new(0., 0., 0.),
                velocity: Velocity::new(0., 0., 0.),
            },
            ExpectedParams {
                velocity_steps: vec![
                    Velocity::new(5., 10., 15.),
                    Velocity::new(7.5, 15., 22.5),
                    Velocity::new(8.75, 17.5, 26.25),
                ],
            },
        )
    }

    fn run_test(
        SetupParams {
            camera_target_coordinates,
            position,
            velocity: velocity_setup,
        }: SetupParams,
        ExpectedParams { velocity_steps }: ExpectedParams,
    ) -> Result<(), Error> {
        let mut amethyst_application = AmethystApplication::blank()
            .with_resource(ScreenDimensions::new(SCREEN_WIDTH, SCREEN_HEIGHT, HIDPI))
            .with_system(
                CameraVelocitySystem {
                    smoothing_factor: 2.,
                },
                any::type_name::<CameraVelocitySystem>(),
                &[],
            ) // kcov-ignore
            .with_effect(move |world| {
                let camera_entity = CameraCreator::create_in_world(world);

                {
                    let (mut camera_target_coordinateses, mut positions, mut velocities) = world
                        .system_data::<(
                            WriteStorage<'_, CameraTargetCoordinates>,
                            WriteStorage<'_, Position<f32>>,
                            WriteStorage<'_, Velocity<f32>>,
                        )>();

                    camera_target_coordinateses
                        .insert(camera_entity, camera_target_coordinates)
                        .expect("Failed to insert `CameraTargetCoordinates` component.");
                    positions
                        .insert(camera_entity, position)
                        .expect("Failed to insert `Position<f32>` component.");
                    velocities
                        .insert(camera_entity, velocity_setup)
                        .expect("Failed to insert `Velocity<f32>` component.");
                }

                world.insert(camera_entity);
            });

        amethyst_application = velocity_steps.into_iter().fold(
            amethyst_application,
            |amethyst_application, velocity_expected| {
                amethyst_application.with_assertion(move |world| {
                    let entity = *world.read_resource::<Entity>();
                    let velocities = world.read_storage::<Velocity<f32>>();
                    let velocity_actual = velocities
                        .get(entity)
                        .copied()
                        .expect("Expected entity to have `Transform` component.");

                    assert_eq!(velocity_expected, velocity_actual);
                })
            },
        );

        amethyst_application.run()
    }

    struct SetupParams {
        camera_target_coordinates: CameraTargetCoordinates,
        position: Position<f32>,
        velocity: Velocity<f32>,
    }

    struct ExpectedParams {
        velocity_steps: Vec<Velocity<f32>>,
    }
}
